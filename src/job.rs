#![allow(dead_code)]




use crate::orbit::Orbit;
use crate::sats::Sat;
use crate::sats::{CubeSat, CubeSatClass, LargeSat, SatArray, SatId};
use crate::units::*;
use crate::GAME;
use rand::{seq::SliceRandom, thread_rng, Rng};
use std::fmt;
use std::fmt::Display;
use std::sync::atomic::AtomicU8;
use std::sync::atomic::Ordering;
use std::sync::Mutex;

#[derive(Debug)]
pub struct Job {
    pub customer: CustomerId,
    pub payload: Payload,
}

#[derive(Debug)]
pub enum Payload {
    CubeSat(CubeSat),
    LargeSat(LargeSat),
    SatArray(SatArray),
    Station(SatId, Cargo),
}

#[derive(Debug)]
pub struct Cargo {
    volume: Volume,
    mass: Mass,
}

#[derive(Clone, Copy, Debug)]
pub struct CustomerId(u32);

#[derive(Debug)]
pub struct CustomerRegistry {
    customers: Mutex<Vec<Customer>>,
    target_customers: AtomicU8,
}

#[derive(Debug)]
pub struct Customer {
    pub name: String,
}

impl Job {
    pub fn generate() -> Job {
        Job {
            customer: GAME.customers.get_or_generate(),
            payload: Payload::generate(),
        }
    }
}

const TARGET_CUSTOMERS: u8 = 5;

impl CustomerRegistry {
    pub fn new() -> CustomerRegistry {
        CustomerRegistry {
            customers: Mutex::new(Vec::new()),
            target_customers: AtomicU8::new(TARGET_CUSTOMERS),
        }
    }

    fn get_or_generate(&self) -> CustomerId {
        let idx = thread_rng().gen_range(0, self.target_customers.load(Ordering::Relaxed));
        let mut customers = self.customers.lock().unwrap();
        if idx as usize >= customers.len() {
            customers.push(Customer::generate());
            return CustomerId((customers.len() - 1) as u32);
        }
        drop(customers);
        CustomerId(idx as u32)
    }

    pub fn on<T, F: FnOnce(&Customer) -> T>(&self, CustomerId(idx): CustomerId, f: F) -> Option<T> {
        let customers = self.customers.lock().unwrap();
        if let Some(customer) = customers.get(idx as usize) {
            Some(f(customer))
        } else {
            None
        }
    }
}

impl Customer {
    fn generate() -> Customer {
        Customer {
            name: Customer::generate_name(),
        }
    }

    fn generate_name() -> String {
        const PREFIXES: [&str; 4] = ["Space", "Rocket", "Sat", "Next"];

        const SUFFIXES: [&str; 5] = ["X", "Lab", "Corp", "Co", "Inc"];
        //TODO: Don't hardcode these

        format!(
            "{}{}{}",
            PREFIXES.choose(&mut thread_rng()).unwrap(),
            if thread_rng().gen::<bool>() { " " } else { "" },
            SUFFIXES.choose(&mut thread_rng()).unwrap()
        )
        //Unwraps on `choose` will not panic as the arrays are not empty
    }
}

impl Payload {
    fn generate() -> Payload {
        //TODO: actually generate these
        Payload::CubeSat(CubeSat {
            class: CubeSatClass::CubeSat1U,
            mass: Mass::kg(1),
            orbit: Orbit,
        })
    }
}

impl Display for Payload {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CubeSat(sat) => write!(
                f,
                "{} CubeSat of {} kg to {}",
                sat.class,
                sat.mass.in_kg(),
                sat.orbit
            ),
            Self::LargeSat(sat) => write!(
                f,
                "{} kg {} m³ Satalite to {}",
                sat.mass.in_kg(),
                sat.volume.in_m3(),
                sat.orbit
            ),
            Self::SatArray(sats) => write!(
                f,
                "Array of {} Satalites of total {} kg and {} m³",
                sats.orbits.len(),
                sats.base_mass.in_kg() + sats.sat_mass.in_kg() * sats.orbits.len() as f64,
                sats.volume.in_m3()
            ),
            Self::Station(sat_id, cargo) => write!(
                f,
                "Delivery to {} in {} of {} kg, {} m³",
                if let Sat::Station(sta) = GAME.sats.get(*sat_id).unwrap() {
                    &sta.name
                } else {
                    "a satalite"
                },
                GAME.sats.get(*sat_id).unwrap().orbit(),
                cargo.mass.in_kg(),
                cargo.volume.in_m3()
            ),
        }
        .unwrap();
        Ok(())
    }
}
