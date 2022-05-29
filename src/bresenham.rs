// assume x1=0 and y1=0
// L = { (c*dx,c*dy) | c >= 0 }
// x=c*dx, y=c*dy thus c=y/dy and x=y/dy*dx
// if we increase y:
//    y'=y+1
// x'=y'/dy*dx=(y+1)*dx/dy=y*dx/dy + dx/dy
// maintain error e as a fraction with fixed and implicit denominator dy

pub struct Bresenham {
    swap:bool,
    done:bool,
    sa:i32,
    sb:i32,
    a:i32,
    b:i32,
    b2:i32,
    e:i32,
    da:i32,
    db:i32
}

impl Bresenham {
    pub fn new(x1:i32,y1:i32,x2:i32,y2:i32)->Self {
	let dx = x2 - x1;
	let dy = y2 - y1;

	let f = |a1:i32,b1:i32,a2:i32,b2:i32,sa:i32,sb:i32,swap:bool| {
	    let a = a1;
	    let b = b1;
	    let e = 0;
	    let da = a2 - a1;
	    let db = b2 - b1;
	    Self{
		swap,
		done:false,
		sa,
		sb,
		a,
		b,
		b2,
		e,
		da,
		db
	    }
	};
	
	if dy.abs() > dx.abs() {
	    if y1 > y2 {
		if x1 > x2 {
		    f(x2,y2,x1,y1,1,1,false)
		} else {
		    f(-x2,y2,-x1,y1,-1,1,false)
		}
	    } else if x1 > x2 {
		f(-x1,y1,-x2,y2,-1,1,false)
	    } else {
		f(x1,y1,x2,y2,1,1,false)
	    }
	} else if x1 > x2 {
	    if y1 > y2 {
		f(y2,x2,y1,x1,1,1,true)
	    } else {
		f(-y2,x2,-y1,x1,-1,1,true)
	    }
	} else if y1 > y2 {
	    f(-y1,x1,-y2,x2,-1,1,true)
	} else {
	    f(y1,x1,y2,x2,1,1,true)
	}
    }
}

impl Iterator for Bresenham {
    type Item = (i32,i32);
    fn next(&mut self)->Option<Self::Item> {
	if self.done {
	    None
	} else {
	    let item =
		if self.swap {
		    Some((self.sb*self.b,self.sa*self.a))
		} else {
		    Some((self.sa*self.a,self.sb*self.b))
		};
	    if self.b == self.b2 {
		self.done = true;
	    }
	    self.b += 1;
	    self.e += self.da;
	    if 2*self.e.abs() > self.db.abs() {
		self.a += 1;
		self.e -= self.db;
	    }
	    item
	}
    }
}
