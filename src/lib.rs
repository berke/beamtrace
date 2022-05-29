pub mod geometry;

use std::path::Path;
use std::error::Error;
use std::fs::File;
use std::io::{Read,Write,BufWriter,BufReader};
use serde::{Serialize,Deserialize};

pub use crate::geometry::{point,rectangle,Point,Rectangle};

pub type Color = u16;

#[derive(Debug,Clone,Serialize,Deserialize)]
pub enum Command {
    Points{ color:Color,points:Vec<Point> },
    Lines{ color:Color,lines:Vec<Vec<Point>> }
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

impl Command {
    pub fn line(color:Color,p1:Point,p2:Point)->Command {
	Self::Lines{ color,lines:vec![vec![p1,p2]] }
    }

    pub fn rectangle(color:Color,Rectangle{ a:p0,b:p1 }:Rectangle)->Command {
	Self::Lines{ color, lines:vec![vec![p0,p0.with_x(p1),p1,p1.with_x(p0),p0]] }
    }
}

impl Page {
    pub fn new()->Self {
	Self{ plots:Vec::new() }
    }

    pub fn len(&self)->usize {
	self.plots.len()
    }

    pub fn is_empty(&self)->bool {
	self.plots.is_empty()
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

impl Default for Page {
    fn default()->Self {
	Self::new()
    }
}

impl Plot {
    pub fn new()->Self {
	Self{ commands:Vec::new() }
    }

    pub fn point(&mut self,color:Color,p:Point) {
	self.command(Command::Points{ color,points:vec![p] });
    }

    pub fn line(&mut self,color:Color,p1:Point,p2:Point) {
	self.command(Command::Lines{ color,lines:vec![vec![p1,p2]] });
    }

    pub fn lines(&mut self,color:Color,lines:Vec<Vec<Point>>) {
	self.command(Command::Lines{ color,lines });
    }

    pub fn rectangle(&mut self,color:Color,Rectangle{ a:p0,b:p1 }:Rectangle) {
	self.lines(color,vec![vec![p0,p0.with_x(p1),p1,p1.with_x(p0),p0]]);
    }

    pub fn command(&mut self,cmd:Command) {
	self.commands.push(cmd);
    }

    pub fn bounding(&self)->Rectangle {
	let mut r = Rectangle::empty();
	for cmd in self.commands.iter() {
	    match cmd {
		Command::Points{ points, .. } => {
		    for &p in points.iter() {
			r.add_point(p);
		    }
		},
		Command::Lines{ lines, .. } => {
		    for line in lines.iter() {
			for &p in line.iter() {
			    r.add_point(p);
			}
		    }
		}
	    }
	}
	r
    }
}

impl Default for Plot {
    fn default()->Self {
	Self::new()
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

impl Default for Book {
    fn default()->Self {
	Self::new()
    }
}
