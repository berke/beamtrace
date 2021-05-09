mod charset0;
mod text;

use beamtrace::{point,Point,Book,Page,Plot,Command};
use text::Text;
use charset0::Font;

fn main() {
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
    println!("BK:\n{:#?}",bk);
}
