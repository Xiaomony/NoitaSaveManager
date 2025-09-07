use std::{
    collections::BTreeSet,
    sync::{Arc, Mutex, MutexGuard},
};

use super::cmdline_output::CmdlineOutput;
use super::CMDOPT;
use noitarchiver_core::{
    output_manager::OutputManager, throw, Core, NAResult, NAchError, ResultExt,
};
use regex::Regex;

type CallBack<'a> = fn(&CommandParser<'a>, MutexGuard<'_, CmdCore>, Vec<String>) -> NAResult<bool>;
type CmdCore = Core<CmdlineOutput>;

const INCORRECT_FORMAT: &str = "Incorrect command format";

struct Command<'a> {
    cmd_name: &'a [&'a str],
    cmd_explanation: &'a str,
    callback: CallBack<'a>,
}

pub struct CommandParser<'a> {
    commands: Vec<Command<'a>>,
    m_core: Arc<Mutex<CmdCore>>,
}

impl<'a> CommandParser<'a> {
    pub fn new() -> Result<Self, NAchError> {
        let mut new = Self {
            commands: Vec::new(),
            m_core: Arc::new(Mutex::new(Core::new(CMDOPT)?)),
        };

        new.add_command(&["help", "h"], "get help", Self::help);
        new.add_command(&["clear", "cls"], "clear screen", Self::clear);
        new.add_command(&["quit", "q"], "exit the application", Self::quit);
        new.add_command(&["startgame", "sg"], "start the noita", Self::startgame);

        new.add_command(&["save", "s"], "save the archive", Self::save);
        new.add_command(&["qsave", "qs"], "quick save", Self::quick_save);
        new.add_command(&["rsave", "rs"], "replace save", Self::replace_save);
        // new.add_command(&["scheduled_save", "ss"], "schedule save")

        // new.add_command(&["load", "l"], "load ");
        // new.add_command(&["qload", "ql"], "quick load");

        new.add_command(&["log", "lg"], "log", Self::log);
        // new.add_command(&["slog", "sl"], "short log");
        // new.add_command(&["modarch", "ma"], "modify the archive info");

        new.add_command(&["delete", "d"], "delete the archive", Self::delete);
        // new.add_command(&["qdelete", "qd"], "quick delete");

        // new.add_command(&["lock", "lc", "f"], "lock the archive");
        // new.add_command(&["unlock", "ul", "uf"], "unlock the archive");

        // new.add_command(&["usage", "use"], "check the current usage on your disk");
        Ok(new)
    }

    fn add_command(&mut self, name: &'a [&str], explanation: &'a str, p_callback: CallBack<'a>) {
        self.commands.push(Command {
            cmd_name: name,
            cmd_explanation: explanation,
            callback: p_callback,
        });
    }

