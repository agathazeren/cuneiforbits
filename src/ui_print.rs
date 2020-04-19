use cuneiform_width::cuneiform_width;

#[macro_export]
macro_rules! ui_print {
    ($($arg:tt)*) => (
        {
            use std::fmt::Write;
            crate::ui_print::UIStdout.write_fmt(std::format_args!($($arg)*)).expect("IO failure during ui_print")
        }
    );
}

pub struct UIStdout;

impl std::fmt::Write for UIStdout {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        use std::io::Write;
        std::io::stdout()
            .write_all(proccess(s).as_bytes())
            .expect("IO error in printing");
        Ok(())
    }
}

fn proccess(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        out.push(c);
        let width = cuneiform_width(c);
        if width > 1 {
            out.push_str(&" ".repeat((width - 1) as usize));
        }
    }
    out
}
