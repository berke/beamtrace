use std::ops::{Add,AddAssign,Mul};
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

pub fn point(x:f64,y:f64)->Point {
    Point{ x, y }
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub enum Command {
    Color([f64;3]),
    Points(Vec<Point>),
    Lines(Vec<Point>),
    Text(Point,f64,String),
    Seq(Vec<Command>)
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

pub trait Maxable {
    fn max(self,other:Self)->Self;
}

impl Point {
    pub fn with_x(&self,other:Self)->Self {
	point(other.x,self.y)
    }

    pub fn with_y(&self,other:Self)->Self {
	point(self.x,other.y)
    }
}

impl Maxable for Point {
    fn max(self,other:Self)->Self {
	point(self.x.max(other.x),self.y.max(other.y))
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

    pub fn line(&mut self,x1:f64,y1:f64,x2:f64,y2:f64) {
	self.command(Command::Lines(vec![point(x1,y1),point(x2,y2)]));
    }

    pub fn lines(&mut self,lines:Vec<Point>) {
	self.command(Command::Lines(lines));
    }

    pub fn text(&mut self,p:Point,s:f64,txt:&str) {
	self.command(Command::Text(p,s,txt.to_string()));
    }

    pub fn command(&mut self,cmd:Command) {
	self.commands.push(cmd);
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
