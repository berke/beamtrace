use std::ops::{Add,AddAssign,Sub,SubAssign,Mul};
use std::path::Path;
use std::error::Error;
use std::fs::File;
use std::io::{Read,Write,BufWriter,BufReader};
use serde::{Serialize,Deserialize};

#[derive(Debug,Copy,Clone,Serialize,Deserialize)]
pub struct Point {
    pub x:f64,
    pub y:f64
}

#[derive(Debug,Copy,Clone,Serialize,Deserialize)]
pub struct Rectangle {
    pub a:Point,
    pub b:Point
}

#[derive(Copy,Clone,Debug)]
pub struct Homography {
    coefs:[[f64;3];3]
}

impl Homography {
    pub fn id()->Self {
	Self{ coefs:[[1.0,0.0,0.0],
		     [0.0,1.0,0.0],
		     [0.0,0.0,1.0]] }
    }

    pub fn rotation(theta:f64)->Self {
	Self{ coefs:[[ theta.cos(),theta.sin(),0.0],
		     [-theta.sin(),theta.cos(),0.0],
		     [         0.0,        0.0,1.0]] }
    }

    pub fn rotate(&mut self,theta:f64) {
	self.compose(Self::rotation(theta))
    }

    pub fn scale(&mut self,s:f64) {
	self.coefs[2][2] /= s;
    }

    pub fn translate(&mut self,p:Point) {
	self.coefs[0][2] += p.x;
	self.coefs[1][2] += p.y;
    }

    pub fn apply(&self,p:Point)->Point {
	let [[a,b,c],[d,e,f],[g,h,i]] = self.coefs;
	let Point{ x,y } = p;
	let z = 1.0;
	let xp = a*x+b*y+c*z;
	let yp = d*x+e*y+f*z;
	let zp = g*x+h*y+i*z;
	point(xp/zp,yp/zp)
    }

    pub fn compose(&mut self,other:Self) {
	let [[a1,b1,c1],[d1,e1,f1],[g1,h1,i1]] = self.coefs;
	let [[a2,b2,c2],[d2,e2,f2],[g2,h2,i2]] = other.coefs;
	self.coefs = [[a1*a2+b1*d2+c1*g2,
		       a1*b2+b1*e2+c1*h2,
		       a1*c2+b1*f2+c1*i2],
		      [d1*a2+e1*d2+f1*g2,
		       d1*b2+e1*e2+f1*h2,
		       d1*c2+e1*f2+f1*i2],
		      [g1*a2+h1*d2+i1*g2,
		       g1*b2+h1*e2+i1*h2,
		       g1*c2+h1*f2+i1*i2]];
    }
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub enum Command {
    Color([f64;3]),
    Points(Vec<Point>),
    Lines(Vec<Point>),
    Translate(Point),
    Scale(f64),
    Rotate(f64),
    Seq(Vec<Command>),
    Group(Plot)
}

impl From<(f64,f64)> for Point {
    fn from((x,y):(f64,f64))->Self {
	Point{ x, y }
    }
}

pub fn point(x:f64,y:f64)->Point {
    Point{ x, y }
}

pub fn rectangle(a:Point,b:Point)->Rectangle {
    Rectangle{ a, b }
}

impl Rectangle {
    pub fn add_point(&mut self,p:Point) {
	self.a = self.a.min(p);
	self.b = self.b.max(p);
    }

