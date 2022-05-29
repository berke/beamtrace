use beamtrace::{point,rectangle,Point,Rectangle,Color,Plot,Command,geometry::ORIGIN};
use std::rc::Rc;
use crate::font::{Font,W,H};
use crate::homography::Homography;

#[derive(Clone,Debug)]
pub struct Object {
    pub area:Rectangle,
    pub contents:Vec<Content>
}

#[derive(Clone,Debug)]
pub enum Content {
    Transform{
	transform:Homography,
	object:Rc<Object>
    },
    Draw(Command),
    Object(Rc<Object>)
}

#[derive(Clone,Debug)]
pub enum Text {
    Char(char),
    Sup(Box<Text>),
    Sub(Box<Text>),
    Seq(Vec<Text>)
}

impl Object {
    pub fn empty()->Self {
	Self{
	    area:Rectangle::empty(),
	    contents:Vec::new()
	}
    }

    pub fn add_object(&mut self,obj:&Rc<Self>) {
	self.area.add_rectangle(obj.area);
	self.contents.push(Content::Object(Rc::clone(obj)))
    }

    pub fn rc(self)->Rc<Self> {
	Rc::new(self)
    }
    
    pub fn plot_with_transform(&self,h:&Homography,pl:&mut Plot) {
	for c in self.contents.iter() {
	    match c {
		&Content::Draw(Command::Points{ color,ref points }) =>
		    pl.command(Command::Points{ color,points:points.iter().map(|&p| h.apply(p)).collect() }),
		&Content::Draw(Command::Lines{ color,ref lines }) =>
		    pl.command(Command::Lines{
			color,
			lines:lines.iter()
			    .map(|line|
				 line.iter().map(|&p| h.apply(p)).collect()).collect() }),
		Content::Object(obj) => obj.plot_with_transform(h,pl),
		&Content::Transform{ transform,ref object } => {
		    let h = h.compose(transform);
		    object.plot_with_transform(&h,pl);
		}
	    }
	}
    }

    pub fn plot(&self,pl:&mut Plot) {
	self.plot_with_transform(&Homography::id(),pl);
    }

    pub fn transformed(self:&Rc<Self>,h:&Homography)->Rc<Object> {
	Rc::new(Object{
	    area:self.area.transform(h),
	    contents:vec![Content::Transform{ transform:h.clone(), object:Rc::clone(&self) }]
	})
    }
}

trait Transformable {
    fn transform(&self,h:&Homography)->Self;
}

impl Transformable for Rectangle {
    fn transform(&self,h:&Homography)->Self {
	let p0 = h.apply(self.a);
	let p1 = h.apply(self.a.with_x(self.b));
	let p2 = h.apply(self.b);
	let p3 = h.apply(self.b.with_y(self.a));
	let mut r0 = rectangle(p0,p1);
	r0.add_point(p2);
	r0.add_point(p3);
	r0
    }
}

pub fn glyph(font:&Font,color:Color,mut p0:Point,s0:f64,c:char)->Rc<Object> {
    let s = s0 / H;
    let area = rectangle(ORIGIN,point(W,H));
    p0 += (0.0,s*H);
    let pp0 = p0;

    let mut lines = Vec::new();
    let mut line = Vec::new();

    if let Some(w) = font.get(c) {
	// let m = w.len();
	let mut p = p0;
	for &(pen,jj,ii) in w.iter() {
	    let pp = pp0 + (s*jj as f64,-s*ii as f64);
	    if pen {
		if line.len() == 0 {
		    line.push(p);
		}
		line.push(pp);
	    } else {
		if line.len() > 0 {
		    let mut new_line = Vec::new();
		    new_line.append(&mut line);
		    lines.push(new_line);
		}
	    }
	    p = pp;
	}
	if line.len() > 0 {
	    let mut new_line = Vec::new();
	    new_line.append(&mut line);
	    lines.push(new_line);
	}
    }
    Rc::new(Object{ area,contents:vec![Content::Draw(Command::Lines{ color,lines })] })
}

