use std::fmt::Display;
use std::fmt;



pub struct Orbit; //TODO



impl Display for Orbit{
    fn fmt(&self, f:&mut fmt::Formatter)->fmt::Result{
        write!(f,"Orbit")
    }

}
