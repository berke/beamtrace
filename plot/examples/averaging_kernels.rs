// mod font;
// mod text;
// mod homography;
// mod maps;
// mod plot;

use beambook::{geometry::{point,rectangle,ORIGIN},Color,Book,Page,Plot};
use beamplot::{
    plot::*,
    font::Font,
    homography::Homography,
    maps::{Map,LinearMap,ExponentialMap}
};

fn main() {
    let font = Font::new();
    let mut bk = Book::new();
    let mut pg = Page::new();

    let origin = ORIGIN;

    let dx = 100.0;
    // let pressures = Array1::linspace(200.0,700.0,50);
    // for &pressure0 in pressures.iter() {
    let pressure0 = 350.0;
    let mut pl = Plot::new();

    let size = 10.0;
    let p1 = point(0.0,200.0);
    let p3 = dx*point(1.0,0.0);
    let p2 = p1 + p3;
    let y0 = 0.0;
    let y1 = 1013.25;
    let x0 = -0.5;
    let x1 = 1.5;

    let x_map = LinearMap::new(0.0,p3.x,x0,x1);
    let y_map = ExponentialMap::new(0.0,p1.y,y0,y1);
    let tick_spacing = size/4.0;
    let ticks_y = try_ticks(10,&y_map,
			    |pos,y| if pos == Position::Last { format!("hPa {}",y) } else { format!("{}",y) },
			    tick_spacing).unwrap();
    println!("Ticks Y: {:?}",ticks_y);
    let ticks_x = try_ticks(5,&x_map,|_,x| format!("{:.1}",x),tick_spacing).unwrap();
    println!("Ticks X: {:?}",ticks_x);

    ruler(&font,size,origin,p1,true,false,&ticks_y).plot(&mut pl);
    ruler(&font,size,p1,p2,false,true,&ticks_x).plot(&mut pl);
    // ruler(&font,x0,x1,p1,p2,size,false,true,
    // 	  |_,y| format!("{:.4}",y),
    // 	  |x0,x1,dl| {
    // 	      let m = ((1.0/dl).floor() as usize).max(2);
    // 	      let mut ticks = Array2::zeros((m,2));
    // 	      ticks.slice_mut(s![..,0]).assign(&Array1::linspace(0.0,1.0,m));
    // 	      ticks.slice_mut(s![..,1]).assign(&Array1::linspace(x0,x1,m));
    // 	      ticks
    // 	  }).plot(&mut pl);
    // let dx = size*point(20.0,0.0);
    // ruler(&font,0.0,101325.0,ORIGIN+dx,p1+dx,size).plot(&mut pl);
    // .plot(&mut pl);

    if x0*x1 < 0.0 { pl.line(0x888,origin+x_map.inverse(0.0)*point(1.0,0.0),origin+p1+x_map.inverse(0.0)*point(1.0,0.0)); }
    pl.rectangle(0xfff,rectangle(origin,p2));
    let mut f1 = |p:f64| (-sq((p - 0.2*pressure0)/150.0)).exp() + 0.05*(p/25.0).cos();
    let mut f2 = |p:f64| (-sq((p - 1.0*pressure0)/150.0)).exp() + 0.10*(p/30.0).cos();
    let mut f3 = |p:f64| (-sq((p - 1.5*pressure0)/150.0)).exp() + 0.15*(p/35.0).cos();
    let mut g = |f:&mut dyn Fn(f64)->f64,color:Color| {
	curve(origin,p1,p2,
	      &y_map,
	      &x_map,
	      1.0,
	      false,
	      color,
	      f).plot(&mut pl)
    };
    g(&mut f1,0xf00);
    g(&mut f2,0x0f0);
    g(&mut f3,0x0ff);
    bottom_align(&text_lines(&font, size, 0xfff, &["T"]).rc()).transformed(&Homography::translation(origin)).plot(&mut pl);

    let origin = origin + point(size,0.0) + p3;
    if x0*x1 < 0.0 { pl.line(0x888,origin+x_map.inverse(0.0)*point(1.0,0.0),origin+p1+x_map.inverse(0.0)*point(1.0,0.0)); }
    pl.rectangle(0xfff,rectangle(origin,origin+p2));
    let mut f1 = |p:f64| (-sq((p - 0.2*pressure0)/10.0)).exp() + 0.05*(p/25.0).cos();
    let mut f2 = |p:f64| (-sq((p - 1.0*pressure0)/10.0)).exp() + 0.10*(p/30.0).cos();
    let mut f3 = |p:f64| (-sq((p - 1.5*pressure0)/10.0)).exp() + 0.15*(p/35.0).cos();
    let mut g = |f:&mut dyn Fn(f64)->f64,color:Color| {
	curve(origin,origin+p1,origin+p2,
	      &y_map,
	      &x_map,
	      1.0,
	      false,
	      color,
	      f).plot(&mut pl)
    };
    g(&mut f1,0xf00);
    g(&mut f2,0x0f0);
    g(&mut f3,0x0ff);
    ruler(&font,size,origin+p1,origin+p2,false,true,&ticks_x).plot(&mut pl);
    bottom_align(&text_lines(&font, size, 0xfff, &["H_2O"]).rc()).transformed(&Homography::translation(origin)).plot(&mut pl);

    let origin = origin + point(size,0.0) + p3;
    if x0*x1 < 0.0 { pl.line(0x888,origin+x_map.inverse(0.0)*point(1.0,0.0),origin+p1+x_map.inverse(0.0)*point(1.0,0.0)); }
    pl.rectangle(0xfff,rectangle(origin,origin + p2));
    let mut f1 = |p:f64| (-sq((p - 0.2*pressure0)/50.0)).exp() + 0.15*(p/15.0).cos();
    let mut f2 = |p:f64| (-sq((p - 1.0*pressure0)/50.0)).exp() + 0.05*(p/20.0).cos();
    let mut f3 = |p:f64| (-sq((p - 1.5*pressure0)/50.0)).exp() + 0.10*(p/25.0).cos();
    let mut f4 = |p:f64| (-sq((p - 1.2*pressure0)/30.0)).exp() + 0.12*(p/50.0).cos();
    let mut g = |f:&mut dyn Fn(f64)->f64,color:Color| {
	curve(origin,origin + p1,origin + p2,
	      &y_map,
	      &x_map,
	      1.0,
	      false,
	      color,
	      f).plot(&mut pl)
    };
    g(&mut f1,0xf00);
    g(&mut f2,0x0f0);
    g(&mut f3,0x0ff);
    g(&mut f4,0xff0);
    ruler(&font,size,origin+p1,origin+p2,false,true,&ticks_x).plot(&mut pl);
    bottom_align(&text_lines(&font, size, 0xfff, &["CH_4"]).rc()).transformed(&Homography::translation(origin)).plot(&mut pl);

    let subtitle =
	left_align(&top_align(&legend(&font,size,
				      &[(0xf00,"IASI"),
					(0x0f0,"TROPOMI"),
					(0x0ff,"IASI+L1(TROPOMI)"),
					(0xff0,"IASI+L2(TROPOMI)")],
				      0xfff).rc())).transformed(&Homography::translation(origin + p3));
    subtitle.plot(&mut pl);

    bottom_align(
	&text_lines(&font,
		    1.5*size,
		    0xfff,
		    &[
			&format!("Averaging kernels, p_0={:.3} hPa",
				 pressure0),
			"Note that the kernels have been normalized by pressure",
			"Error amplification: 3x,1.5x, Q=σAT^4"
		    ]).rc())
	.transformed(&Homography::translation(point(0.0,subtitle.area.a.y - 2.0*size))).plot(&mut pl);
    
    pg.plot(pl);
    // }
    bk.page(pg);
    bk.save_to_file("traj.mpk").unwrap();
}

fn sq(x:f64)->f64 { x*x }