    pub fn next_command(&mut self) -> NAResult<bool> {
        let core = self.m_core.lock().explain("Fail to get Core lock")?;

        print!(">>>");
        let line = CMDOPT.getline(String::new())?;
        let re = Regex::new(r#""([^"]+)"|(\S+)"#)
            .explain("Fail to initialize regex to parse the command")?;
        let mut parts: Vec<String> = re
            .captures_iter(&line)
            .map(|cap| {
                cap.get(1)
                    .or_else(|| cap.get(2))
                    .unwrap()
                    .as_str()
                    .to_string()
            })
            .collect();

        if parts.is_empty() {
            return throw(INCORRECT_FORMAT);
        }
        let command_str = parts.remove(0);

        if let Some(command) = self.commands.iter().find(|item| {
            item.cmd_name
                .iter()
                .any(|&sub_item| sub_item == command_str)
        }) {
            (command.callback)(self, core, parts)
        } else {
            throw(INCORRECT_FORMAT)
        }
    }

    fn help(&self, mut _core: MutexGuard<'_, CmdCore>, parameter: Vec<String>) -> NAResult<bool> {
        if parameter.is_empty() {
            // TODO:help string
            CMDOPT.log("help sdfadf".to_string());
        } else if let Some(item) = self.commands.iter().find(|item| {
            item.cmd_name
                .iter()
                .any(|&sub_item| sub_item == parameter.first().unwrap())
        }) {
            CMDOPT.log_green(format!("[{}] ({}", item.cmd_name[0], item.cmd_name[1]));
            if item.cmd_name.len() >= 2 {
                for i in 2..item.cmd_name.len() {
                    CMDOPT.log_green(format!(" or {}", item.cmd_name[i]));
                }
            }
            CMDOPT.log_green(")\n\t".to_string() + item.cmd_explanation);
        }
        Ok(true)
    }

    pub fn cls(&self) {
        std::process::Command::new("cmd")
            .args(["/C", "cls"])
            .status()
            .unwrap();
        CMDOPT.log_green(format!("{:=^90}\n", "NoitArchiver"));
        for (index, item) in self.commands.iter().enumerate() {
            print!("{}.{}({})\t", index + 1, item.cmd_name[0], item.cmd_name[1]);
            if index % 3 == 2 {
                println!();
            }
        }
        CMDOPT.log_green(format!("\n{:=^90}\n", ""));
        CmdlineOutput::flush();
    }

    fn clear(&self, mut _core: MutexGuard<'_, CmdCore>, _parameter: Vec<String>) -> NAResult<bool> {
        self.cls();
        Ok(true)
    }

    fn quit(&self, mut _core: MutexGuard<'_, CmdCore>, _parameter: Vec<String>) -> NAResult<bool> {
        Ok(false)
    }

    fn startgame(&self, core: MutexGuard<'_, CmdCore>, _parameter: Vec<String>) -> NAResult<bool> {
        core.startgame()?;
        Ok(true)
    }

    fn save(
        &self,
        mut core: MutexGuard<'_, CmdCore>,
        mut parameter: Vec<String>,
    ) -> NAResult<bool> {
        let name = if parameter.is_empty() {
            CMDOPT.getline("input the archive name(leave blank to cancle):".to_string())?
        } else {
            parameter.remove(0)
        };
        if name.is_empty() {
            return Ok(true);
        }
        let note = if parameter.is_empty() {
            CMDOPT.getline("input the archive note(can be blank):".to_string())?
        } else {
            parameter.remove(0)
        };
        core.save(name, note)?;

        CMDOPT.log_green("Succeed".to_string());
        Ok(true)
    }

    fn quick_save(
        &self,
        mut core: MutexGuard<'_, CmdCore>,
        mut _parameter: Vec<String>,
    ) -> NAResult<bool> {
        core.quick_save()?;
        CMDOPT.log_green("Succeed".to_string());
        Ok(true)
    }

    fn replace_save(
        &self,
        mut core: MutexGuard<'_, CmdCore>,
        mut _parameter: Vec<String>,
    ) -> NAResult<bool> {
        core.replace_save()?;
        CMDOPT.log_green("Succeed".to_string());
        Ok(true)
    }

    fn log(&self, core: MutexGuard<'_, CmdCore>, mut _parameter: Vec<String>) -> NAResult<bool> {
        CMDOPT.log("Locked archives will be displayed in green\n".to_string());
        core.get_arch_infos()
            .archives
            .iter()
            .enumerate()
            .for_each(|(index, item)| {
                let arch_log = format!(
                    "[{}] {}  {}\t{}\t\t\t{}\n",
                    index + 1,
                    item.get_data(),
                    item.get_time(),
                    item.get_name(),
                    item.get_note()
                );
                if item.is_locked() {
                    CMDOPT.log_green(arch_log);
                } else {
                    CMDOPT.log(arch_log);
                }
            });
        Ok(true)
    }

    fn delete(
        &self,
        mut core: MutexGuard<'_, CmdCore>,
        mut parameter: Vec<String>,
    ) -> NAResult<bool> {
        if parameter.is_empty() {
            parameter.push(CMDOPT.getline(
                "Please input the indexes of the archives you want to delete:".to_string(),
            )?);
        }
        let re_single =
            Regex::new(r"\d+").explain("Fail to initialize the regex to parse the command")?;
        let re_range = Regex::new(r"([0-9]+)-([0-9]+)")
            .explain("Fail to initialize the regex to parse the command")?;
        let mut indexes = BTreeSet::new();
        for item in parameter {
            if item.contains('-') {
                indexes.extend(re_range.captures_iter(&item).flat_map(|cap| {
                    cap.get(1).unwrap().as_str().parse::<usize>().unwrap() - 1
                        ..=cap.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1
                }));
            } else {
                indexes.extend(
                    re_single
                        .find_iter(&item)
                        .map(|cap| cap.as_str().parse::<usize>().unwrap() - 1),
                );
            }
        }

        let vec_indexes = indexes.into_iter().collect();
        core.delete_archives(vec_indexes)?;
        Ok(true)
    }
}
