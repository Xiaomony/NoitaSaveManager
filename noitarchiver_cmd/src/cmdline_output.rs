use std::io::{self, Write};

use noitarchiver_core::{
    NAResult, ResultExt,
    output_manager::{self, OutputManager},
};

use colored::Colorize;

// this function is for auto_save
pub fn format_with_pad_centered(s: &str, width: usize) -> String {
    let w = unicode_width::UnicodeWidthStr::width(s);

    if w >= width {
        s.to_string()
    } else {
        let n = width - w;
        format!("{}{s}{}", "-".repeat(n / 2), "-".repeat(n / 2 + n % 2))
    }
}

pub fn print_with_pad(s: &str, width: usize) {
    let w = unicode_width::UnicodeWidthStr::width(s);
    if w >= width {
        print!("{s}");
    } else {
        print!("{s}{}", " ".repeat(width - w));
    }
}

pub struct CmdlineOutput {}

impl CmdlineOutput {
    pub fn flush() {
        io::stdout().flush().unwrap();
    }

    pub fn succeed(&self) {
        self.log_green(t!("msg.success").to_string() + "\n");
    }

    pub fn cancel(&self) {
        self.log(t!("msg.cancel").to_string() + "\n");
    }

    // input string after printing "prompt + :"
    pub fn input(&self, mut msg: String) -> NAResult<String> {
        msg.push(':');
        self.getline(msg)
    }
}

impl output_manager::OutputManager for CmdlineOutput {
    fn fatal_error(&self, msg: String) {
        print!("{}", format!("[FATAL] {msg}").red().bold());
        Self::flush();
    }

    fn warning(&self, msg: String) {
        print!("{}", format!("[WARNING] {msg}").bright_yellow().bold());
        Self::flush();
    }

    fn log(&self, msg: String) {
        print!("{}", msg.cyan());
        Self::flush();
    }

    fn log_green(&self, msg: String) {
        print!("{}", msg.green());
        Self::flush();
    }

    fn debug(&self, msg: String) {
        print!("{}", format!("[DEBUG] {msg}").purple());
        Self::flush();
    }

    fn getline(&self, input_msg: String) -> NAResult<String> {
        self.log(input_msg);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .explain_fatal(&t!("err.fail_to_input"))?;
        Ok(input.trim().to_string())
    }

    fn confirm(&self, msg: String) -> NAResult<bool> {
        print!("{}", msg.yellow());
        let line = self
            .getline(format!("\n{}(y/n):", t!("msg.confirm")))
            .explain(&t!("msg.fail_confirm"))?
            .to_lowercase();
        if line == "yes" || line == "y" {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
