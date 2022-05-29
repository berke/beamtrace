mod charset0;

use beamtrace::{point,Point,Book,Page,Plot,Command};

const W : f64 = 7.0;
const D : f64 = 4.0;
const H : f64 = 11.0;

//         +---------+ --/-----/-
//         |         |         |
//         |         |         |
//         |         |         |
//         |         |         |
//         |         |         | H
// (x0,y0) +---------+ --/--   |
//         |         |   |     |
//         |         |   | D   |
//         |         |   |     |
//         +---------+ --/-----/-
//
//         |         |
//         |         |
//         /---------/
//         |    W    |

fn rect(pl:&mut Plot,p0:Point,p1:Point) {
    pl.lines(vec![p0,p0.with_x(p1),p1,p1.with_x(p0),p0]);
}

fn glyph(plot:&mut Plot,mut p0:Point,s:f64,c:char) {
    p0 += (0.0,s*H);
    let pp0 = p0;

    let mut line = Vec::new();

    if c.is_ascii() {
	let k : u32 = c.into();
	if k < 128 {
	    let w = charset0::FONT0[k as usize];
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
			plot.lines(new_line);
		    }
		}
		p = pp;
	    }
	    if line.len() > 0 {
		let mut new_line = Vec::new();
		new_line.append(&mut line);
		plot.lines(new_line);
	    }
	}
    }
}

#[derive(Clone,Debug)]
pub enum Text<'a> {
    Span(&'a str),
    Sup(Box<Text<'a>>),
    Sub(Box<Text<'a>>),
    Seq(Vec<Text<'a>>)
}

fn span(pl:&mut Plot,mut p0:Point,s:f64,u:&str)->Point {
    for c in u.chars() {
	pl.rgb12(0xf00);
	rect(pl,p0,p0+(s*W,s*(H-D)));
	rect(pl,p0+(0.0,s*(H-D)),p0+(s*W,s*H));
	pl.rgb12(0xfff);
	glyph(pl,p0,s,c);
	p0 += (s*W,0.0);
    }
    p0+(0.0,s*H)
}


fn text(pl:&mut Plot,mut p0:Point,s:f64,t:&Text)->Point {
    let mut p1 = p0;
    match t {
	Text::Span(u) => p1 = p0.with_x(span(pl,p0,s,u)),
	Text::Seq(v) => {
	    for t in v.iter() {
		p1 = text(pl,p0,s,t);
		p0 = p0.with_x(p1);
	    }
	},
	Text::Sup(t) => {
	    p1 = text(pl,p0 + (0.0,-H/3.0),2.0*s/3.0,t);
	    p0 = p0.with_x(p1);
	},
	Text::Sub(t) => {
	    p1 = text(pl,p0 + (0.0,H/3.0),2.0*s/3.0,t);
	    p0 = p0.with_x(p1);
	}
    }
    p1
}


// fn main0() {
//     let mut bk = Book::new();

//     for k in 32..126 {
// 	let mut pg = Page::new();
// 	let mut pl = Plot::new();
// 	let c : char = k.into();
// 	let w = 7.0;
// 	let h = 11.0;
// 	// pl.color(1.0,1.0,0.0);
// 	// pl.text((0.0,0.0),0.1,&format!("x"));
// 	// pl.text((w,h),0.1,&format!("x"));
// 	pl.rgb12(0xf00);
// 	rect(pl,(0.0,0.0),(W,H-D));
// 	rect(pl,(0.0,H-D),(W,H));
// 	pl.rgb12(0xfff);
// 	glyph(pl,(0.0,0.0),1.0,c);
// 	pg.plot(pl);
// 	bk.page(pg);

//     }
//     bk.save_to_file("traj.mpk").unwrap();
//     println!("BK:\n{:?}",bk);
// }

fn main() {
    let mut bk = Book::new();
    let mut pg = Page::new();
    let mut pl = Plot::new();
    text(&mut pl,point(0.0,0.0),1.0,
	 &Text::Seq(vec![Text::Span("HELLO"),
			 Text::Sub(Box::new(
				 Text::Seq(vec![
				     Text::Span("2"),
				     Text::Sup(Box::new(Text::Span("128")))]
				 ))),
			 Text::Span("WORLD")]));
    pg.plot(pl);
    bk.page(pg);
    bk.save_to_file("traj.mpk").unwrap();
    println!("BK:\n{:?}",bk);
}
