use termion::{cursor,clear};
use std::sync::Mutex;
use std::io::stdout;
use std::io::Write;
use std::fmt::Debug;
use std::sync::atomic::AtomicU16;
use crate::GAME;
use std::sync::atomic::Ordering;


pub struct DebugLog{
    lines:Mutex<Vec<String>>,
    info: Mutex<Box<dyn Debug + Send>>,//TODO: Find a good atomic box implementation
    info_scroll: AtomicU16, 
}

lazy_static!{
    pub static ref DEBUG: DebugLog = DebugLog::new();
}


impl DebugLog{
    const LINE_LEN:usize = 80;
    const Y_OFFSET:u16 = 25;

    
    pub fn log(&self,s:&str){
        if s.len() > DebugLog::LINE_LEN {
            todo!();
        }
        self.lines.lock().unwrap().push(s.to_string());
    }

    pub fn redraw(&self){
        print!("{}{}",cursor::Goto(1,DebugLog::Y_OFFSET), clear::AfterCursor);
        for (idx,line) in self.lines.lock().unwrap().iter().enumerate() {
            print!("{}{}",cursor::Goto(1,DebugLog::Y_OFFSET + idx as u16),line);
        }

        print!("{}",cursor::Goto(DebugLog::LINE_LEN as u16 + 1,DebugLog::Y_OFFSET));

        let mut row = 0;

        let scroll = self.info_scroll.load(Ordering::SeqCst); // We do not want scroll to change out from under us. 
        for c in format!("{:#?}",self.info.lock().unwrap()).chars(){
            match c {
                '\n' => {
                    row += 1; 
                    print!("{}",cursor::Goto(DebugLog::LINE_LEN as u16 + 1 , DebugLog::Y_OFFSET + row - scroll));
                }
                _ => {
                    if row >= scroll {
                        print!("{}",c);
                    }
                        
                }
            }
        }
        
        stdout().flush().unwrap();
    }

    pub fn new()->DebugLog{
        DebugLog{
            lines:Mutex::new(Vec::new()),
            info:Mutex::new(Box::new("".to_string())),
            info_scroll: AtomicU16::new(0),
        }
    }

    pub fn load_info<T:Debug + Send + 'static>(&self,info:T){
        *self.info.lock().unwrap() = Box::new(info);
    }

    pub fn on_event(&self, event:&termion::event::Event){
        use termion::event::Event::*;
        use termion::event::Key::*;
        match event{
            Key(Alt('g')) => {
                self.load_info(&*GAME);
            }
            Key(Alt('n')) => {
                let _old = self.info_scroll.fetch_add(1,Ordering::SeqCst);
            }
            Key(Alt('p')) => {
                let _old = self.info_scroll.fetch_sub(1,Ordering::SeqCst);
            }
            _ => {}
        }
    }
}


#[macro_export]
macro_rules! debug_at{
    () => {
        crate::debug_log::DEBUG.log(&format!("At {} L{}",file!(),line!()));
    }
}


