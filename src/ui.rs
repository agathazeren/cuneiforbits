use std::io::stdout;
use std::io::Write;
use std::mem;
use termion::cursor;
use termion::event::Event;
use termion::event::Key;
#[macro_use]
use crate::ui_print;




pub struct UI {
    current_view: Box<dyn FullView>,
    view_stack: Vec<Box<dyn FullView>>,
    input_mode: InputMode,
}

pub trait FullView {
    fn full_redraw(&self);
    fn update(&mut self, input: Input) -> Option<Transition>;
    #[allow(unused_variables)]
    fn restart(&mut self, last: Box<dyn FullView>) -> Option<Transition> {
        self.full_redraw();
        None
    }
    fn start(&mut self) -> Option<Transition> {
        self.full_redraw();
        None
    }
}

pub enum Input {
    Up,
    Down,
    Left,
    Right,
    Select,
    Back,
}

pub enum Transition {
    Push(Box<dyn FullView>),
    Pop,
}

type Continue = bool;

struct InputMode; // expand this later when needed

impl UI {
    pub fn new() -> UI {
        UI {
            current_view: Box::new(basic_tl_view::View::new()),
            view_stack: Vec::new(),
            input_mode: InputMode,
        }
    }

    pub fn start(&mut self) {
        ui_print!("{}", cursor::Hide);
        self.current_view.full_redraw();
    }

    pub fn input(&mut self, event: Event) -> Continue {
        if let Some(input) = self.input_mode.map(event) {
            let trans = self.current_view.update(input);
            self.handle_trans(trans)
        } else {
            true
        }
    }

    fn handle_trans(&mut self, transition: Option<Transition>) -> Continue {
        match transition {
            Some(Transition::Push(mut v)) => {
                mem::swap(&mut v, &mut self.current_view);
                self.view_stack.push(v);
                let start_trans = self.current_view.start();
                if !self.handle_trans(start_trans) {
                    return false;
                }
            }
            Some(Transition::Pop) => {
                if let Some(mut v) = self.view_stack.pop() {
                    mem::swap(&mut v, &mut self.current_view);
                    self.current_view.restart(v);
                    self.current_view.full_redraw();
                } else {
                    return false;
                }
            }
            None => {}
        }
        true
    }
}

impl Drop for UI {
    fn drop(&mut self) {
        print!(
            "{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            termion::cursor::Show
        );
        stdout().flush().unwrap();
    }
}

impl InputMode {
    fn map(&self, event: Event) -> Option<Input> {
        match event {
            Event::Key(k) => match k {
                Key::Left | Key::Char('a') => Some(Input::Left),
                Key::Right | Key::Char('d') => Some(Input::Right),
                Key::Up | Key::Char('w') => Some(Input::Up),
                Key::Down | Key::Char('s') => Some(Input::Down),
                Key::Char('\n') | Key::Char(' ') => Some(Input::Select),
                Key::Esc => Some(Input::Back),
                _ => None,
            },
            Event::Mouse(me) => match me {
                _ => None,
            },
            Event::Unsupported(_) => None,
        }
    }
}

mod view_prelude {
    pub use super::FullView;
    pub use super::Input;
    pub use super::Transition;
    pub use crate::GAME;
}

mod basic_tl_view {
    use super::view_prelude::*;
    use std::convert::TryInto;
    use std::io::stdout;
    use std::io::Write;
    use termion::{clear, cursor};
    #[macro_use] use crate::ui_print;
    
    pub struct View {
        title: &'static str,
        selection: u8,
        tabs: Vec<Tab>,
    }

    struct Tab {
        name: &'static str,
        transition: Option<Transition>,
    }

    impl FullView for View {
        fn full_redraw(&self) {
            ui_print!("{}{}", clear::All, cursor::Goto(1, 1));
            ui_print!("{}{}", self.title, cursor::Goto(1, 2));
            for (idx, tab) in self.tabs.iter().enumerate() {
                print!("{}", cursor::Goto(1, 2 + idx as u16));
                if idx as u8 == self.selection {
                    ui_print!("▶");
                } else {
                    ui_print!("{}", cursor::Right(2));
                }
                ui_print!("{}", tab.name);
            }
            stdout().flush().unwrap();
        }

