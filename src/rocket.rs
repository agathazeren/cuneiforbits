use crate::units::*;
use std::fmt;

pub type Crewed = bool;

#[derive(Clone, Copy)]
pub enum PropellantType {
    Hyrdolox,
    Methalox,
    Keralox,
    Hypergolic,
}

#[derive(Clone, Copy)]
pub enum Fuel {
    Hydrogen,
    Methane,
    RP1,
}

#[derive(Clone, Copy)]
pub enum CryoClass {
    STP,
    Cryo,
    SuperCryo,
}

#[derive(Clone, Copy)]
pub enum ComponentClass {
    Engine(PropellantType, Isp),
    Tank(CryoClass, Preasure),
    Fairing(Volume),
    Capsule(Crewed, Volume, Volume),
}

#[derive(Clone)]
pub struct Component {
    pub name: String,
    pub display: String,
    pub mass: Mass,
    pub class: ComponentClass,
}

#[derive(Clone)]
pub struct Rocket {
    pub name: String,
    pub components: Vec<Component>,
}

impl fmt::Display for Rocket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for component in &self.components {
            write!(f, "{}", component.display)?
        }
        Ok(())
    }
}

impl fmt::Display for Component {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.display)
    }
}

impl Rocket {
    pub fn new() -> Rocket {
        Rocket {
            name: "New Rocket".to_string(),
            components: Vec::new(),
        }
    }
}

impl ComponentClass {
    pub fn symbol(&self) -> String {
        //TODO pick nice unicode sybols for these
        match self {
            ComponentClass::Engine(_, _) => "E",
            ComponentClass::Tank(_, _) => "T",
            ComponentClass::Fairing(_) => "F",
            ComponentClass::Capsule(_, _, _) => "C",
        }
        .to_string()
    }
}

impl Component {
    pub const MAX_WIDTH: u16 = 5; //picked out of thin air
}

lazy_static! {
    pub static ref INITIAL_KNOWN_COMPONENTS: Vec<Component> = vec![
        Component {
            name: "Foo".to_string(),
            display: "Foo".to_string(),
            mass: Mass::kg(12),
            class: ComponentClass::Engine(PropellantType::Hyrdolox, Isp::s(3)),
        },
        Component {
            name: "Bar".to_string(),
            display: "Foo".to_string(),
            mass: Mass::kg(12),
            class: ComponentClass::Engine(PropellantType::Hyrdolox, Isp::s(3)),
        },
        Component {
            name: "Baz".to_string(),
            display: "Foo".to_string(),
            mass: Mass::kg(12),
            class: ComponentClass::Engine(PropellantType::Hyrdolox, Isp::s(3)),
        },
        Component {
            name: "Quux".to_string(),
            display: "Foo".to_string(),
            mass: Mass::kg(12),
            class: ComponentClass::Engine(PropellantType::Hyrdolox, Isp::s(3)),
        },
    ];
}
