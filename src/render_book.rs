mod bresenham;

use std::error::Error;
use ndarray::Array3;
use beamtrace::{Book,Command};

fn rgb12_to_color(x:u16)->[u8;3] {
    [(((x >> 4) & 0xf0) | ((x >> 8) & 0x0f)) as u8,
     (( x       & 0xf0) | ((x >> 4) & 0x0f)) as u8,
     (((x << 4) & 0xf0) | ( x       & 0x0f)) as u8]
}

fn draw_disk(u:&mut Array3<u8>,
	     i0:i32,
	     j0:i32,
	     r:i32,
	     color:[u8;3]) {
    let (m,n,_) = u.dim();
    for di in -r..=r {
	let i = i0 + di;
	if 0 <= i && i < m as i32 {
	    for dj in -r..=r {
		let j = j0 + dj;
		if 0 <= j && j < n as i32 {
		    if di*di + dj*dj < r*r {
			u[[i as usize,j as usize,0]] = color[0];
			u[[i as usize,j as usize,1]] = color[1];
			u[[i as usize,j as usize,2]] = color[2];
		    }
		}
	    }
	}
    }
}

fn main()->Result<(),Box<dyn Error>> {
    let in_fn = std::env::args().nth(1).expect("Specify input path");
    let out_base = std::env::args().nth(2).expect("Specify output base");
    let bk = Book::load_from_file(in_fn).unwrap();

    let ss = 16;
    let ny = 1024;
    let nx = 1920;

    let invert = 0xfff;

    let mut bm : Array3<u8> = Array3::from_elem((ny*ss,nx*ss,3),(invert & 0xff) as u8);

    let size = 11;
    let margin = 16.0;

    for (ipage,page) in bk.pages.iter().enumerate() {
	for (iplot,plot) in page.plots.iter().enumerate() {
	    let mut rect = plot.bounding();
	    println!("Page bounds: {:?}",rect);
	    let scale = (nx as f64 / rect.dx()).min(ny as f64 / rect.dy());
	    println!("Scale factor: {}",scale);
	    rect.a.x -= margin/scale;
	    rect.a.y -= margin/scale;
	    rect.b.x += margin/scale;
	    rect.b.y += margin/scale;
	    let scale = (nx as f64 / rect.dx()).min(ny as f64 / rect.dy());
	    let scale = ss as f64 * scale;
	    for cmd in plot.commands.iter() {
		match cmd {
		    &Command::Points{ color,ref points } => {
			for &p in points.iter() {
			    let p = p - rect.a;
			    let ix = (p.x * scale).floor() as i32;
			    let iy = (p.y * scale).floor() as i32;
			    draw_disk(&mut bm,ix,iy,size,rgb12_to_color(color ^ invert));
			}
		    },
		    &Command::Lines{ color,ref lines } => {
			for line in lines.iter() {
			    for ps in line.windows(2) {
				let p1 = ps[0] - rect.a;
				let p2 = ps[1] - rect.a;
				let ix1 = (p1.x * scale).floor() as i32;
				let iy1 = (p1.y * scale).floor() as i32;
				let ix2 = (p2.x * scale).floor() as i32;
				let iy2 = (p2.y * scale).floor() as i32;
				let color = rgb12_to_color(color ^ invert);
				for (ix,iy) in bresenham::Bresenham::new(ix1,iy1,ix2,iy2) {
				    draw_disk(&mut bm,iy,ix,size,color);
				}
			    }
			}
		    }
		}
	    }

	    let mut w : Array3<u8> = Array3::zeros((ny,nx,3));
	    for iy in 0..ny {
		for ix in 0..nx {
		    let mut cnt = [0_usize;3];
		    for u in 0..ss {
			for v in 0..ss {
			    cnt[0] += bm[[ss*iy+u,ss*ix+v,0]] as usize;
			    cnt[1] += bm[[ss*iy+u,ss*ix+v,1]] as usize;
			    cnt[2] += bm[[ss*iy+u,ss*ix+v,2]] as usize;
			}
		    }
		    let c = [(cnt[0] / (ss * ss)) as u8,
			     (cnt[1] / (ss * ss)) as u8,
			     (cnt[2] / (ss * ss)) as u8];
		    for ic in 0..3 {
			w[[iy,ix,ic]] = c[ic];
		    }
		}
	    }

	    let out_fn = format!("{}-{:03}-{:03}.png",out_base,ipage,iplot);
	    ndarray_image::save_image(out_fn,w.view(),ndarray_image::Colors::Rgb)?;
	}
    }
    Ok(())
}
