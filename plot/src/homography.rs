use serde::{Serialize,Deserialize};
use beambook::geometry::{point,Point};

#[derive(Copy,Clone,Debug,Serialize,Deserialize)]
pub struct Homography {
    coefs:[[f64;3];3]
}

impl Homography {
    pub fn id()->Self {
	Self{ coefs:[[1.0,0.0,0.0],
		     [0.0,1.0,0.0],
		     [0.0,0.0,1.0]] }
    }

    pub fn rotation(theta:f64)->Self {
	Self{ coefs:[[ theta.cos(),theta.sin(),0.0],
		     [-theta.sin(),theta.cos(),0.0],
		     [         0.0,        0.0,1.0]] }
    }

    pub fn scaling(s:f64)->Self {
	Self{ coefs:[[1.0,0.0,0.0],
		     [0.0,1.0,0.0],
		     [0.0,0.0,1.0/s]] }
    }

    pub fn translation(p:Point)->Self {
	Self{ coefs:[[1.0,0.0,p.x],
		     [0.0,1.0,p.y],
		     [0.0,0.0,1.0]] }
    }

    pub fn apply(&self,p:Point)->Point {
	let [[a,b,c],[d,e,f],[g,h,i]] = self.coefs;
	let Point{ x,y } = p;
	let z = 1.0;
	let xp = a*x+b*y+c*z;
	let yp = d*x+e*y+f*z;
	let zp = g*x+h*y+i*z;
	point(xp/zp,yp/zp)
    }

    pub fn compose(&self,other:Self)->Self {
	let [[a1,b1,c1],[d1,e1,f1],[g1,h1,i1]] = self.coefs;
	let [[a2,b2,c2],[d2,e2,f2],[g2,h2,i2]] = other.coefs;
	Self{ coefs:[[a1*a2+b1*d2+c1*g2,
		      a1*b2+b1*e2+c1*h2,
		      a1*c2+b1*f2+c1*i2],
		     [d1*a2+e1*d2+f1*g2,
		      d1*b2+e1*e2+f1*h2,
		      d1*c2+e1*f2+f1*i2],
		     [g1*a2+h1*d2+i1*g2,
		      g1*b2+h1*e2+i1*h2,
		      g1*c2+h1*f2+i1*i2]] }
    }
}
