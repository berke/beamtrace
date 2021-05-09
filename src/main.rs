mod font;
mod text;
mod homography;

use std::rc::Rc;
use std::f64::consts::PI;
use beamtrace::{geometry::{point,rectangle,Point,Rectangle,ORIGIN},Color,Book,Page,Plot,Command};
use text::{Text,Object,Content};
use font::{Font,W,D,H};
use homography::Homography;
use ndarray::{Array1,Array2,s};

const DEGREE : f64 = PI/180.0;

fn make_rotated_copies(obj:&Rc<Object>,ntheta:usize,theta0:f64)->Rc<Object> {
    let mut container = Object::empty();
    let r = ntheta as f64 * obj.area.dy() / (2.0*PI*obj.area.dx());
    for itheta in 0..ntheta {
	let theta = theta0 + 2.0 * itheta as f64 * PI / ntheta as f64;
	let h = Homography::rotation(theta).compose(Homography::translation(point(r,0.0)));
	let obj2 = obj.transformed(&h);
	container.add_object(&obj2);
    }
    container.rc()
}

fn draw_bounding_box(color:Color,obj:&Rc<Object>)->Rc<Object> {
    let mut obj1 = Object::empty();
    obj1.add_object(&obj);
    obj1.contents.push(Content::Draw(Command::rectangle(color,obj.area)));
    obj1.rc()
}

fn main1() {
    let mut font = Font::new();
    let mut bk = Book::new();


    let ntheta0 = 100;
    for itheta0 in 0..ntheta0 {
	let theta0 = 2.0 * itheta0 as f64 * PI / ntheta0 as f64;
	let obj : Rc<Object> = text::text(&font,
					  0xf00,
					  point(0.0,0.0),
					  1.0,
					  //&Text::parse("2ln(x)+αcos(x^2)+βsin(y^2)+exp(-kT)^{-1}").unwrap());
					  &Text::parse(&format!("{:.3}",theta0)).unwrap());
	let mut pg = Page::new();
	let mut pl = Plot::new();
	let obj1 = draw_bounding_box(0xff0,&obj);
	let obj2 = make_rotated_copies(&obj1,5,theta0);
	let obj3 = draw_bounding_box(0x00f,&obj2);
	let obj4 = obj3.transformed(&Homography::translation(point(12.0 + 10.0*theta0,3.4)));
	let obj5 = make_rotated_copies(&obj4,3,theta0);
	let obj6 = draw_bounding_box(0x481,&obj5);
	let obj7 = obj6.transformed(&Homography::translation(point(0.0,50.0)));
	let obj8 = make_rotated_copies(&obj7,7,theta0);
	let obj9 = draw_bounding_box(0xfff,&obj8);
	obj9.plot(&mut pl);
	pg.plot(pl);
	bk.page(pg);
    }
    bk.save_to_file("traj.mpk").unwrap();
}

#[derive(PartialEq)]
enum Position {
    First,
    Middle,
    Last
}

fn left_justify(obj:&Rc<Object>)->Rc<Object> {
    let h = Homography::translation(point(-obj.area.b.x,0.0));
    obj.transformed(&h)

}

fn left_align(obj:&Rc<Object>)->Rc<Object> {
    let h = Homography::translation(point(-obj.area.a.x,0.0));
    obj.transformed(&h)
}

fn top_align(obj:&Rc<Object>)->Rc<Object> {
    let h = Homography::translation(point(0.0,-obj.area.a.y));
    obj.transformed(&h)
}

fn bottom_align(obj:&Rc<Object>)->Rc<Object> {
    let h = Homography::translation(point(0.0,-obj.area.b.y));
    obj.transformed(&h)
}

fn rot90(obj:&Rc<Object>)->Rc<Object> {
    let h = Homography::rotation(PI/2.0);
    obj.transformed(&h)
}

