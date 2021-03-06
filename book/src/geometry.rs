use std::f64::{NEG_INFINITY,INFINITY};
use std::ops::{Add,AddAssign,Sub,SubAssign,Mul,Div};
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

impl From<(f64,f64)> for Point {
    fn from((x,y):(f64,f64))->Self {
	Point{ x, y }
    }
}

pub const ORIGIN : Point = Point{ x:0.0,y:0.0 };

pub fn point(x:f64,y:f64)->Point {
    Point{ x, y }
}

pub fn rectangle(p1:Point,p2:Point)->Rectangle {
    Rectangle{ a:p1.min(p2), b:p1.max(p2) }
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

    pub fn contains(&self,p:Point)->bool {
	self.a.x <= p.x && p.x <= self.b.x &&
	    self.a.y <= p.y && p.y <= self.b.y
    }

    pub fn dx(&self)->f64 {
	self.b.x - self.a.x
    }

    pub fn dy(&self)->f64 {
	self.b.y - self.a.y
    }

    pub fn all()->Self {
	Rectangle{
	    a:point(NEG_INFINITY,NEG_INFINITY),
	    b:point(INFINITY,INFINITY)
	}
    }

    pub fn empty()->Self {
	Rectangle{
	    a:point(INFINITY,INFINITY),
	    b:point(NEG_INFINITY,NEG_INFINITY)
	}
    }

    pub fn bounding(pts:&[Point])->Rectangle {
	pts.iter().fold(Self::all(),|mut r,&p| { r.add_point(p); r })
    }
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

    pub fn norm(self)->f64 {
	self.x.hypot(self.y)
    }

    pub fn dot(self,other:Self)->f64 {
	self.x*other.x + self.y*other.y
    }

    pub fn ortho(self)->Self {
	Self{ x:self.y,y:-self.x }
    }

    pub fn normalize(self)->Self {
	let n = self.norm();
	point(self.x/n,self.y/n)
    }

    pub fn equivalent(&self,other:Point)->bool {
	(self.x - other.x).abs() <= 0.0 &&
	(self.y - other.y).abs() <= 0.0
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

impl Div<f64> for Point {
    type Output = Point;

    fn div(self,other:f64)->Point {
	point(self.x/other,self.y/other)
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
