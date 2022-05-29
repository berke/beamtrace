use beambook::{geometry::{point,rectangle,Point,Rectangle,ORIGIN},Color,Command};

/// Returns [true] if the segment [p0,p1] entirely fits into the rectangle.
pub fn segment_fits(rect:&Rectangle,p0:Point,p1:Point)->bool {
    rect.contains(p0) && rect.contains(p1)
}

// Half plane defined by p.u - s >= 0
pub fn clip_segment_half_plane(u:Point,s:f64,p0:Point,p1:Point)->Option<(Point,Point)> {
    let c0 = u.dot(p0) >= s;
    let c1 = u.dot(p1) >= s;
    match (c0,c1) {
	(false,false) => None,
	(true,true) => Some((p0,p1)),
	(true,false)|(false,true) => {
	    // Reorder so that p0 is inside, p1 outside
	    let (p0,p1) =
		if c0 {
		    (p0,p1)
		} else {
		    (p1,p0)
		};
	    let v = p1 - p0;
	    // Find largest λ such that
	    //   u·(p0 + λv) = s
	    //   u·p0 + λu·v = s
	    //   λu·v = s - u·p0
	    //   λ = (s - u·p0)/(u·v)
	    let lambda = (s - u.dot(p0))/u.dot(v);
	    Some((p0,p0 + lambda*v))
	}
    }
}

pub fn clip_segment_rect(rect:&Rectangle,p0:Point,p1:Point)->Option<(Point,Point)> {
    clip_segment_half_plane(point(1.0,0.0),rect.a.x,p0,p1)
	.and_then(|(p0,p1)| clip_segment_half_plane(point(-1.0,0.0),-rect.b.x,p0,p1))
	.and_then(|(p0,p1)| clip_segment_half_plane(point(0.0,1.0),rect.a.y,p0,p1))
	.and_then(|(p0,p1)| clip_segment_half_plane(point(0.0,-1.0),-rect.b.y,p0,p1))
}

pub struct LineBuilder {
    lines:Vec<Vec<Point>>,
    line:Vec<Point>
}

impl LineBuilder {
    pub fn new()->Self {
	Self{ lines:Vec::new(),line:Vec::new() }
    }

    pub fn flush_line(&mut self) {
	if !self.line.is_empty() {
	    let mut new_line = Vec::new();
	    new_line.append(&mut self.line);
	    self.lines.push(new_line);
	}
    }

    pub fn add_segment(&mut self,p0:Point,p1:Point) {
	if self.line.is_empty() {
	    self.line.push(p0);
	    self.line.push(p1);
	} else {
	    if self.line[self.line.len() - 1].equivalent(p0) {
		self.line.push(p1);
	    } else {
		self.flush_line();
		self.line.push(p0);
		self.line.push(p1);
	    }
	}
    }

    pub fn finish(mut self)->Vec<Vec<Point>> {
	self.flush_line();
	self.lines
    }
}

pub fn clip_line(rect:&Rectangle,line:&[Point])->Vec<Vec<Point>> {
    let mut lb = LineBuilder::new();
    let m = line.len();
    for i in 1..m {
	let p0 = line[i - 1];
	let p1 = line[i];
	let seg = 
	    if segment_fits(rect,p0,p1) {
		Some((p0,p1))
	    } else {
		clip_segment_rect(rect,p0,p1)
	    };
	if let Some((q0,q1)) = seg {
	    lb.add_segment(q0,q1)
	}
    }
    lb.finish()
}

#[cfg(test)]
fn sample_along_segment(p0:Point,p1:Point)->Point {
    p0 + fastrand::f64()*(p1-p0)
}

fn segment_contains(p0:Point,p1:Point,p:Point,tol:f64)->bool {
    let t = p1 - p0;
    let r = t.norm();
    let u = t/r;
    let v = u.ortho();
    let dp = p - p0;
    if v.dot(dp).abs() < tol {
	let w = u.dot(dp);
	-tol < w && w < r + tol
    } else {
	false
    }
}

#[test]
fn test_clip() {
    let tol = std::f64::EPSILON.sqrt();
    let f = |r:f64| r*(2.0*fastrand::f64() - 1.0);
    let d = 50.0;
    let ntest = 200;
    for iter in 0..ntest {
	let (x0,x1,y0,y1) = (f(d),f(d),f(d),f(d));
	let (x0,x1) = (x0.min(x1),x0.max(x1));
	let (y0,y1) = (y0.min(y1),y0.max(y1));
	let r = rectangle(point(x0,y0),point(x1,y1));
	let s = fastrand::f64() * 2.0 * d;
	let (x0,x1,y0,y1) = (f(s),f(s),f(s),f(s));
	let p0 = point(x0,y0);
	let p1 = point(x1,y1);
	for ncheck in 0..ntest {
	    let p = sample_along_segment(p0,p1);
	    let sc = segment_contains(p0,p1,p,tol);
	    if !sc {
		panic!("p0={:?} p1={:?} p={:?} sc={}",p0,p1,p,sc);
	    }
	}
	let s = clip_segment_rect(&r,p0,p1);
	// println!("{:?} {:?} -> {:?}",
	// 	 r,
	// 	 (p0,p1),
	// 	 s);
	
	// Make sure segment is entirely contained
	match s {
	    None => (),
	    Some((q0,q1)) =>
		for ncheck in 0..ntest {
		    let q = sample_along_segment(q0,q1);
		    let rc = r.contains(q);
		    let sc = segment_contains(p0,p1,q,tol);
		    assert!(sc);
		    if !rc || !sc {
			panic!("q0={:?} q1={:?} q={:?} rc={} sc={}",
			       q0,q1,q,rc,sc);
		    }
		}
	}

	for ncheck in 0..ntest {
	    let p = sample_along_segment(p0,p1);
	    let sc =
		s.map(|(q0,q1)| segment_contains(q0,q1,p,tol))
		.unwrap_or(false);
	    let rc = r.contains(p);
	    assert!(rc == sc);
	}
    }
}