    pub fn add_rectangle(&mut self,r:Rectangle) {
	self.add_point(r.a);
	self.add_point(r.b);
    }
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Plot {
    pub commands:Vec<Command>
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Page {
    pub plots:Vec<Plot>
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Book {
    pub pages:Vec<Page>
}

impl Point {
    pub fn with_x(&self,other:Self)->Self {
	point(other.x,self.y)
    }

    pub fn with_y(&self,other:Self)->Self {
	point(self.x,other.y)
    }

    pub fn max(self,other:Self)->Self {
	point(self.x.max(other.x),self.y.max(other.y))
    }

    pub fn min(self,other:Self)->Self {
	point(self.x.min(other.x),self.y.min(other.y))
    }

    pub fn as_pair(&self)->(f64,f64) {
	(self.x,self.y)
    }
}

impl SubAssign<Point> for Point {
    fn sub_assign(&mut self,other:Self) {
	self.x -= other.x;
	self.y -= other.y;
    }
}

impl Sub<Point> for Point {
    type Output = Point;

    fn sub(self,other:Self)->Self {
	point(self.x - other.x,self.y - other.y)
    }
}

impl AddAssign<Point> for Point {
    fn add_assign(&mut self,other:Self) {
	self.x += other.x;
	self.y += other.y;
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self,other:Self)->Self {
	point(self.x + other.x,self.y + other.y)
    }
}

impl Mul<Point> for f64 {
    type Output = Point;

    fn mul(self,other:Point)->Point {
	point(self * other.x,self * other.y)
    }
}

impl Add<(f64,f64)> for Point {
    type Output = Point;

    fn add(self,(x,y):(f64,f64))->Self {
	point(self.x + x,self.y + y)
    }
}

impl AddAssign<(f64,f64)> for Point {
    fn add_assign(&mut self,other:(f64,f64)) {
	self.x += other.0;
	self.y += other.1;
    }
}

impl Sub<(f64,f64)> for Point {
    type Output = Point;

    fn sub(self,(x,y):(f64,f64))->Self {
	point(self.x - x,self.y - y)
    }
}

impl SubAssign<(f64,f64)> for Point {
    fn sub_assign(&mut self,other:(f64,f64)) {
	self.x -= other.0;
	self.y -= other.1;
    }
}

impl Page {
    pub fn new()->Self {
	Self{ plots:Vec::new() }
    }

    pub fn len(&self)->usize {
	self.plots.len()
    }

    pub fn plot(&mut self,pl:Plot) {
	self.plots.push(pl);
    }

    pub fn get_plot(&self,k:isize)->Option<&Plot> {
	let m = self.len();
	if m == 0 {
	    None
	} else {
	    let i = k.rem_euclid(m as isize) as usize;
	    Some(&self.plots[i])
	}
    }
}

impl Plot {
    pub fn new()->Self {
	Self{ commands:Vec::new() }
    }

    pub fn rgb12(&mut self,rgb:u32) {
	self.color(
	    (rgb >> 8) as f64 / 15.0,
	    ((rgb >> 4) & 15) as f64 / 15.0,
	    (rgb & 15) as f64 / 15.0);
    }

    pub fn color(&mut self,r:f64,g:f64,b:f64) {
	self.command(Command::Color([r,g,b]));
    }

    pub fn rotate(&mut self,theta:f64) {
	self.command(Command::Rotate(theta));
    }

    pub fn translate(&mut self,p:Point) {
	self.command(Command::Translate(p));
    }

    pub fn scale(&mut self,s:f64) {
	self.command(Command::Scale(s));
    }

    pub fn point(&mut self,p:Point) {
	self.command(Command::Points(vec![p]));
    }

    pub fn line(&mut self,p1:Point,p2:Point) {
	self.command(Command::Lines(vec![p1,p2]));
    }

    pub fn lines(&mut self,lines:Vec<Point>) {
	self.command(Command::Lines(lines));
    }

    pub fn command(&mut self,cmd:Command) {
	self.commands.push(cmd);
    }

    pub fn group(&mut self,pl:Plot) {
	self.commands.push(Command::Group(pl));
    }
}

impl Book {
    pub fn new()->Self {
	Self{ pages:Vec::new() }
    }

    pub fn write<W:Write>(&self,w:&mut W)->Result<(),Box<dyn Error>> {
	self.serialize(&mut rmp_serde::Serializer::new(w))?;
	Ok(())
    }

    pub fn read<R:Read>(r:&mut R)->Result<Self,Box<dyn Error>> {
	let this : Self = rmp_serde::decode::from_read(r)?;
	Ok(this)
    }

    pub fn save_to_file<P:AsRef<Path>>(&self,path:P)->Result<(),Box<dyn Error>> {
	let fd = File::create(path)?;
	let mut buf = BufWriter::new(fd);
	self.write(&mut buf)
    }

    pub fn load_from_file<P:AsRef<Path>>(path:P)->Result<Self,Box<dyn Error>> {
	let fd = File::open(path)?;
	let mut buf = BufReader::new(fd);
	Self::read(&mut buf)
    }

    pub fn get_page(&self,k:isize)->Option<&Page> {
	let m = self.pages.len();
	if m == 0 {
	    None
	} else {
	    let i = k.rem_euclid(m as isize) as usize;
	    Some(&self.pages[i])
	}
    }

    pub fn page(&mut self,page:Page) {
	self.pages.push(page);
    }
}
