mod job;
mod orbit;
mod rocket;
mod sats;
mod ui;

#[macro_use]
mod debug_log;

#[macro_use]
mod ui_print;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate static_assertions;

use debug_log::DEBUG;
use job::CustomerRegistry;
use job::Job;
use rocket::Component;
use rocket::Rocket;
use sats::SatRegistry;
use std::sync::Mutex;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use ui::UI;

mod units {
    #[derive(Clone, Copy, Debug)]
    /// A mass, represented as an integer number of grams.
    pub struct Mass(u64);
    /// A specific impulse, represented as an integer number of seconds.
    #[derive(Clone, Copy, Debug)]
    pub struct Isp(u64);
    /// A volume, represented as an integer number of liters.
    #[derive(Clone, Copy, Debug)]
    pub struct Volume(u64);
    /// A preasure, represented as an integer number of pascals.
    #[derive(Clone, Copy, Debug)]
    pub struct Preasure(u64);

    impl Mass {
        pub fn kg(kg: u64) -> Mass {
            Mass(1000 * kg)
        }

        pub fn in_kg(self) -> f64 {
            let Mass(g) = self;
            g as f64 / 1000.0
        }
    }

    impl Isp {
        pub fn s(s: u64) -> Isp {
            Isp(s)
        }
    }

    impl Volume {
        pub fn in_m3(self) -> f64 {
            let Volume(l) = self;
            l as f64 * 1000.0
        }
    }
}
fn main() {
    let stdin = std::io::stdin();
    let raw = std::io::stdout().into_raw_mode().unwrap();

    let mut ui = UI::new();
    ui.start();

    for event in stdin.events() {
        let event = event.unwrap();
        if !ui.input(&event) {
            break;
        }
        DEBUG.on_event(&event);
        DEBUG.redraw();
    }

    drop(ui); //ui should be dropped before the terminal exits raw mode
    drop(raw);
}

lazy_static! {
    pub static ref GAME: Game = Game::new();
}

#[derive(Debug)]
pub struct Game {
    sats: SatRegistry,
    customers: CustomerRegistry,
    rocket_designs: Mutex<Vec<Rocket>>,
    available_jobs: Mutex<Vec<Job>>,
    accepted_jobs: Mutex<Vec<Job>>,
    known_components: Mutex<Vec<Component>>,
}

const TARGET_JOBS: usize = 3;

impl Game {
    fn new() -> Game {
        Game {
            sats: SatRegistry::new(),
            customers: CustomerRegistry::new(),
            rocket_designs: Mutex::new(Vec::new()),
            available_jobs: Mutex::new(Vec::new()),
            accepted_jobs: Mutex::new(Vec::new()),
            known_components: Mutex::new(rocket::INITIAL_KNOWN_COMPONENTS.to_vec()),
        }
    }

    fn tick(&self) {
        let mut jobs = self.available_jobs.lock().unwrap();
        if jobs.len() < TARGET_JOBS {
            jobs.push(Job::generate());
        }
        drop(jobs);
    }

    fn accept_job_at(&self, idx: usize) {
        let mut available = self.available_jobs.lock().unwrap();
        let job = available.remove(idx);
        drop(available);
        let mut accepted = self.accepted_jobs.lock().unwrap();
        accepted.push(job);
    }

    fn decline_job_at(&self, idx: usize) {
        let _ = self.available_jobs.lock().unwrap().remove(idx);
    }
}

assert_impl_all!(Game: Sync);
