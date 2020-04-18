use crate::sats::{CubeSat, LargeSat, SatArray, SatId, CubeSatClass};
use crate::units::*;
use rand::{seq::SliceRandom, thread_rng, Rng};
use std::fmt::Display;
use std::fmt;
use crate::GAME;
use std::sync::Mutex;
use std::sync::atomic::AtomicU8;
use std::sync::atomic::Ordering;
use crate::orbit::Orbit;


pub struct Job {
    pub customer: CustomerId,
    pub payload: Payload,
}

pub enum Payload {
    CubeSat(CubeSat),
    LargeSat(LargeSat),
    SatArray(SatArray),
    Station(SatId, Cargo),
}

pub struct Cargo {
    volume: Volume,
    mass: Mass,
}

#[derive(Clone,Copy)]
pub struct CustomerId(u32);

pub struct CustomerRegistry {
    customers: Mutex<Vec<Customer>>,
    target_customers: AtomicU8,
}

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


const TARGET_CUSTOMERS:u8 = 5;

impl CustomerRegistry {
    pub fn new()->CustomerRegistry{
        CustomerRegistry{
            customers:Mutex::new(Vec::new()),
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

    pub fn on<T,F:FnOnce(&Customer)->T>(&self,CustomerId(idx):CustomerId,f:F)->Option<T>{
        let customers = self.customers.lock().unwrap();
        if let Some(customer) = customers.get(idx as usize){
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
        //temp for testing
        Payload::CubeSat(CubeSat{class:CubeSatClass::CubeSat1U,mass:Mass::kg(1),orbit:Orbit})
    }

   
        
}

impl Display for Payload{
    fn fmt(&self, f:&mut fmt::Formatter<'_>)-> fmt::Result{
        match self{
            Self::CubeSat(sat) => write!(f,"{} CubeSat of {} kg to {}",sat.class,sat.mass.as_kg(),sat.orbit),
            Self::LargeSat(sat) => write!(f,"Large Sat"),
            Self::SatArray(sat) => write!(f,"Sat Array"),
            Self::Station(sat,cargo) => write!(f,"Delivery"),
        }.unwrap();
        Ok(())
    }
}
        


   
