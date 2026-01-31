use std::{
    collections::BTreeSet,
    sync::{Arc, Condvar, Mutex},
    thread::{self, JoinHandle},
    time::Duration,
};

use super::CMDOPT;
use super::cmdline_output::*;
use colored::Colorize;
use noita_save_manager_core::{
    Core, NSError, NSResult, ResultExt, output_manager::OutputManager, throw,
};
use regex::Regex;
use rustyline::ExternalPrinter;

type CallBack<'a> = fn(&CommandParser<'a>, &mut CmdCore, Vec<String>) -> NSResult<bool>;
type CmdCore = Core<CmdlineOutput>;

const TITLE: &str = concat!("NoitaSaveManager  v", env!("CARGO_PKG_VERSION"));

struct Command<'a> {
    cmd_name: &'a [&'a str],
    cmd_explanation: String,
    cmd_manual: String,
    callback: CallBack<'a>,
}
struct SsaveKit {
    m_ssave_thread: Option<JoinHandle<()>>,
    m_auto_save_flag: Arc<Mutex<bool>>,
    m_condvar: Arc<Condvar>,
    m_rustyline_reader: rustyline::DefaultEditor,
}

pub struct CommandParser<'a> {
    commands: Vec<Command<'a>>,
    m_core: Arc<Mutex<CmdCore>>,
    m_ssave_kit: Arc<Mutex<SsaveKit>>,
}