pub fn text(font:&Font,color:Color,p0:Point,s:f64,t:&Text)->Rc<Object> {
    let mut r0 = rectangle(p0,p0);
    let mut contents : Vec<Content> = Vec::new();
    match t {
	&Text::Char(c) => {
	    let p1 = p0.with_x(r0.b);
	    let obj = glyph(font,color,p1,s,c);
	    let p2 = p1 + (s*W/H,s);
	    r0.add_rectangle(rectangle(p1,p2));
	    contents.push(Content::Object(obj));
	},
	Text::Seq(v) => {
	    for t in v.iter() {
		let obj = text(font,color,p0.with_x(r0.b),s,t);
		r0.add_rectangle(obj.area);
		contents.push(Content::Object(obj));
	    }
	},
	Text::Sup(t) => {
	    let p = p0.with_x(r0.b) + (0.0,-H/3.0);
	    let obj = text(font,color,p,2.0*s/3.0,t);
	    r0.add_rectangle(obj.area);
	    contents.push(Content::Object(obj));
	},
	Text::Sub(t) => {
	    let p1 = p0.with_x(r0.b) + (0.0, H/3.0);
	    let obj = text(font,color,p1,2.0*s/3.0,t);
	    r0.add_rectangle(obj.area);
	    contents.push(Content::Object(obj));
	}
    }
    Rc::new(Object{ area:r0,
		    contents })
}

#[derive(Debug)]
pub struct ParseError(String);

use std::fmt;
use std::error::Error;

impl fmt::Display for ParseError {
    fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result {
	write!(f,"{}",self)
    }
}

impl Error for ParseError {
}

macro_rules! error {
    ($str:literal,$($arg:expr),*) => {
	Err(ParseError(format!($str,$($arg),*)))
    };
    ($str:literal) => {
	Err(ParseError($str.to_string()))
    }
}

const EOF : char = '\x00';

struct Symbol {
    i:usize,
    c:char
}

fn cons(t1:Text,t2:Text)->Text {
    Text::Seq(vec![t1,t2])
}

fn parse_one<'a,'b>(u:&str,w:&'b [Symbol])->Result<(Text,&'b [Symbol]),ParseError> {
    match w[0].c {
	EOF => error!("Unexpected EOF"),
	'{' => {
	    let (t,w) = parse_inner(u,&w[1..])?;
	    match w[0].c {
		'}' => Ok((t,&w[1..])),
		_ => return error!("Expecting '}}' at {}",&u[w[0].i..])
	    }
	},
	'^' => error!("Bad superscript"),
	'_' => error!("Bad subscript"),
	c => Ok((Text::Char(c),&w[1..]))
    }
}

// X ::= '{' L '}'
//     | c
//
// Y ::= X
//     | X^X
//
// L ::= Y*

fn parse_two<'a,'b>(u:&'a str,w:&'b [Symbol])->Result<(Text,&'b [Symbol]),ParseError> {
    let (t1,w) = parse_one(u,w)?;
    match w[0].c {
	'^' => {
	    let (t2,w) = parse_one(u,&w[1..])?;
	    Ok((cons(t1,Text::Sup(Box::new(t2))),w))
	},
	'_' => {
	    let (t2,w) = parse_one(u,&w[1..])?;
	    Ok((cons(t1,Text::Sub(Box::new(t2))),w))
	},
	_ => Ok((t1,w))
    }
}


fn parse_inner<'a,'b>(u:&'a str,mut w:&'b [Symbol])->Result<(Text,&'b [Symbol]),ParseError> {
    let mut ts = Vec::new();
    loop {
	match w[0].c {
	    EOF | '}' => break,
	    _ => {
		let (t,wn) = parse_two(u,w)?;
		w = wn;
		ts.push(t);
	    }
	}
    }
    Ok((Text::Seq(ts),w))
}

#[test]
pub fn test_parser() {
    let u = "x^{e^{2πft + j(x^2+y^2) + x^2 + αy}";
    let v = Text::parse(u).unwrap();
    println!("{} -> {:#?}",u,v);
}

impl Text {
    pub fn parse<'a>(u:&'a str)->Result<Text,ParseError> {
	let mut v : Vec<Symbol> = u.char_indices().map(|(i,c)| Symbol{ i,c }).collect();
	v.push(Symbol{ i:u.len(),c:EOF });
	let w = &v;
	let (t,w) = parse_inner(u,w)?;
	if w[0].c == EOF {
	    Ok(t)
	} else {
	    error!("Incomplete input at {}",&u[w[0].i..])
	}
    }

    pub fn span(u:&str)->Text {
	Text::Seq(u.chars().map(|c| Text::Char(c)).collect())
    }
}
