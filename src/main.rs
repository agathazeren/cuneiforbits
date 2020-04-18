mod job;
mod orbit;
mod rocket;
mod sats;
mod ui;
#[macro_use]
mod ui_print;


#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate static_assertions;

use job::CustomerRegistry;
use job::Job;
use rocket::Rocket;
use sats::SatRegistry;

use std::io::Write;
use std::mem;
use std::sync::Mutex;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use ui::UI;

mod units {
    pub struct Mass(u64); //g
    pub struct Isp(u64); //s
    pub struct Volume(u64); //L
    pub struct Preasure(u64); //Pa

    impl Mass {
        pub fn kg(kg: u64) -> Mass {
            Mass(1000 * kg)
        }

        pub fn as_kg(&self) -> f64 {
            let Mass(g) = self;
            *g as f64/1000.0
        }
    }
}
fn main() {
    let stdin = std::io::stdin();
    let mut raw = std::io::stdout().into_raw_mode().unwrap();

    let mut ui = UI::new();
    ui.start();

    for event in stdin.events() {
        if !ui.input(event.unwrap()) {
            break;
        }
    }

    ui_print!("foo");

    drop(ui); //ui should be dropped before the terminal exits raw mode
    drop(raw);
}

lazy_static! {
    pub static ref GAME: Game = Game::new();
}

pub struct Game {
    sats: SatRegistry,
    customers: CustomerRegistry,
    rocket_designs: Mutex<Vec<Rocket>>,
    available_jobs: Mutex<Vec<Job>>,
    accepted_jobs: Mutex<Vec<Job>>,
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
