use crate::orbit::Orbit;
use crate::units::*;

pub struct SatId(u32);

struct SatRegistry {
    sats: Vec<Sat>, //Should this be indexmap?
}

enum Sat {
    CubeSat(CubeSat),
    LargeSat(LargeSat),
    ArraySat(ArraySat),
    Station(Station),
}

pub struct CubeSat {
    class: CubeSatClass,
    mass: Mass,
    orbit: Orbit,
}

enum CubeSatClass {
    CubeSat1U,
    CubeSat2U,
    CubeSat3U,
    CubeSat6U,
}

pub struct LargeSat {
    volume: Volume,
    mass: Mass,
    orbit: Orbit,
}

pub struct SatArray {
    volume: Volume,
    base_mass: Mass,
    sat_mass: Mass,
    orbits: Vec<Orbit>,
}

pub struct ArraySat {
    mass: Mass,
    orbit: Orbit,
}

struct Station {
    name: String,
    orbit: Orbit,
}
