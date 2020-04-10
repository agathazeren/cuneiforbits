use crate::units::*;
use std::fmt;

type Crewed = bool;

enum PropellantType {
    Hyrdolox,
    Methalox,
    Keralox,
    Hypergolic,
}

enum Fuel {
    Hydrogen,
    Methane,
    RP1,
}

enum CryoClass {
    STP,
    Cryo,
    SuperCryo,
}

enum ComponentClass {
    Engine(PropellantType, Isp),
    Tank(CryoClass, Preasure),
    Fairing(Volume),
    Capsule(Crewed, Volume, Volume),
}

struct Component {
    name: String,
    display: String,
    mass: Mass,
    class: ComponentClass,
    attatch_up: bool,
    attatch_down: bool,
}

struct Rocket {
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
