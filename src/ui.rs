use termion::event::Event;
use termion::event::Key;
use std::io::Write;
use termion::cursor;
use std::mem;


pub struct UI{
    current_view:Box<dyn FullView>,
    view_stack:Vec<Box<dyn FullView>>,
    input_mode:InputMode,
}

pub trait FullView{
    fn full_redraw(&self);
    fn update(&mut self,input:Input)->Option<Transition>;
    #[allow(unused_variables)]
    fn restart(&mut self,last:Box<dyn FullView>){
        self.full_redraw();
    }
}

pub enum Input{
    Up,Down,Left,Right,
    Select,
    Back,
}

pub enum Transition{
    Push(Box<dyn FullView>),
    Pop,
}

type Continue = bool;

struct InputMode; // expand this later when needed

impl UI{
    pub fn new()->UI{
        UI{
            current_view:Box::new(basic_tl_view::View::new()),
            view_stack:Vec::new(),
            input_mode:InputMode,
        }
    }

    pub fn start(&mut self){
        print!("{}",cursor::Hide);
        self.current_view.full_redraw();
        
    }

    pub fn input(&mut self,event:Event)->Continue{
        if let Some(input) = self.input_mode.map(event){
            match self.current_view.update(input){
                Some(Transition::Push(mut v)) => {
                    mem::swap(&mut v,&mut self.current_view);
                    self.view_stack.push(v);
                    self.current_view.full_redraw();
                },
                Some(Transition::Pop) => {
                    if let Some(next_view) = self.view_stack.pop(){
                        self.current_view = next_view;
                    } else {
                        return false;
                    }
                    self.current_view.full_redraw();
                },
                None => {}
            }
        }
        true
    }
}

impl InputMode{
    fn map(&self, event:Event)->Option<Input>{
        match event{
            Event::Key(k) => match k{
                Key::Left | Key::Char('a') => Some(Input::Left),
                Key::Right | Key::Char('d') => Some(Input::Right),
                Key::Up | Key::Char('w') => Some(Input::Up),
                Key::Down | Key::Char('s') => Some(Input::Down),
                Key::Char('\n') | Key::Char(' ') => Some(Input::Select),
                Key::Esc => Some(Input::Back),
                _ => None,
            },
            Event::Mouse(me) => match me{
                _ => None,
            },
            Event::Unsupported(_) => None,
        }
    }
}


mod view_prelude{
    pub use super::FullView;
    pub use super::Input;
    pub use super::Transition;
}

mod basic_tl_view{
    use super::view_prelude::*;
    use termion::{clear,cursor};
    use std::convert::TryInto;
    use std::io::stdout;
    use std::io::Write;

    
    pub struct View{
        title:&'static str,
        selection:u8,
        tabs:Vec<Tab>,
    }

    struct Tab{
        name:&'static str,
        transition:Option<Transition>,
    }

    impl FullView for View{
        fn full_redraw(&self){
            print!("{}{}",clear::All,cursor::Goto(1,1));
            print!("{}{}",self.title,cursor::Goto(1,2));
            for (idx,tab) in self.tabs.iter().enumerate(){
                print!("{}",cursor::Goto(1,2+idx as u16));
                if idx as u8  == self.selection{
                    print!("▶");
                } else {
                    print!("{}",cursor::Right(2));
                }
                print!("{}",tab.name);
            }
            stdout().flush();
        }

        fn update(&mut self,input:Input)->Option<Transition>{
            match input{
                Input::Up => {
                    if self.selection == 0 {
                        self.selection = self.max_selection();
                    } else {
                        self.selection -= 1;
                    }
                    self.full_redraw(); //optimize all of these to only change what is neccecary
                    None
                },
                Input::Down => {
                    if self.selection == self.max_selection(){
                       self.selection = 0;
                    } else {
                       self.selection += 1;
                    }
                    self.full_redraw();
                    None
                },
                Input::Select => {
                    Some(self.tabs[usize::from(self.selection)].transition.take().unwrap())
                },
                Input::Back => Some(Transition::Pop),
                _ => None
            }
        }

        fn restart(&mut self, last: Box<dyn FullView>){
            self.tabs[usize::from(self.selection)].transition = Some(Transition::Push(last));
            self.full_redraw();
        }
    }

    impl View{
        pub fn new()->View{
            View{
                title:"Cuneiforbits",
                selection:0,
                tabs:vec![
                    Tab{name:"Missions",transition:Some(Transition::Push(Box::new(super::unimplemented_view::View::new())))},
                    Tab{name:"Jobs",transition:Some(Transition::Push(Box::new(super::unimplemented_view::View::new())))},
                    Tab{name:"Rockets",transition:Some(Transition::Push(Box::new(super::unimplemented_view::View::new())))},
                    Tab{name:"Exit",transition:Some(Transition::Pop)},
                ],
            }
        }

        fn max_selection(&self)->u8{
            (self.tabs.len() - 1).try_into().unwrap()
        }
    }
}

mod unimplemented_view{
    use super::view_prelude::*;
    use std::io::stdout;
    use std::io::Write;
    use termion::{cursor,clear};
    
    pub struct View;

    impl FullView for View{
        fn full_redraw(&self){
            print!("{}{}Unimplemended View. 𒀿|",clear::All,cursor::Goto(1,1));
            eprint!("STDERR");
            stdout().flush();
        }

        fn update(&mut self, input:Input)->Option<Transition>{
            match input{
                Input::Back => Some(Transition::Pop),
                _ => None,
            }
        }
    }

    impl View{
        pub fn new()->View{
            View
        }
    }
}

        