        fn update(&mut self, input: Input) -> Option<Transition> {
            match input {
                Input::Up => {
                    if self.selection == 0 {
                        self.selection = self.max_selection();
                    } else {
                        self.selection -= 1;
                    }
                    self.full_redraw(); //optimize all of these to only change what is neccecary
                    None
                }
                Input::Down => {
                    if self.selection == self.max_selection() {
                        self.selection = 0;
                    } else {
                        self.selection += 1;
                    }
                    self.full_redraw();
                    None
                }
                Input::Select => Some(
                    self.tabs[usize::from(self.selection)]
                        .transition
                        .take()
                        .unwrap(),
                ),
                Input::Back => Some(Transition::Pop),
                _ => None,
            }
        }

        fn restart(&mut self, last: Box<dyn FullView>) -> Option<Transition>{
            self.tabs[usize::from(self.selection)].transition = Some(Transition::Push(last));
            self.full_redraw();
            None
        }
    }

    impl View {
        pub fn new() -> View {
            View {
                title: "Cuneiforbits",
                selection: 0,
                tabs: vec![
                    Tab {
                        name: "Missions",
                        transition: Some(Transition::Push(Box::new(
                            super::unimplemented_view::View::new(),
                        ))),
                    },
                    Tab {
                        name: "Jobs",
                        transition: Some(Transition::Push(Box::new(super::jobs_view::View::new()))),
                    },
                    Tab {
                        name: "Rockets",
                        transition: Some(Transition::Push(Box::new(
                            super::rockets_view::View::new()
                        ))),
                    },
                    Tab {
                        name: "Tick",
                        transition: Some(Transition::Push(Box::new(super::tick_view::View::new()))),
                    },
                    Tab {
                        name: "Exit",
                        transition: Some(Transition::Pop),
                    },
                ],
            }
        }

        fn max_selection(&self) -> u8 {
            (self.tabs.len() - 1).try_into().unwrap()
        }
    }
}

mod unimplemented_view {
    use super::view_prelude::*;
    use std::io::stdout;
    use std::io::Write;
    use termion::{clear, cursor};
    #[macro_use] use crate::ui_print;

    pub struct View;

    impl FullView for View {
        fn full_redraw(&self) {
            ui_print!("{}{}Unimplemended View. 𒀿|", clear::All, cursor::Goto(1, 1));
            eprint!("STDERR");
            stdout().flush().unwrap();
        }

        fn update(&mut self, input: Input) -> Option<Transition> {
            match input {
                Input::Back => Some(Transition::Pop),
                _ => None,
            }
        }
    }

    impl View {
        pub fn new() -> View {
            View
        }
    }
}

mod jobs_view {
    use super::view_prelude::*;
    use std::convert::TryFrom;
    use std::convert::TryInto;
    use std::io::stdout;
    use std::io::Write;
    use termion::{clear, cursor};
    use std::cell::Cell;


    pub struct View {
        vert_sel: u8,
        horiz_sel: HorizSel,
        no_jobs: Cell<bool>,
    }

    enum HorizSel {
        Name,
        Accept,
        Decline,
    }
    use HorizSel::*;

