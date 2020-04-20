#![allow(dead_code)] //temp

use crate::orbit::Orbit;
use crate::units::*;
use std::fmt;
use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
pub struct SatId(u32);

#[derive(Debug)]
pub struct SatRegistry {
    sats: Vec<Sat>, //Should this be indexmap?
}

#[derive(Debug)]
#[allow(clippy::enum_variant_names)] //Should we change this as clippy suggests? I am torn.
pub enum Sat {
    CubeSat(CubeSat),
    LargeSat(LargeSat),
    ArraySat(ArraySat),
    Station(Station),
}

#[derive(Debug)]
pub struct CubeSat {
    pub class: CubeSatClass,
    pub mass: Mass,
    pub orbit: Orbit,
}

#[derive(Debug)]
pub enum CubeSatClass {
    CubeSat1U,
    CubeSat2U,
    CubeSat3U,
    CubeSat6U,
}

#[derive(Debug)]
pub struct LargeSat {
    pub volume: Volume,
    pub mass: Mass,
    pub orbit: Orbit,
}

#[derive(Debug)]
pub struct SatArray {
    pub volume: Volume,
    pub base_mass: Mass,
    pub sat_mass: Mass,
    pub orbits: Vec<Orbit>,
}

#[derive(Debug)]
pub struct ArraySat {
    mass: Mass,
    orbit: Orbit,
}

#[derive(Debug)]
pub struct Station {
    pub name: String,
    pub orbit: Orbit,
}

impl SatRegistry {
    pub fn new() -> SatRegistry {
        SatRegistry { sats: Vec::new() }
    }

    pub fn get(&self, id: SatId) -> Option<&Sat> {
        let SatId(idx) = id;
        self.sats.get(idx as usize)
    }
}

impl Sat {
    pub fn orbit(&self) -> Orbit {
        match self {
            Sat::CubeSat(sat) => sat.orbit,
            Sat::LargeSat(sat) => sat.orbit,
            Sat::ArraySat(sat) => sat.orbit,
            Sat::Station(sat) => sat.orbit,
        }
    }
}

impl Display for CubeSatClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::CubeSat1U => "1U",
                Self::CubeSat2U => "2U",
                Self::CubeSat3U => "3U",
                Self::CubeSat6U => "6U",
            }
        )
    }
}
