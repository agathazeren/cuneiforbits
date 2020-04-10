use crate::sats::{CubeSat, LargeSat, SatArray, SatId};
use crate::units::*;
use rand::{seq::SliceRandom, thread_rng, Rng};

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
    target_customers: u8,
}

struct Customer {
    name: String,
}

impl Job {
    fn generate(&mut self, customers: &mut CustomerRegistry) -> Job {
        Job {
            customer: customers.get_or_generate(),
            payload: Payload::generate(),
        }
    }
}

impl CustomerRegistry {
    fn get_or_generate(&mut self) -> CustomerId {
        let idx = thread_rng().gen_range(0, self.target_customers);
        if usize::from(idx) >= self.customers.len() {
            self.customers.push(Customer::generate());
            return CustomerId((self.customers.len() - 1) as u32);
        }
        CustomerId(idx as u32)
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
        unimplemented!()
    }
}