    impl FullView for View {
        fn full_redraw(&self) {
            const MAX_CUSTOMER_NAME_LEN: u16 = 20;

            print!("{}{}", clear::All, cursor::Goto(1, 1));
            print!("Jobs{}", cursor::Goto(1, 2));
            let jobs = GAME.available_jobs.lock().unwrap();
            for (idx, job) in jobs.iter().enumerate() {
                let row = (3 + idx * 2) as u16;
                print!(
                    "{}{}",
                    cursor::Goto(3, row),
                    GAME.customers.on(job.customer, |c| c.name.clone()).unwrap()
                );
                print!("{}{}", cursor::Goto(5, row + 1), job.payload);
                print!(
                    "{}✓{}X",
                    cursor::Goto(3 + MAX_CUSTOMER_NAME_LEN + 2, row),
                    cursor::Right(3)
                );
            }
            self.no_jobs.set(jobs.len() == 0);
            drop(jobs);
            if !self.no_jobs.get(){
                let x: u16 = match self.horiz_sel {
                    Name => 1,
                    Accept => 3 + MAX_CUSTOMER_NAME_LEN + 1,
                    Decline => 3 + MAX_CUSTOMER_NAME_LEN + 5,
                };
                let symbol = match self.horiz_sel {
                    Name => "▶".to_string(),
                    Accept | Decline => format!("[{}]", cursor::Right(1)),
                };
                print!(
                    "{}{}",
                    cursor::Goto(x, (3 + self.vert_sel * 2).into()),
                    symbol
                );
            }
            stdout().flush().unwrap();
        }

        fn update(&mut self, input: Input) -> Option<Transition> {
            match input {
                Input::Back => Some(Transition::Pop),
                Input::Up => {
                    if !self.no_jobs.get(){
                        if self.vert_sel == 0 {
                            self.vert_sel = u8::try_from(GAME.available_jobs.lock().unwrap().len()).unwrap() - 1;
                        } else {
                            self.vert_sel -= 1;
                        }
                    }
                    self.full_redraw();
                    None
                }
                Input::Down => {
                    if !self.no_jobs.get(){
                        let max_idx = u8::try_from(GAME.available_jobs.lock().unwrap().len()).unwrap() - 1;
                        if self.vert_sel == max_idx {
                            self.vert_sel = 0;
                        } else {
                            self.vert_sel += 1;
                        }
                    };
                    self.full_redraw();
                    None
                }
                Input::Left => {
                    self.horiz_sel = match self.horiz_sel {
                        Name | Accept => Name,
                        Decline => Accept,
                    };
                    self.full_redraw();
                    None
                }
                Input::Right => {
                    self.horiz_sel = match self.horiz_sel {
                        Name => Accept,
                        Accept | Decline => Decline,
                    };
                    self.full_redraw();
                    None
                }
                Input::Select => {
                    match self.horiz_sel {
                        Name => {}
                        Accept => {
                            if !self.no_jobs.get() {
                                GAME.accept_job_at(self.vert_sel.try_into().unwrap())
                            }
                            if self.vert_sel >= u8::try_from(GAME.available_jobs.lock().unwrap().len()).unwrap() && self.vert_sel > 0{
                                self.vert_sel -= 1;
                            }
                            self.full_redraw();
                        }
                        Decline => {
                            if !self.no_jobs.get() {
                                GAME.decline_job_at(self.vert_sel.try_into().unwrap())
                            }
                            if self.vert_sel >= u8::try_from(GAME.available_jobs.lock().unwrap().len()).unwrap() && self.vert_sel > 0{
                                self.vert_sel -= 1;
                            }
                            self.full_redraw();
                        }
                    }
                    None
                }
            }
        }
    }

    impl View {
        pub fn new() -> View {
            View {
                vert_sel: 0,
                horiz_sel: Name,
                no_jobs: Cell::new(GAME.available_jobs.lock().unwrap().len() == 0),
            }
        }
    }
}

mod tick_view {
    use super::view_prelude::*;
    use crate::GAME;
    use std::io::stdout;
    use std::io::Write;
    use termion::clear;

    pub struct View;

    impl FullView for View {
        fn full_redraw(&self) {
            print!("{}", clear::All);
            stdout().flush().unwrap();
        }
        fn update(&mut self, _: Input) -> Option<Transition> {
            unreachable!()
        }
        fn start(&mut self) -> Option<Transition> {
            GAME.tick();
            Some(Transition::Pop)
        }
    }

    impl View {
        pub fn new() -> View {
            View
        }
    }
}

mod rockets_view{
    use super::view_prelude::*;
    #[macro_use] use crate::ui_print;
    use termion::{clear,cursor};
    use std::io::stdout;
    use std::io::Write;

    pub struct View{
        sel:Sel,
    }

