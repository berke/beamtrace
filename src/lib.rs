use serde::{Serialize,Deserialize};
use std::path::Path;
use std::error::Error;
use std::fs::File;
use std::io::{Read,Write,BufWriter,BufReader};

#[derive(Debug,Clone,Serialize,Deserialize)]
pub enum Command {
    Color([f64;3]),
    Points(Vec<(f64,f64)>),
    Lines(Vec<(f64,f64)>),
    Text((f64,f64),f64,String),
    Seq(Vec<Command>)
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Frame {
    pub commands:Vec<Command>
}

impl Frame {
    pub fn new()->Self {
	Self{ commands:Vec::new() }
    }

    pub fn color(&mut self,r:f64,g:f64,b:f64) {
	self.commands.push(Command::Color([r,g,b]));
    }

    pub fn line(&mut self,x1:f64,y1:f64,x2:f64,y2:f64) {
	self.commands.push(Command::Lines(vec![(x1,y1),(x2,y2)]));
    }

    pub fn text(&mut self,p:(f64,f64),s:f64,txt:&str) {
	self.commands.push(Command::Text(p,s,txt.to_string()));
    }
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Request {
    pub frames:Vec<Frame>
}

impl Request {
    pub fn new()->Self {
	Self{ frames:Vec::new() }
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

    pub fn get_frame(&self,k:isize)->Option<&Frame> {
	let m = self.frames.len();
	if m == 0 {
	    None
	} else {
	    let i = k.rem_euclid(m as isize) as usize;
	    Some(&self.frames[i])
	}
    }

    pub fn push_frame(&mut self,frame:Frame) {
	self.frames.push(frame);
    }
}
