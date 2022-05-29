use std::rc::Rc;
use std::f64::consts::PI;
use beambook::{geometry::{point,rectangle,Point,ORIGIN},Color,Command};
use ndarray::Array1;
use crate::{
    text::{self,Text,Object,Content},
    font::{Font,D,H},
    homography::Homography,
    maps::Map
};

const DEGREE : f64 = PI/180.0;

#[derive(PartialEq)]
pub enum Position {
    First,
    Middle,
    Last
}

pub fn left_justify(obj:&Rc<Object>)->Rc<Object> {
    let h = Homography::translation(point(-obj.area.b.x,0.0));
    obj.transformed(&h)

}

pub fn left_align(obj:&Rc<Object>)->Rc<Object> {
    let h = Homography::translation(point(-obj.area.a.x,0.0));
    obj.transformed(&h)
}

pub fn top_align(obj:&Rc<Object>)->Rc<Object> {
    let h = Homography::translation(point(0.0,-obj.area.a.y));
    obj.transformed(&h)
}

pub fn bottom_align(obj:&Rc<Object>)->Rc<Object> {
    let h = Homography::translation(point(0.0,-obj.area.b.y));
    obj.transformed(&h)
}

pub fn rot90(obj:&Rc<Object>)->Rc<Object> {
    let h = Homography::rotation(PI/2.0);
    obj.transformed(&h)
}

pub fn try_ticks<M,F> (d:isize,map:&M,mut label:F,min_spacing:f64)->Option<Vec<(f64,String)>>
where M:Map,F:FnMut(Position,f64)->String {
    let (x0,x1) = map.domain();
    let (y0,y1) = map.codomain();
    let _p = 2;
    let dy = 10.0_f64.powf((y1 - y0).abs().log10().floor()) / d as f64;
    let mut ticks = Vec::new();
    let mut y = y0 - y0.rem_euclid(dy);
    //for i in 0..m {
    let mut i = 0;
    while y <= y1 {
	let x = map.inverse(y);
	if x0 <= x && x <= x1 {
	    ticks.push((x,y));
	    if i > 0 && (ticks[i - 1].0 - x).abs() < min_spacing {
		return None
	    }
	    i += 1;
	}
	y += dy;
    }
    let m = ticks.len();
    Some(ticks.iter().enumerate().map(|(i,&(x,y))| {
	let pos =
	    if i == 0 {
		Position::First
	    } else if i + 1 == m {
		Position::Last
	    } else {
		Position::Middle
	    };
	(x,label(pos,y))
    }).collect())
}

pub fn ruler(font:&Font,size:f64,
	 p0:Point,p1:Point,
	 left:bool,rot:bool,
	 ticks:&[(f64,String)])->Object {
    //       w   g
    // y0 +------ 1
    //    |
    //    |
    //    +------ 2
    //    |
    //    |
    // y1 +------ 3

    //let mut current : Vec<Rc<Object>> = Vec::new();
    let _spacing = 2.0*size;

    let _theta = 0.0 * DEGREE;
    let _dl = (p1 - p0).norm();
    let dv = (p1 - p0).normalize();
    let du = if left { -1.0 } else { 1.0 } * point(dv.y,dv.x);

    let n = ticks.len();

    let mut current = Object::empty();
    current.contents.push(Content::Draw(Command::line(0xfff,p0,p1)));
    for i in 0..n {
	let &(v,ref txt) = &ticks[i];
	let p = p0 + v*dv;
	let pa = p + 2.0*size*du;
	let pb = pa + 0.5*size*du - (1.0-D/H)*size*dv;
	current.contents.push(Content::Draw(Command::line(0xfff,p,pa)));
	let obj = text::text(&font,
			     0xfff,
			     ORIGIN,
			     size,
			     &Text::parse(&txt).unwrap());
	let obj = if rot { top_align(&rot90(&obj)) } else { obj };
	let obj = if left { left_justify(&obj) } else { obj };
	let obj = obj.transformed(&Homography::translation(pb));
	current.add_object(&obj);
    }
    current
}

pub fn curve<F:FnMut(f64)->f64>(p0:Point,p1:Point,_p2:Point,
			    x_map:&dyn Map,
			    y_map:&dyn Map,
			    delta:f64,flip:bool,
			    color:Color,mut f:F)->Object {
    let mut obj = Object::empty();
    let du = (p1 - p0).normalize();
    let dv = if flip { -1.0 } else { 1.0 } * point(du.y,du.x);
    let dl = (p1 - p0).norm();
    let m = (dl / delta).ceil() as usize;
    let (u0,u1) = x_map.domain();
    // let (v0,v1) = y_map.domain();
    let us = Array1::linspace(u0,u1,m);
    let mut line = Vec::new();
    for i in 0..m {
	let u = us[i];
	let x = x_map.direct(u);
	let y = f(x);
	let v = y_map.inverse(y);
	let p = p0 + u*du + v*dv;
	line.push(p);
    }
    obj.contents.push(Content::Draw(Command::Lines{ color,lines:vec![line] }));
    obj
}

pub fn text_lines(font:&Font,size:f64,color:Color,text:&[&str])->Object {
    let obj = Object::empty();
    let mut obj = obj.rc();
    for t in text.iter() {
	let item = text::text(&font,
			      color,
			      ORIGIN,
			      size,
			      &Text::parse(t).unwrap());
	{
	    let x = Rc::get_mut(&mut obj).unwrap();
	    x.add_object(&item);
	}
	obj = bottom_align(&obj);
    }
    Rc::try_unwrap(obj).unwrap()
}

pub fn legend(font:&Font,size:f64,es:&[(Color,&str)],textcolor:Color)->Object {
    let width = 5.0 * size;
    let p1 = ORIGIN + width*point(1.0,0.0);
    let p2 = p1 + size*point(1.0,0.0);

    let mut obj = Object::empty();
    obj.area = rectangle(point(-size,0.0),p1);
    let mut obj = obj.rc();
    for &(c,ref label) in es.iter() {
	let item = text::text(&font,
			      textcolor,
			      p2,
			      size,
			      &Text::parse(label).unwrap());
	{
	    let x = Rc::get_mut(&mut obj).unwrap();
	    x.add_object(&item);
	    x.contents.push(Content::Draw(Command::line(c,ORIGIN,p1)));
	}
	obj = bottom_align(&obj);
    }
    Rc::try_unwrap(obj).unwrap()
}
