use std::f64::{NEG_INFINITY,INFINITY};
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

impl From<(f64,f64)> for Point {
    fn from((x,y):(f64,f64))->Self {
	Point{ x, y }
    }
}

pub const ORIGIN : Point = Point{ x:0.0,y:0.0 };

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

    pub fn all()->Self {
	Rectangle{
	    a:point(INFINITY,INFINITY),
	    b:point(NEG_INFINITY,NEG_INFINITY)
	}
    }

    pub fn bounding(pts:&[Point])->Rectangle {
	pts.iter().fold(Self::all(),|r,&p| { let mut r = r.clone(); r.add_point(p); r })
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