impl<'a> CommandParser<'a> {
    pub fn new() -> Result<Self, NSError> {
        let mut new = Self {
            commands: Vec::new(),
            m_core: Arc::new(Mutex::new(Core::new(CMDOPT)?)),
            m_ssave_kit: Arc::new(Mutex::new(SsaveKit {
                m_ssave_thread: None,
                m_auto_save_flag: Arc::new(Mutex::new(false)),
                m_condvar: Arc::new(Condvar::new()),
                m_rustyline_reader: rustyline::DefaultEditor::new().unwrap(),
            })),
        };
        rust_i18n::set_locale(
            new.m_core
                .lock()
                .explain(&t!("err.fail_get_mutex_lock"))?
                .get_locale(),
        );

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
        #[cfg(target_family = "windows")]
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
            &["asave", "as"],
            &t!("exp.asave"),
            &t!("man.asave"),
            Self::auto_save,
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
            &["modify", "mo"],
            &t!("exp.modify"),
            &t!("man.modify"),
            Self::modify_save,
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

    pub fn next_command(&mut self) -> NSResult<bool> {
        let line = self
            .m_ssave_kit
            .lock()
            .unwrap()
            .m_rustyline_reader
            .readline(">>>")
            .unwrap();
        let core = &mut *self.m_core.lock().explain(&t!("err.fail_get_mutex_lock"))?;
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

    fn help(&self, _core: &mut CmdCore, parameter: Vec<String>) -> NSResult<bool> {
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
                    CMDOPT.log_green(format!(" / {}", item.cmd_name[i]));
                }
            }
            CMDOPT.log_green(format!("{: >20}\n", item.cmd_explanation));
            CMDOPT.log_green(item.cmd_manual.clone());
        }
        Ok(true)
    }

    pub fn cls(&self) {
        #[cfg(target_family = "unix")]
        std::process::Command::new("clear").status().unwrap();
        #[cfg(target_family = "windows")]
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
            print_with_pad(&item.cmd_explanation, 22);
            #[cfg(target_family = "windows")]
            match index + 1 {
                3 | 8 | 11 | 14 | 16 => println!(),
                5 | 9 | 18 => println!("\n"),
                _ => (),
            }
            #[cfg(target_family = "unix")]
            match index + 1 {
                3 | 7 | 10 | 13 | 15 => println!(),
                4 | 8 | 17 => println!("\n"),
                _ => (),
            }
        }
        CMDOPT.log_green(format!("\n{:=^120}\n", ""));
        CmdlineOutput::flush();
    }

    fn clear(&self, _core: &mut CmdCore, _parameter: Vec<String>) -> NSResult<bool> {
        self.cls();
        Ok(true)
    }

    fn quit(&self, _core: &mut CmdCore, _parameter: Vec<String>) -> NSResult<bool> {
        Ok(false)
    }

    fn startgame(&self, core: &mut CmdCore, _parameter: Vec<String>) -> NSResult<bool> {
        core.startgame()?;
        Ok(true)
    }

    #[cfg(target_family = "windows")]
    fn set_noita_path(&self, core: &mut CmdCore, mut parameter: Vec<String>) -> NSResult<bool> {
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

    fn save(&self, core: &mut CmdCore, mut parameter: Vec<String>) -> NSResult<bool> {
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

    fn quick_save(&self, core: &mut CmdCore, _parameter: Vec<String>) -> NSResult<bool> {
        core.quick_save(false)?;
        CMDOPT.succeed();
        Ok(true)
    }

    fn overwrite(&self, core: &mut CmdCore, _parameter: Vec<String>) -> NSResult<bool> {
        core.overwrite_save()?;
        CMDOPT.succeed();
        Ok(true)
    }

    fn auto_save(&self, _core: &mut CmdCore, mut parameter: Vec<String>) -> NSResult<bool> {
        let Ok(mut time_interval) = (if parameter.is_empty() {
            CMDOPT
                .input(t!("prompt.auto_save_interval").to_string())?
                .parse::<u64>()
        } else {
            parameter.remove(0).parse::<u64>()
        }) else {
            CMDOPT.cancel();
            return Ok(true);
        };

        let Ok(max_auto_saves) = (if parameter.is_empty() {
            CMDOPT
                .input(t!("prompt.auto_save_max_saves").to_string())?
                .parse::<usize>()
        } else {
            parameter.remove(0).parse::<usize>()
        }) else {
            CMDOPT.cancel();
            return Ok(true);
        };
        if max_auto_saves == 0 {
            CMDOPT.cancel();
            return Ok(true);
        }

        time_interval *= 60;

        // core functionality of auto_save
        let mut kit = self
            .m_ssave_kit
            .lock()
            .explain(&t!("err.fail_get_mutex_lock"))
            .unwrap();
        if let Some(previous_handle) = kit.m_ssave_thread.take() {
            let mut flag = kit
                .m_auto_save_flag
                .lock()
                .explain(&t!("err.fail_get_mutex_lock"))
                .unwrap();
            *flag = false;
            kit.m_condvar.notify_all();
            drop(flag);
            previous_handle.join().unwrap();
        };
        if time_interval != 0 {
            let core_ref = self.m_core.clone();
            let ref_condvar = kit.m_condvar.clone();
            let ref_flag = kit.m_auto_save_flag.clone();
            *kit.m_auto_save_flag
                .lock()
                .explain(&t!("err.fail_get_mutex_lock"))
                .unwrap() = true;
            let mut printer = kit.m_rustyline_reader.create_external_printer().unwrap();

            let handle = thread::spawn(move || {
                loop {
                    let flag = ref_flag
                        .lock()
                        .explain(&t!("err.fail_get_mutex_lock"))
                        .unwrap();
                    let result = ref_condvar
                        .wait_timeout(flag, Duration::from_secs(time_interval))
                        .explain(&t!("err.fail_get_mutex_lock"))
                        .unwrap();
                    if !*result.0 {
                        return;
                    }

                    let mut core = core_ref
                        .lock()
                        .explain(&t!("err.fail_get_mutex_lock"))
                        .unwrap();
                    printer
                        .print(
                            format_with_pad_centered(&t!("msg.auto_saving"), 69)
                                .bright_yellow()
                                .bold()
                                .to_string(),
                        )
                        .unwrap();
                    let (removed, latest) = core.auto_save(max_auto_saves).unwrap();
                    if let Some(removed_save) = removed {
                        printer
                            .print(
                                format!("{}\n\t{}", t!("msg.auto_save_delete_old"), removed_save)
                                    .cyan()
                                    .to_string(),
                            )
                            .unwrap();
                    }
                    printer
                        .print(
                            format!("{}\n\t{}", t!("msg.auto_save_new"), latest)
                                .cyan()
                                .to_string(),
                        )
                        .unwrap();
                    printer
                        .print(
                            format_with_pad_centered(&t!("msg.success"), 69)
                                .green()
                                .to_string(),
                        )
                        .unwrap();
                }
            });
            kit.m_ssave_thread = Some(handle);
        } else {
            CMDOPT.log_green(t!("msg.auto_save_stop").to_string() + "\n");
        }
        Ok(true)
    }

    fn load(&self, core: &mut CmdCore, mut parameter: Vec<String>) -> NSResult<bool> {
        if parameter.is_empty() {
            parameter.push(CMDOPT.input(t!("prompt.load_index").to_string())?);
        }
        if let Ok(index) = parameter.first().unwrap().as_str().parse::<usize>() {
            if index <= core.get_save_infos().saves.len() {
                core.load_save(index - 1)?;
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

    fn quick_load(&self, core: &mut CmdCore, _parameter: Vec<String>) -> NSResult<bool> {
        core.quick_load()?;
        CMDOPT.succeed();
        Ok(true)
    }

    fn print_log(&self, core: &mut CmdCore, start: usize) {
        if core.get_save_infos().saves.is_empty() {
            CMDOPT.log(t!("msg.no_save").to_string() + "\n");
            return;
        }
        CMDOPT.log(t!("msg.locked_save_in_green").to_string() + "\n");
        core.get_save_infos().saves[start..]
            .iter()
            .enumerate()
            .for_each(|(index, item)| {
                let save_log = format!("[{}] {}\n", index + 1 + start, item);
                if item.is_locked() {
                    CMDOPT.log_green(save_log);
                } else {
                    CMDOPT.log(save_log);
                }
            });
    }
    fn log(&self, core: &mut CmdCore, _parameter: Vec<String>) -> NSResult<bool> {
        self.print_log(core, 0);
        Ok(true)
    }

    fn short_log(&self, core: &mut CmdCore, _parameter: Vec<String>) -> NSResult<bool> {
        let start = std::cmp::max(core.get_save_infos().saves.len() as isize - 6, 0) as usize;
        self.print_log(core, start);
        Ok(true)
    }

    fn modify_save(&self, core: &mut CmdCore, mut parameter: Vec<String>) -> NSResult<bool> {
        let index_str = if parameter.is_empty() {
            CMDOPT.input(t!("prompt.modify_index").to_string())?
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
            let temp = CMDOPT.input(t!("prompt.modify_name").to_string())?;
            if temp.is_empty() { None } else { Some(temp) }
        } else {
            Some(parameter.remove(0))
        };
        let new_note = if parameter.is_empty() {
            let temp = CMDOPT.input(t!("prompt.modify_note").to_string())?;
            if temp.is_empty() { None } else { Some(temp) }
        } else {
            Some(parameter.remove(0))
        };

        core.modify_save_info(index - 1, new_name, new_note)?;

        CMDOPT.succeed();
        Ok(true)
    }

    fn get_indexes_by_parameter(parameter: Vec<String>) -> NSResult<Vec<usize>> {
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

    fn delete(&self, core: &mut CmdCore, mut parameter: Vec<String>) -> NSResult<bool> {
        if parameter.is_empty() {
            parameter.push(CMDOPT.input(t!("prompt.delete_index").to_string())?);
        }
        let indexes = Self::get_indexes_by_parameter(parameter)?;
        if indexes.is_empty() {
            CMDOPT.cancel();
            return Ok(true);
        }
        core.delete_saves(indexes)?;
        Ok(true)
    }

    fn quick_delete(&self, core: &mut CmdCore, _parameter: Vec<String>) -> NSResult<bool> {
        core.quick_delete_save()?;
        Ok(true)
    }

    fn lock(&self, core: &mut CmdCore, parameter: Vec<String>) -> NSResult<bool> {
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

    fn unlock(&self, core: &mut CmdCore, parameter: Vec<String>) -> NSResult<bool> {
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

    fn usage(&self, _core: &mut CmdCore, _parameter: Vec<String>) -> NSResult<bool> {
        let usage = Core::<CmdlineOutput>::usage_by_mb()?;
        if usage > 1024.0 {
            CMDOPT.log(format!("{:.2} GB\n", usage / 1024.0));
        } else {
            CMDOPT.log(format!("{:.2} MB\n", usage));
        }
        Ok(true)
    }
}
