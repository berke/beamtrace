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

#[derive(Debug,Clone)]
pub struct Request {
    pub commands:Vec<Command>
}

impl From<Command> for Request {
    fn from(command:Command)->Self {
	Self{ commands:vec![command] }
    }
}

impl Request {
    pub fn new()->Self {
	Self{ commands:Vec::new() }
    }

    pub fn push(&mut self,cmd:Command) {
	self.commands.push(cmd);
    }

    pub fn clear(&mut self) {
	self.commands.clear();
    }

    pub fn write<W:Write>(&self,w:&mut W)->Result<(),Box<dyn Error>> {
	self.commands.serialize(&mut rmp_serde::Serializer::new(w))?;
	Ok(())
    }

    pub fn read<R:Read>(r:&mut R)->Result<Self,Box<dyn Error>> {
	let commands : Vec<Command> = rmp_serde::decode::from_read(r)?;
	Ok(Self{ commands })
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

    pub fn get(self)->Vec<Command> {
	self.commands
    }
}