    enum Sel{
        New,
        Rocket(u8),
        RocketEdit(u8),
    }

    impl FullView for View{
        fn full_redraw(&self){
            print!("{}{}",clear::All,cursor::Goto(1,1));

            print!("Rockets  (+)");

            let rockets = GAME.rocket_designs.lock().unwrap();
            
            for (idx,rocket) in rockets.iter().enumerate(){
                print!("{}",cursor::Goto(2,(2 + 2*idx) as u16));
                ui_print!("{}: {}",rocket.name,rocket);
            }

            drop(rockets);

            const EDIT_BUTTON_X:u16 = 40;

            match self.sel{
                Sel::New => print!("{}{{{}}}",cursor::Goto(10,1),cursor::Right(1)),
                Sel::Rocket(idx) => print!("{}>{}(edit)",cursor::Goto(1,(2+2*idx) as u16),cursor::Right(EDIT_BUTTON_X-1)),
                Sel::RocketEdit(idx) => print!("{}[edit]",cursor::Goto(EDIT_BUTTON_X,(2+2*idx) as u16)),
            }

            stdout().flush().unwrap();
        }

        fn update(&mut self,input:Input)->Option<Transition>{
            match input{
                Input::Back => Some(Transition::Pop),
                Input::Up => {
                    let rocket_cnt = (GAME.rocket_designs.lock().unwrap().len()) as u8;
                    if rocket_cnt == 0 {
                        self.sel = Sel::New;
                        return None;
                    }
                    let max_idx = rocket_cnt - 1;
                    match self.sel{
                        Sel::New => self.sel = Sel::Rocket(max_idx),
                        Sel::Rocket(idx) | Sel::RocketEdit(idx) => self.sel = if idx == 0 {Sel::New} else {Sel::Rocket(idx - 1)},
                    }
                    self.full_redraw();
                    None
                },
                Input::Down => {
                    let rocket_cnt = (GAME.rocket_designs.lock().unwrap().len()) as u8;
                    if rocket_cnt == 0 {
                        self.sel = Sel::New;
                        return None;
                    }
                    let max_idx = rocket_cnt - 1;
                    match self.sel{
                        Sel::New => self.sel = Sel::Rocket(0),
                        Sel::Rocket(idx) | Sel::RocketEdit(idx) => self.sel = if idx == max_idx {Sel::New} else {Sel::Rocket(idx + 1)},
                    }
                    self.full_redraw();
                    None
                },
                Input::Left | Input::Right => {
                    self.sel = match self.sel{
                        Sel::New => Sel::New,
                        Sel::Rocket(idx) => Sel::RocketEdit(idx),
                        Sel::RocketEdit(idx) => Sel::Rocket(idx),
                    };
                    self.full_redraw();
                    None
                },
                Input::Select => {
                    match self.sel{
                        Sel::New => Some(Transition::Push(Box::new(super::unimplemented_view::View::new()))),
                        Sel::RocketEdit(idx) => Some(Transition::Push(Box::new(super::unimplemented_view::View::new()))),
                        Sel::Rocket(_) => None,
                    }
                },
            }
        }

        fn start(&mut self)->Option<Transition>{
            self.check_idx();
            self.full_redraw();
            None
        }

        fn restart(&mut self, _: Box<dyn FullView>)->Option<Transition>{
            self.check_idx();
            self.full_redraw();
            None
        }

    }


    impl View{
        pub fn new()->View{
            View{
                sel:Sel::Rocket(0)
            }
        }

        fn check_idx(&mut self){
            let rocket_cnt = GAME.rocket_designs.lock().unwrap().len() as u8;
            if rocket_cnt == 0 {
                self.sel = Sel::New;
                return
            }
            let max_idx = rocket_cnt - 1;
            match self.sel{
                Sel::Rocket(idx) => if idx > max_idx {
                    self.sel = Sel::Rocket(idx - 1)
                },
                Sel::RocketEdit(idx) => if idx > max_idx {
                    self.sel = Sel::RocketEdit(idx - 1)
                },
                Sel::New => {},
            }
        }
    }
}
                
