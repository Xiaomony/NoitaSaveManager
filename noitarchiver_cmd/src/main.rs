#[macro_use]
extern crate rust_i18n;
i18n!(
    "locales",
    fallback = ["en-US", "en-GB", "zh-CN", "zh-TW", "ja-JP"]
);

mod cmdline_output;
mod command_parser;

use std::error::Error;

use cmdline_output::CmdlineOutput;
use command_parser::CommandParser;
use noitarchiver_core::{output_manager::OutputManager, NAComResult};

const CMDOPT: CmdlineOutput = CmdlineOutput {};

fn process() -> NAComResult {
    let mut parser = CommandParser::new()?;
    parser.cls();
    loop {
        match parser.next_command() {
            Err(e) => {
                if e.is_fatal() {
                    return Err(e);
                } else if let Some(msg) = e.get_explanation().first() {
                    CMDOPT.warning(msg.to_string());
                    println!();
                }
            }
            Ok(false) => break,
            Ok(true) => {}
        }
    }
    Ok(())
}

fn main() {
    if let Err(e) = process() {
        CMDOPT.fatal_error(format!("{} :\n", e));
        e.get_explanation().iter().rev().for_each(|item| {
            CMDOPT.fatal_error(format!("\t-{}\n", item));
        });
        CMDOPT.debug("Trace Back:\n".to_string());
        let mut source = e.source();
        while let Some(err) = source {
            source = err.source();
            CMDOPT.debug(format!("\t{}\n", err));
        }
        println!("\n\n");
        CMDOPT
            .getline(t!("msg.press_enter_to_exit").to_string())
            .unwrap();
    }
}
