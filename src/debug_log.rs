use crate::GAME;
use std::fmt::Debug;
use std::io::stdout;
use std::io::Write;
use std::sync::atomic::AtomicU16;
use std::sync::atomic::Ordering;
use std::sync::Mutex;
use termion::{clear, cursor};

pub struct DebugLog {
    lines: Mutex<Vec<String>>,
    info: Mutex<Box<dyn Debug + Send>>, //TODO: Find a good atomic box implementation
    info_scroll: AtomicU16,
}

lazy_static! {
    pub static ref DEBUG: DebugLog = DebugLog::new();
}

impl DebugLog {
    const LINE_LEN: usize = 60;
    const Y_OFFSET: u16 = 15;

    pub fn log(&self, s: &str) {
        if s.len() > DebugLog::LINE_LEN {
            todo!();
        }
        self.lines.lock().unwrap().push(s.to_string());
    }

    pub fn redraw(&self) {
        print!(
            "{}{}",
            cursor::Goto(1, DebugLog::Y_OFFSET),
            clear::AfterCursor
        );
        for (idx, line) in self.lines.lock().unwrap().iter().enumerate() {
            print!(
                "{}{}",
                cursor::Goto(1, DebugLog::Y_OFFSET + idx as u16),
                line
            );
        }

        print!(
            "{}",
            cursor::Goto(DebugLog::LINE_LEN as u16 + 1, DebugLog::Y_OFFSET)
        );

        let mut row = 0;

        let scroll = self.info_scroll.load(Ordering::SeqCst); // We do not want scroll to change out from under us.
        for c in format!("{:#?}", self.info.lock().unwrap()).chars() {
            match c {
                '\n' => {
                    row += 1;
                    if row as i16 - scroll as i16
                        > (termion::terminal_size().unwrap().1 - DebugLog::Y_OFFSET) as i16
                    {
                        break;
                    }
                    if row >= scroll {
                        print!(
                            "{}",
                            cursor::Goto(
                                DebugLog::LINE_LEN as u16 + 1,
                                DebugLog::Y_OFFSET + row - scroll
                            )
                        );
                    }
                }
                _ => {
                    if row >= scroll {
                        print!("{}", c);
                    }
                }
            }
        }

        stdout().flush().unwrap();
    }

    pub fn new() -> DebugLog {
        DebugLog {
            lines: Mutex::new(Vec::new()),
            info: Mutex::new(Box::new("".to_string())),
            info_scroll: AtomicU16::new(0),
        }
    }

    pub fn load_info<T: Debug + Send + 'static>(&self, info: T) {
        *self.info.lock().unwrap() = Box::new(info);
    }

    pub fn on_event(&self, event: &termion::event::Event) {
        use termion::event::Event::*;
        use termion::event::Key::*;
        match event {
            Key(Alt('g')) => {
                self.load_info(&*GAME);
            }
            Key(Alt('C')) => {
                self.load_info(&GAME.known_components);
            }
            Key(Alt('v')) => {
                self.load_info((1..100).collect::<Vec<u8>>());
            }
            Key(Alt('n')) => {
                self.scroll(1);
            }
            Key(Alt('p')) => {
                self.scroll(-1);
            }
            _ => {}
        }
    }

    fn scroll(&self, scroll: i16) {
        let mut old = self.info_scroll.load(Ordering::SeqCst);
        while old
            != self
                .info_scroll
                .compare_and_swap(old, new(old, scroll), Ordering::SeqCst)
        {
            //do something to prevent deadlock
            old = self.info_scroll.load(Ordering::SeqCst);
        }

        fn new(old: u16, scroll: i16) -> u16 {
            let temp = old as i16 + scroll;
            if temp < 0 {
                0
            } else {
                temp as u16
            }
        }
    }
}

#[macro_export]
macro_rules! debug_at {
    () => {
        crate::debug_log::DEBUG.log(&format!("At {} L{}", file!(), line!()));
    };
}
