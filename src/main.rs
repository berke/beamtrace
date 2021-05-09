mod charset0;
mod text;

use beamtrace::{point,Point,Rectangle,Book,Page,Plot,Command,Homography,Transformable};
use text::Text;
use charset0::Font;

fn main1() {
    let mut font = Font::new();
    font.add_ascii();
    font.add_math();
    let mut bk = Book::new();
    let mut pg = Page::new();

    let mut pl1 = Plot::new();
    let p0 = point(0.0,0.0);
    pl1.rgb12(0xfff);
    let r = text::text(&font,
    			&mut pl1,p0,1.0,
    			&Text::parse("FELIX et CASSIUS").unwrap());
    			//&Text::parse("e^{2πft + j(x^2+y^2)} + x^{2 + αy^{3 + z^{x + 33}}} - 5").unwrap());
    pl1.rgb12(0xff0);
    text::rect(&mut pl1,r);
    pl1.rgb12(0x0f0);
    let w = 0.5;
    pl1.line(p0,p0+(w,0.0));
    pl1.line(p0,p0-(w,0.0));
    pl1.line(p0,p0+(0.0,w));
    pl1.line(p0,p0-(0.0,w));

    let sx = r.b.x - r.a.x;
    let sy = r.b.y - r.a.y;

    let mut pl = Plot::new();
    let mut theta = 0.0;
    let ntheta = 30;
    let radius = sy * ntheta as f64 / (2.0*PI);
    use std::f64::consts::PI;
    for itheta in 0..ntheta {
	let theta = 2.0 * itheta as f64 * PI / ntheta as f64;
	let x = radius * theta.cos();
	let y = radius * theta.sin();
	let p = point(x,y);
	let mut pl2 = Plot::new();
	pl2.rotate(-(theta ));
	pl2.translate(p);
	pl2.group(pl1.clone());
	pl.group(pl2);
    }

    pg.plot(pl);
    bk.page(pg);
    bk.save_to_file("traj.mpk").unwrap();
}

//

use ndarray::{Array1};

fn crosshair(pl:&mut Plot,p:Point,w:f64) {
    let mut pl1 = Plot::new();
    pl1.rgb12(0xf00);
    pl1.line(point(-w,0.0),point(w,0.0));
    pl1.line(point(0.0,-w),point(0.0,w));
    pl.group(pl1)
}

fn main() {
    let mut font = Font::new();
    font.add_ascii();
    font.add_math();
    let mut bk = Book::new();
    let mut pg = Page::new();
    let mut pl = Plot::new();

    let f = |x:f64|->f64 { (x*x).cos() };

    let vscale = 5.0;
    let x0 = -10.0;
    let x1 = 10.0;
    let nx = 2500;
    let xs = Array1::linspace(x0,x1,nx);
    let pts : Vec<Point> = xs.iter().map(|&x| point(x,vscale*f(x))).collect();
    let mut r = Rectangle::bounding(&pts);
    r.a.x = x0;
    r.b.x = x1;
    pl.rgb12(0xf00);
    pl.rect(r);
    println!("Points: {:?}",pts);
    println!("Bounding box: {:?}",r);
    pl.rgb12(0xfff);
    pl.lines(pts);

    let mut pl1 = Plot::new();
    let mut r1 = text::text(&font,
    		       &mut pl1,point(0.0,0.0),
		       1.0,
		       &Text::parse("αcos(x^2)+βsin(y^2)+exp(-kT)^{-1}").unwrap());
    let mut pl2 = Plot::new();
    let w1 = r1.b.x - r1.a.x;
    let w = r.b.x - r.a.x;
    let sx = w / w1;
    let h1 = r1.b.x - r1.a.x;
    let h = r.b.x - r.a.x;
    let sy = 0.1 * h / h1;
    let s = sx.max(sy);


    let mut h = Homography::id();
    h.scale(s);

    let mut r1b = r1.clone();
    r1b.apply(&h);

    //h.translate(point(r.a.x - r1b.a.x,r1b.b.y - r.a.y));
    h.translate(point(r.a.x - r1b.a.x,r.a.y - r1b.b.y));

    pl2.transform(h);
    pl2.rgb12(0xfff);
    pl2.group(pl1);
    pl.group(pl2);

    crosshair(&mut pl,point(r.a.x - r1.a.x, r1.b.y - r1.a.y),s/2.0);

    pg.plot(pl);
    bk.page(pg);
    bk.save_to_file("traj.mpk").unwrap();
}
