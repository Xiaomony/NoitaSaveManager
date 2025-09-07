use std::io::{self, Write};

use noitarchiver_core::{output_manager::{self, OutputManager}, NAResult, ResultExt};

use colored::Colorize;

pub struct CmdlineOutput {}

impl CmdlineOutput {
    pub fn flush() {
        io::stdout().flush().unwrap();
    }

    pub fn succeed(&self) {
        self.log_green("Succeed\n".to_string());
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
            .explain_fatal("Fail to get input")?;
        Ok(input.trim().to_string())
    }

    fn confirm(&self, msg: String) -> NAResult<bool> {
        print!("{}", msg.yellow());
        let line = self
            .getline("\nPlease confirm(y/n):".to_string())
            .explain("Fail while confirming")?
            .to_lowercase();
        if line == "yes" || line == "y" {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
