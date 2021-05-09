use beamtrace::{point,rectangle,Point,Rectangle,Book,Page,Plot,Command};
use crate::charset0::{Font,W,D,H};

#[derive(Clone,Debug)]
pub enum Text {
    Char(char),
    Sup(Box<Text>),
    Sub(Box<Text>),
    Seq(Vec<Text>)
}

pub fn rect(pl:&mut Plot,Rectangle{ a:p0,b:p1 }:Rectangle) {
    pl.lines(vec![p0,p0.with_x(p1),p1,p1.with_x(p0),p0]);
}

pub fn glyph(font:&Font,pl:&mut Plot,mut p0:Point,s:f64,c:char) {
    pl.rgb12(0xf00);
    rect(pl,rectangle(p0,p0+(s*W,s*(H-D))));
    rect(pl,rectangle(p0+(0.0,s*(H-D)),p0+(s*W,s*H)));
    pl.rgb12(0xfff);

    p0 += (0.0,s*H);
    let pp0 = p0;

    let mut line = Vec::new();

    if let Some(w) = font.get(c) {
	let m = w.len();
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
		    pl.lines(new_line);
		}
	    }
	    p = pp;
	}
	if line.len() > 0 {
	    let mut new_line = Vec::new();
	    new_line.append(&mut line);
	    pl.lines(new_line);
	}
    }
}

pub fn text(font:&Font,pl:&mut Plot,mut p0:Point,s:f64,t:&Text)->Rectangle {
    let mut r0 = rectangle(p0,p0);
    match t {
	&Text::Char(c) => {
	    let p1 = p0.with_x(r0.b);
	    glyph(font,pl,p1,s,c);
	    let p2 = p1 + (s*W,s*H);
	    r0.add_rectangle(rectangle(p1,p2));
	},
	Text::Seq(v) => {
	    for t in v.iter() {
		let r = text(font,pl,p0.with_x(r0.b),s,t);
		r0.add_rectangle(r);
	    }
	},
	Text::Sup(t) => {
	    let p = p0.with_x(r0.b) + (0.0,-H/3.0);
	    let r = text(font,pl,p,2.0*s/3.0,t);
	    r0.add_rectangle(r);
	},
	Text::Sub(t) => {
	    let p1 = p0.with_x(r0.b) + (0.0, H/3.0);
	    let r = text(font,pl,p1,2.0*s/3.0,t);
	    r0.add_rectangle(r);
	}
    }
    r0
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

use std::str::Chars;

const EOF : char = '\x00';

struct Symbol {
    i:usize,
    c:char
}

fn cons(t1:Text,t2:Text)->Text {
    Text::Seq(vec![t1,t2])
}

fn parse_one<'a,'b>(u:&str,mut w:&'b [Symbol])->Result<(Text,&'b [Symbol]),ParseError> {
    println!("ONE [{}]",&u[w[0].i..w[w.len() - 1].i]);
    match w[0].c {
	EOF => error!("Unexpected EOF"),
	'{' => {
	    println!("LBRA");
	    let (t,w) = parse_inner(u,&w[1..])?;
	    println!("RBRA");
	    match w[0].c {
		'}' => Ok((t,&w[1..])),
		_ => return error!("Expecting '}}' at {}",&u[w[0].i..])
	    }
	},
	'^' => error!("Bad superscript"),
	c => Ok((Text::Char(w[0].c),&w[1..]))
    }
}

// X ::= '{' L '}'
//     | c
//
// Y ::= X
//     | X^X
//
// L ::= Y*

fn parse_two<'a,'b>(u:&'a str,mut w:&'b [Symbol])->Result<(Text,&'b [Symbol]),ParseError> {
    let (t1,w) = parse_one(u,w)?;
    match w[0].c {
	'^' => {
	    let (t2,w) = parse_one(u,&w[1..])?;
	    Ok((cons(t1,Text::Sup(Box::new(t2))),w))
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
}