fn ruler<F,G>(font:&Font,y0:f64,y1:f64,p0:Point,p1:Point,size:f64,left:bool,rot:bool,mut label:F,mut ticks:G)->Object
where F:FnMut(Position,f64)->String,
      G:FnMut(f64,f64,f64)->Array2<f64> {
    //       w   g
    // y0 +------ 1
    //    |
    //    |
    //    +------ 2
    //    |
    //    |
    // y1 +------ 3

    //let mut current : Vec<Rc<Object>> = Vec::new();
    let spacing = 2.0*size;

    let theta = 0.0 * DEGREE;
    let dl = (p1 - p0).norm();
    let dv = (p1 - p0).normalize();
    let du = if left { -1.0 } else { 1.0 } * point(dv.y,dv.x);

    let vys = ticks(y0,y1,(size+spacing)/dl);
    let (n,_) = vys.dim();

    let mut current = Object::empty();
    current.contents.push(Content::Draw(Command::line(0xfff,p0,p1)));
    for i in 0..n {
	let v = vys[[i,0]];
	let y = vys[[i,1]];
	let position =
	    if i == 0 {
		Position::First
	    } else if i + 1 == n {
		Position::Last
	    } else {
		Position::Middle
	    };
	let txt = label(position,y);
	let p = p0 + v*dl*dv;
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

fn curve<F:FnMut(f64)->f64>(p0:Point,p1:Point,p2:Point,
			    x0:f64,x1:f64,
			    y0:f64,y1:f64,
			    delta:f64,flip:bool,
			    color:Color,mut f:F)->Object {
    let mut obj = Object::empty();
    let du = (p1 - p0);
    let dv = if flip { -1.0 } else { 1.0 } * point(du.y,du.x);
    let dl = (p1 - p0).norm();
    let m = (dl / delta).ceil() as usize;
    let xs = Array1::linspace(x0,x1,m);
    let mut line = Vec::new();
    for i in 0..m {
	let x = xs[i];
	let y = f(x);
	let p = p0 + (x-x0)/(x1-x0)*du + (y-y0)/(y1-y0)*dv;
	line.push(p);
    }
    obj.contents.push(Content::Draw(Command::Lines{ color,lines:vec![line] }));
    obj
}

fn text_lines(font:&Font,size:f64,color:Color,text:&[&str])->Object {
    let mut obj = Object::empty();
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

fn legend(font:&Font,size:f64,es:&[(Color,&str)],textcolor:Color)->Object {
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

fn main() {
    let mut font = Font::new();
    let mut bk = Book::new();
    let mut pg = Page::new();

    let dx = 200.0;
    // let pressures = Array1::linspace(200.0,700.0,50);
    // for &pressure0 in pressures.iter() {
	let pressure0 = 350.0;
	let mut pl = Plot::new();

	let mut obj = Object::empty();
	let size = 10.0;
	let p1 = point(0.0,200.0);
	let p3 = dx*point(1.0,0.0);
	let p2 = p1 + p3;
	let y0 = 0.0;
	let y1 = 1013.25;
	let x0 = -0.1;
	let x1 = 2.0;
	ruler(&font,y0,y1,ORIGIN,p1,size,true,false,
	      |pos,y| if pos == Position::Last { format!("hPa {:.3}",y) } else { format!("{:.3}",y) },
	      |y0,y1,dl| {
		  let m = ((1.0/dl).floor() as usize).max(2);
		  let mut ticks = Array2::zeros((m,2));
		  let lus : Array1<f64> = Array1::linspace(0.0,-4.0,m);
		  // ticks.slice_mut(s![..,0]).assign(&Array1::linspace(0.0,1.0,m));
		  // ticks.slice_mut(s![..,1]).assign(&Array1::linspace(y0,y1,m));
		  for i in 0..m {
		      ticks[[i,0]] = 1.0-lus[i].exp();
		      ticks[[i,1]] = y0 + (y1-y0)*ticks[[i,0]];
		  }
		  ticks[[m-1,0]] = 1.0;
		  ticks[[m-1,1]] = y1;
		  ticks
	      }).plot(&mut pl);
	ruler(&font,x0,x1,p1,p2,size,false,true,
	      |_,y| format!("{:.4}",y),
	      |x0,x1,dl| {
		  let m = ((1.0/dl).floor() as usize).max(2);
		  let mut ticks = Array2::zeros((m,2));
		  ticks.slice_mut(s![..,0]).assign(&Array1::linspace(0.0,1.0,m));
		  ticks.slice_mut(s![..,1]).assign(&Array1::linspace(0.0,x1,m));
		  ticks
	      }).plot(&mut pl);
	// let dx = size*point(20.0,0.0);
	// ruler(&font,0.0,101325.0,ORIGIN+dx,p1+dx,size).plot(&mut pl);
	// .plot(&mut pl);
	pl.rectangle(0xfff,rectangle(ORIGIN,p2));
	let mut f1 = |p:f64| (-sq((p - 0.2*pressure0)/150.0)).exp() + 0.05*(p/25.0).cos();
	let mut f2 = |p:f64| (-sq((p - 1.0*pressure0)/150.0)).exp() + 0.10*(p/30.0).cos();
	let mut f3 = |p:f64| (-sq((p - 1.5*pressure0)/150.0)).exp() + 0.15*(p/35.0).cos();
	let mut g = |f:&mut Fn(f64)->f64,color:Color| {
	    curve(ORIGIN,p1,p2,
		  y0,y1,
		  x0,x1,
		  1.0,
		  false,
		  color,
		  f).plot(&mut pl)
	};
	g(&mut f1,0xf00);
	g(&mut f2,0x0f0);
	g(&mut f3,0x0ff);

	bottom_align(
	    &text_lines(&font,
			size,
			0xfff,
			&[
			    &format!("Averaging kernels, p_0={:.3} hPa",
				     pressure0),
			    "Note that the kernels have been normalized by pressure",
			    "Error amplification: 3x,1.5x, Q=σAT^4"
			]).rc()).plot(&mut pl);

	left_align(&top_align(&legend(&font,size,
				      &[(0xf00,"F1"),
					(0x0f0,"F2"),
					(0x0ff,"F2")],
				      0xfff).rc())).transformed(&Homography::translation(p3)).plot(&mut pl);
	
	pg.plot(pl);
    // }
    bk.page(pg);
    bk.save_to_file("traj.mpk").unwrap();
}

fn sq(x:f64)->f64 { x*x }
