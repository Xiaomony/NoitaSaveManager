use std::{
    collections::BTreeSet,
    sync::{Arc, Mutex, MutexGuard},
};

use super::cmdline_output::CmdlineOutput;
use super::CMDOPT;
use colored::Colorize;
use noitarchiver_core::{
    output_manager::OutputManager, throw, Core, NAResult, NAchError, ResultExt,
};
use regex::Regex;

type CallBack<'a> = fn(&CommandParser<'a>, MutexGuard<'_, CmdCore>, Vec<String>) -> NAResult<bool>;
type CmdCore = Core<CmdlineOutput>;

const TITLE: &str = concat!("NoitArhiver  v", env!("CARGO_PKG_VERSION"));

struct Command<'a> {
    cmd_name: &'a [&'a str],
    cmd_explanation: String,
    cmd_manual: String,
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
        rust_i18n::set_locale(
            new.m_core
                .lock()
                .explain(&t!("err.fail_get_core_lock"))?
                .get_locale(),
        );
        // rust_i18n::set_locale("ja-JP");

        // BASIC COMMAND
        new.add_command(&["help", "h"], &t!("exp.help"), &t!("man.help"), Self::help);
        new.add_command(
            &["clear", "cls"],
            &t!("exp.clear"),
            &t!("man.clear"),
            Self::clear,
        );
        new.add_command(
            &["quit", "q", "exit"],
            &t!("exp.quit"),
            &t!("man.quit"),
            Self::quit,
        );
        new.add_command(
            &["startgame", "sg"],
            &t!("exp.startgame"),
            &t!("man.startgame"),
            Self::startgame,
        );
        new.add_command(
            &["setpath", "sp"],
            &t!("exp.setpath"),
            &t!("man.setpath"),
            Self::set_noita_path,
        );

        // SAVE
        new.add_command(&["save", "s"], &t!("exp.save"), &t!("man.save"), Self::save);
        new.add_command(
            &["qsave", "qs"],
            &t!("exp.qsave"),
            &t!("man.qsave"),
            Self::quick_save,
        );
        new.add_command(
            &["overwrite", "ow", "rsave", "rs"],
            &t!("exp.overwrite"),
            &t!("man.overwrite"),
            Self::overwrite,
        );
        new.add_command(
            &["ssave", "ss"],
            &t!("exp.ssave"),
            &t!("man.ssave"),
            Self::scheduled_save,
        );

        // LOAD
        new.add_command(&["load", "l"], &t!("exp.load"), &t!("man.load"), Self::load);
        new.add_command(
            &["qload", "ql"],
            &t!("exp.qload"),
            &t!("man.qload"),
            Self::quick_load,
        );

        // LOG && MODIFY
        new.add_command(
            &["list", "ls", "log", "lg"],
            &t!("exp.list"),
            &t!("man.list"),
            Self::log,
        );
        new.add_command(
            &["slist", "sl", "slog"],
            &t!("exp.slist"),
            &t!("man.slist"),
            Self::short_log,
        );
        new.add_command(
            &["modarch", "ma"],
            &t!("exp.modarch"),
            &t!("man.modarch"),
            Self::modify_archive,
        );

        // DELETE
        new.add_command(
            &["delete", "d", "del"],
            &t!("exp.delete"),
            &t!("man.delete"),
            Self::delete,
        );
        new.add_command(
            &["qdelete", "qd", "qdel"],
            &t!("exp.qdelete"),
            &t!("man.qdelete"),
            Self::quick_delete,
        );

        // LOCK
        new.add_command(
            &["lock", "lc", "f"],
            &t!("exp.lock"),
            &t!("man.lock"),
            Self::lock,
        );
        new.add_command(
            &["unlock", "ul", "uf"],
            &t!("exp.unlock"),
            &t!("man.unlock"),
            Self::unlock,
        );

        // OTHER
        new.add_command(
            &["usage", "use"],
            &t!("exp.usage"),
            &t!("man.usage"),
            Self::usage,
        );
        Ok(new)
    }

    fn add_command<T: AsRef<str> + std::fmt::Display>(
        &mut self,
        name: &'a [&str],
        explanation: &T,
        manual: &T,
        p_callback: CallBack<'a>,
    ) {
        self.commands.push(Command {
            cmd_name: name,
            cmd_explanation: explanation.as_ref().to_string(),
            cmd_manual: manual.as_ref().to_string(),
            callback: p_callback,
        });
    }

    pub fn next_command(&mut self) -> NAResult<bool> {
        let core = self.m_core.lock().explain(&t!("err.fail_get_core_lock"))?;

        // print ">>>" seperately to avoid it to be printed in blue
        print!(">>>");
        let line = CMDOPT.getline(String::new())?;
        let re = Regex::new(r#""([^"]*)"|(\S+)"#).explain(&t!("err.fail_init_regex"))?;
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
            return throw(&t!("warn.incorrect_cmd_format"));
        }
        let command_str = parts.remove(0);

        if let Some(command) = self.commands.iter().find(|item| {
            item.cmd_name
                .iter()
                .any(|&sub_item| sub_item == command_str)
        }) {
            (command.callback)(self, core, parts)
        } else {
            throw(&t!("warn.incorrect_cmd_format"))
        }
    }

    fn help(&self, mut _core: MutexGuard<'_, CmdCore>, parameter: Vec<String>) -> NAResult<bool> {
        if parameter.is_empty() {
            CMDOPT.log(t!("instruction").to_string());
        } else if let Some(item) = self.commands.iter().find(|item| {
            item.cmd_name
                .iter()
                .any(|&sub_item| sub_item == parameter.first().unwrap())
        }) {
            CMDOPT.log_green(format!(
                "[{}]\t\t{}: {}",
                item.cmd_name[0],
                t!("msg.aliases"),
                item.cmd_name[1]
            ));
            if item.cmd_name.len() >= 2 {
                for i in 2..item.cmd_name.len() {
                    CMDOPT.log_green(format!(" or {}", item.cmd_name[i]));
                }
            }
            CMDOPT.log_green(format!("{: >20}\n", item.cmd_explanation));
            CMDOPT.log_green(item.cmd_manual.clone());
        }
        Ok(true)
    }

    pub fn cls(&self) {
        std::process::Command::new("cmd")
            .args(["/C", "cls"])
            .status()
            .unwrap();
        CMDOPT.log_green(format!("{:=^120}\n", TITLE));
        for (index, item) in self.commands.iter().enumerate() {
            print!(
                "{: <34}",
                format!(
                    "{}.{}({})",
                    index + 1,
                    item.cmd_name[0].cyan(),
                    item.cmd_name[1].bright_yellow()
                )
            );
            super::cmdline_output::print_with_pad(&item.cmd_explanation, 22);
            match index + 1 {
                3 | 8 | 11 | 14 | 16 => println!(),
                5 | 9 | 18 => println!("\n"),
                _ => (),
            }
        }
        CMDOPT.log_green(format!("\n{:=^120}\n", ""));
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

    fn set_noita_path(
        &self,
        mut core: MutexGuard<'_, CmdCore>,
        mut parameter: Vec<String>,
    ) -> NAResult<bool> {
        core.set_noita_path(if parameter.is_empty() {
            let path = CMDOPT.input(t!("prompt.noita_path").to_string())?;
            if path.is_empty() {
                CMDOPT.cancel();
                return Ok(true);
            } else {
                path
            }
        } else {
            parameter.remove(0)
        })?;
        Ok(true)
    }

    fn save(
        &self,
        mut core: MutexGuard<'_, CmdCore>,
        mut parameter: Vec<String>,
    ) -> NAResult<bool> {
        let name = if parameter.is_empty() {
            CMDOPT.input(t!("prompt.save_name").to_string())?
        } else {
            parameter.remove(0)
        };
        if name.is_empty() {
            CMDOPT.cancel();
            return Ok(true);
        }
        let note = if parameter.is_empty() {
            CMDOPT.input(t!("prompt.save_note").to_string())?
        } else {
            parameter.remove(0)
        };
        core.save(name, note)?;

        CMDOPT.succeed();
        Ok(true)
    }

    fn quick_save(
        &self,
        mut core: MutexGuard<'_, CmdCore>,
        _parameter: Vec<String>,
    ) -> NAResult<bool> {
        core.quick_save()?;
        CMDOPT.succeed();
        Ok(true)
    }

    fn overwrite(
        &self,
        mut core: MutexGuard<'_, CmdCore>,
        _parameter: Vec<String>,
    ) -> NAResult<bool> {
        core.overwrite_save()?;
        CMDOPT.succeed();
        Ok(true)
    }

    // TODO: scheduled_save
    fn scheduled_save(
        &self,
        _core: MutexGuard<'_, CmdCore>,
        _parameter: Vec<String>,
    ) -> NAResult<bool> {
        Ok(true)
    }

    fn load(&self, core: MutexGuard<'_, CmdCore>, mut parameter: Vec<String>) -> NAResult<bool> {
        if parameter.is_empty() {
            parameter.push(CMDOPT.input(t!("prompt.load_index").to_string())?);
        }
        if let Ok(index) = parameter.first().unwrap().as_str().parse::<usize>() {
            if index <= core.get_arch_infos().archives.len() {
                core.load_archive(index - 1)?;
            } else {
                return throw(&t!("warn.invalid_index"));
            }
        } else {
            CMDOPT.cancel();
            return Ok(true);
        }
        CMDOPT.succeed();
        Ok(true)
    }

    fn quick_load(&self, core: MutexGuard<'_, CmdCore>, _parameter: Vec<String>) -> NAResult<bool> {
        core.quick_load()?;
        CMDOPT.succeed();
        Ok(true)
    }

    fn print_log(&self, core: MutexGuard<'_, CmdCore>, start: usize) {
        CMDOPT.log(t!("msg.locked_archive_in_green").to_string() + "\n");
        core.get_arch_infos().archives[start..]
            .iter()
            .enumerate()
            .for_each(|(index, item)| {
                let arch_log = format!(
                    "[{}] {}  {}\t{}\t\t\t{}\n",
                    index + 1 + start,
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
    }
    fn log(&self, core: MutexGuard<'_, CmdCore>, _parameter: Vec<String>) -> NAResult<bool> {
        self.print_log(core, 0);
        Ok(true)
    }

    fn short_log(&self, core: MutexGuard<'_, CmdCore>, _parameter: Vec<String>) -> NAResult<bool> {
        let start = std::cmp::max(core.get_arch_infos().archives.len() as isize - 6, 0) as usize;
        self.print_log(core, start);
        Ok(true)
    }

    fn modify_archive(
        &self,
        mut core: MutexGuard<'_, CmdCore>,
        mut parameter: Vec<String>,
    ) -> NAResult<bool> {
        let index_str = if parameter.is_empty() {
            CMDOPT.input(t!("prompt.modarch_index").to_string())?
        } else {
            parameter.remove(0)
        };
        if index_str.is_empty() {
            CMDOPT.cancel();
            return Ok(true);
        }

        let Ok(index) = index_str.parse::<usize>() else {
            return throw(&t!("warn.incorrect_cmd_format"));
        };

        let new_name = if parameter.is_empty() {
            let temp = CMDOPT.input(t!("prompt.modarch_name").to_string())?;
            if temp.is_empty() {
                None
            } else {
                Some(temp)
            }
        } else {
            Some(parameter.remove(0))
        };
        let new_note = if parameter.is_empty() {
            let temp = CMDOPT.input(t!("prompt.modarch_note").to_string())?;
            if temp.is_empty() {
                None
            } else {
                Some(temp)
            }
        } else {
            Some(parameter.remove(0))
        };

        core.modify_arch_info(index - 1, new_name, new_note)?;

        CMDOPT.succeed();
        Ok(true)
    }

    fn get_indexes_by_parameter(parameter: Vec<String>) -> NAResult<Vec<usize>> {
        let re_single = Regex::new(r"\d+").explain(&t!("err.fail_init_regex"))?;
        let re_range = Regex::new(r"([0-9]+)-([0-9]+)").explain(&t!("err.fail_init_regex"))?;
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

        Ok(indexes.into_iter().collect())
    }

    fn delete(
        &self,
        mut core: MutexGuard<'_, CmdCore>,
        mut parameter: Vec<String>,
    ) -> NAResult<bool> {
        if parameter.is_empty() {
            parameter.push(CMDOPT.input(t!("prompt.delete_index").to_string())?);
        }
        let indexes = Self::get_indexes_by_parameter(parameter)?;
        if indexes.is_empty() {
            CMDOPT.cancel();
            return Ok(true);
        }
        core.delete_archives(indexes)?;
        Ok(true)
    }

    fn quick_delete(
        &self,
        mut core: MutexGuard<'_, CmdCore>,
        _parameter: Vec<String>,
    ) -> NAResult<bool> {
        core.quick_delete_archive()?;
        Ok(true)
    }

    fn lock(&self, mut core: MutexGuard<'_, CmdCore>, parameter: Vec<String>) -> NAResult<bool> {
        let indexes = Self::get_indexes_by_parameter(if parameter.is_empty() {
            CMDOPT
                .input(t!("prompt.lock_index").to_string())?
                .split(' ')
                .map(|item| item.to_string())
                .collect()
        } else {
            parameter
        })?;
        if indexes.is_empty() {
            CMDOPT.cancel();
            return Ok(true);
        }
        core.lock(indexes)?;
        Ok(true)
    }

    fn unlock(&self, mut core: MutexGuard<'_, CmdCore>, parameter: Vec<String>) -> NAResult<bool> {
        let indexes = Self::get_indexes_by_parameter(if parameter.is_empty() {
            CMDOPT
                .input(t!("prompt.unlock_index").to_string())?
                .split(' ')
                .map(|item| item.to_string())
                .collect()
        } else {
            parameter
        })?;
        if indexes.is_empty() {
            CMDOPT.cancel();
            return Ok(true);
        }
        core.unlock(indexes)?;
        Ok(true)
    }

    fn usage(&self, _core: MutexGuard<'_, CmdCore>, _parameter: Vec<String>) -> NAResult<bool> {
        let usage = Core::<CmdlineOutput>::usage_by_mb()?;
        if usage > 1024.0 {
            CMDOPT.log(format!("{:.2} GB\n", usage / 1024.0));
        } else {
            CMDOPT.log(format!("{:.2} MB\n", usage));
        }
        Ok(true)
    }
}
