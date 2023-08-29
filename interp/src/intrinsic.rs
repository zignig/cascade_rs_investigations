// Poor attempt to mess with macros
extern crate interp_derive;

use interp_derive::{Cover,ingest};

#[derive(Debug,Cover)]
pub struct Cylinder {
    pub height: f64,
    pub radius: f64,
}

impl Cylinder { 
    pub fn generate(){

    }
}

#[derive(Cover)]
pub struct Cube {
    pub size: f64,
}

impl Cube { 
    pub fn generate(){

    }
}

//#[ingest]
mod stuff { 
    pub fn test(){}
}