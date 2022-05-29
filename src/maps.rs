use std::f64::consts::E;

pub trait Map {
    fn domain(&self)->(f64,f64);
    fn codomain(&self)->(f64,f64);
    fn direct(&self,x:f64)->f64;
    fn inverse(&self,x:f64)->f64;
}

pub struct LinearMap {
    x0:f64,
    x1:f64,
    y0:f64,
    y1:f64
}

impl LinearMap {
    pub fn new(x0:f64,x1:f64,y0:f64,y1:f64)->Self {
	Self{ x0,x1,y0,y1 }
    }
}

impl Map for LinearMap {
    fn domain(&self)->(f64,f64) { (self.x0,self.x1) }
    fn codomain(&self)->(f64,f64) { (self.y0,self.y1) }
    fn direct(&self,x:f64)->f64 {
	self.y0 + (self.y1 - self.y0)*(x - self.x0)/(self.x1 - self.x0)
    }
    fn inverse(&self,y:f64)->f64 {
	self.x0 + (self.x1 - self.x0)*(y - self.y0)/(self.y1 - self.y0)
    }
}

pub struct ExponentialMap {
    x0:f64,
    x1:f64,
    y0:f64,
    y1:f64
}

impl ExponentialMap {
    pub fn new(x0:f64,x1:f64,y0:f64,y1:f64)->Self {
	Self{ x0,x1,y0,y1 }
    }

    pub fn u(s:f64)->f64 {
	(s.exp()-1.0)/(E-1.0)
    }

    pub fn v(t:f64)->f64 {
	((E-1.0)*t+1.0).ln()
    }
}

// g(t) : [0,1] -> [0,1]
// t = (exp(s)-1)/(e-1)
// log(t*(e-1)+1) = (exp(s)-1

impl Map for ExponentialMap {
    fn domain(&self)->(f64,f64) { (self.x0,self.x1) }
    fn codomain(&self)->(f64,f64) { (self.y0,self.y1) }
    fn direct(&self,x:f64)->f64 {
	self.y0 + (self.y1 - self.y0)*Self::u((x - self.x0)/(self.x1 - self.x0))
    }
    fn inverse(&self,y:f64)->f64 {
	self.x0 + (self.x1 - self.x0)*Self::v((y - self.y0)/(self.y1 - self.y0))
    }
}
