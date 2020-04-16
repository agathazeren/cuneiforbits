mod job;
mod orbit;
mod rocket;
mod sats;
mod ui;

/*
use job::Job;
use job::CustomerRegistry;
use rocket::Rocket;
use sats::SatRegistry;
 */

use ui::UI;
use termion::input::TermRead; 
use termion::raw::IntoRawMode;
use std::mem;
use std::io::Write;

mod units {
    pub struct Mass(u64); //kg
    pub struct Isp(u64); //s
    pub struct Volume(u64); //L
    pub struct Preasure(u64); //Pa
}
fn main() {
    let stdin = std::io::stdin();
    let mut raw = std::io::stdout().into_raw_mode().unwrap();
    
    
    let mut ui = UI::new();

    ui.start();

    
    for event in stdin.events(){
        if !ui.input(event.unwrap()){
            break
        }
    }
    print!("{}{}{}",termion::clear::All,termion::cursor::Goto(1,1),termion::cursor::Show);
    raw.flush();

    drop(raw);
}


/*
static GAME:Game = Game::new();

struct Game{
    sats:SatRegistry,
    customers:CustomerRegistry,
    rocket_designs:Mutex<Vec<Rocket>>,
    availible_jobs:Mutex<Vec<Job>>,            
}

impl Game{
    fn new()->Game{
        Game{
            sats:SatRegistry::new(),
            customers:CustomerRegistry::new(),
            rocket_designs:Mutex::new(Vec::new()),
            availible_jobs:Mutex::new(Vec::new()),
        }
    }
}
*/
