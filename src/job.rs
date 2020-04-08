use crate::sats::{CubeSat, LargeSat, SatArray, SatId};
use crate::units::*;

struct Job {
    customer: CustomerId,
    payload: Payload,
}

enum Payload {
    CubeSat(CubeSat),
    LargeSat(LargeSat),
    SatArray(SatArray),
    Station(SatId, Cargo),
}

struct Cargo {
    volume: Volume,
    mass: Mass,
}

struct CustomerId(u32);

struct CustomerRegistry {
    customers: Vec<Customer>,
}

struct Customer {
    name: String,
}
