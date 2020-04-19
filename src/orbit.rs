use std::fmt;
use std::fmt::Display;

#[derive(Clone, Copy)]
pub struct Orbit; //TODO

impl Display for Orbit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Orbit")
    }
}
