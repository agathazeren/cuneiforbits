use crate::units::*;
use std::fmt;

pub type Crewed = bool;

pub enum PropellantType {
    Hyrdolox,
    Methalox,
    Keralox,
    Hypergolic,
}

pub enum Fuel {
    Hydrogen,
    Methane,
    RP1,
}

pub enum CryoClass {
    STP,
    Cryo,
    SuperCryo,
}

pub enum ComponentClass {
    Engine(PropellantType, Isp),
    Tank(CryoClass, Preasure),
    Fairing(Volume),
    Capsule(Crewed, Volume, Volume),
}

pub struct Component {
    name: String,
    display: String,
    mass: Mass,
    class: ComponentClass,
    attatch_up: bool,
    attatch_down: bool,
}

pub struct Rocket {
    components: Vec<Component>,
}

impl fmt::Display for Rocket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for component in &self.components {
            write!(f, "{}", component.display)?
        }
        Ok(())
    }
}
