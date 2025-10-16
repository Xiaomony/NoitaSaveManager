mod utils;

pub use utils::error::*;
use utils::file_operator::{FileOperator, SAVE_FOLDER_PATH};
pub use utils::output_manager;
use utils::output_manager::OutputManager;
pub use utils::save_infos::{AllInfos, SingleSave};

// third-party imports
use chrono::{Datelike, Local, Timelike};
use sys_locale::get_locale;

#[macro_use]
extern crate rust_i18n;
i18n!("locales");

// std imports
use std::os::windows::process::CommandExt;
use std::path::Path;
use std::process::Command;

#[derive(Debug)]
pub struct Core<Opm: OutputManager> {
    m_file_operator: FileOperator,
    m_info: AllInfos,
    m_opm: Opm,
    m_locale: &'static str,
}

impl<Opm: OutputManager> Core<Opm> {
    pub fn new(opm: Opm) -> NSResult<Self> {
        let map_language = |local: &str| -> &str {
            match local {
                "zh-CN" | "zh-SG" | "zh-Hans" => "zh-CN",
                "zh-TW" | "zh-HK" | "zh-MO" | "zh-Hant" => "zh-TW",
                "en-GB" | "en-AU" | "en-NZ" | "en-IN" | "en-ZA" | "en-HK" | "en-SG" | "en-IE"
                | "en-PK" | "en-MT" | "en-MY" | "en-NG" => "en-GB",
                "ja-JP" => "ja-JP",
                _ => "en-US",
            }
        };
        let locale = match get_locale() {
            Some(l) => map_language(&l),
            None => "en-US",
        };
        rust_i18n::set_locale(locale);
        let file_operator = FileOperator::new()?;
        Ok(Self {
            m_info: file_operator.load_infos()?,
            m_file_operator: file_operator,
            m_opm: opm,
            m_locale: locale,
        })
    }

    #[inline]
    fn get_date() -> String {
        Local::now().format(&t!("date_format")).to_string()
    }

    #[inline]
    fn get_time() -> String {
        Local::now().format(&t!("time_format")).to_string()
    }

    #[inline]
    pub fn get_save_infos(&self) -> &AllInfos {
        &self.m_info
    }

    #[inline]
    pub fn write_infos(&mut self) -> NSComResult {
        self.m_file_operator.write_infos(&self.m_info)
    }

    #[inline]
    pub fn get_locale(&self) -> &str {
        self.m_locale
    }

    pub fn startgame(&self) -> NSComResult {
        let noipath = self.m_info.get_exe_path();
        if !(noipath.exists() && noipath.ends_with("noita.exe")) {
            return throw(&t!("please_set_noita_path"));
        } else {
            self.m_opm
                .warning(t!("start_without_steam_warning").to_string() + "\n");
            Command::new(noipath)
                .creation_flags(0x00000008) // run noita in detached process
                .current_dir(noipath.parent().unwrap())
                .spawn()?;
        }
        Ok(())
    }

    pub fn set_noita_path(&mut self, path: String) -> NSComResult {
        self.m_info.set_noita_path(path);
        self.write_infos()
    }

    pub fn save(&mut self, save_name: String, save_note: String) -> NSComResult {
        if self
            .m_info
            .saves
            .iter()
            .any(|item| item.get_name() == save_name)
        {
            return throw(&t!("change_save_name"));
        }

        if save_name.is_empty() {
            return throw(&t!("save_name_empty"));
        }

        self.m_file_operator.save(&save_name)?;
        self.m_info.saves.push(SingleSave::new(
            Self::get_date(),
            Self::get_time(),
            save_name,
            save_note,
        ));
        self.write_infos()?;
        Ok(())
    }

    pub fn quick_save(&mut self, is_auto_save: bool) -> NSComResult {
        let now = Local::now();
        let hash = |mut src: u32, hashed: &mut String| {
            src %= 100;
            while src > 0 {
                match (src % 62) as u8 {
                    n @ 0..=25 => *hashed += &String::from((n + b'a') as char),
                    n @ 26..=51 => *hashed += &String::from((n - 26 + b'A') as char),
                    n @ 52..=61 => *hashed += &(n - 52).to_string(),
                    _ => (),
                }
                if src < 62 {
                    break;
                }
                src -= 62;
            }
        };

        let mut name = String::from(if is_auto_save { "as_" } else { "qs_" });

        hash((now.year() % 100) as u32, &mut name);
        hash(now.month(), &mut name);
        hash(now.day(), &mut name);
        hash(now.hour(), &mut name);
        hash(now.minute(), &mut name);
        hash(now.second(), &mut name);

        self.save(name, "".to_string())?;
        Ok(())
    }

    pub fn overwrite_save(&mut self) -> NSComResult {
        if let Some(save) = self.m_info.saves.last_mut() {
            save.protect()?;
            let name = save.get_name();

            if !self
                .m_opm
                .confirm(t!("overwrite_warning", save_name = name).to_string())?
            {
                return Ok(());
            }

            self.m_file_operator.remove_save(name)?;
            self.m_file_operator.save(name)?;

            save.modify_date(Self::get_date());
            save.modify_time(Self::get_time());

            self.write_infos()?;
            Ok(())
        } else {
            throw(&t!(
                "no_save_to_operation",
                operation = t!("overwrite_operation")
            ))
        }
    }

    pub fn auto_save(
        &mut self,
        max_auto_saves: usize,
    ) -> NSResult<(Option<SingleSave>, SingleSave)> {
        let saves = &mut self.m_info.saves;
        let mut removed_save = None;
        let auto_saves_count = saves
            .iter()
            .filter(|&item| item.get_name().starts_with("as_"))
            .count();
        if auto_saves_count >= max_auto_saves {
            let index = saves
                .iter()
                .position(|item| item.get_name().starts_with("as_"))
                .unwrap();
            self.m_file_operator
                .remove_save(saves[index].get_name())
                .explain(&t!("delete_save_fail"))?;
            removed_save = Some(saves.remove(index));
        }
        self.quick_save(true)?;
        self.m_file_operator
            .write_infos(&self.m_info)
            .explain(&t!("fail_modify_info_after_delete"))?;
        let latest = self.m_info.saves.last().unwrap();
        Ok((removed_save, (latest.clone())))
    }

    pub fn load_save(&self, index: usize) -> NSComResult {
        if let Some(item) = self.m_info.saves.get(index) {
            self.m_file_operator
                .load_save(item.get_name().to_string())?;
            Ok(())
        } else {
            if self.m_info.saves.is_empty() {
                return throw(&t!(
                    "no_save_to_operation",
                    operation = t!("load_operation")
                ));
            }
            throw(&t!("invalid_index"))
        }
    }

    #[inline]
    pub fn quick_load(&self) -> NSComResult {
        if self.m_info.saves.is_empty() {
            return throw(&t!(
                "no_save_to_operation",
                operation = t!("load_operation")
            ));
        }
        self.load_save(self.m_info.saves.len() - 1)
    }

    pub fn modify_save_info(
        &mut self,
        index: usize,
        new_name: Option<String>,
        new_note: Option<String>,
    ) -> NSComResult {
        if let Some(item) = self.m_info.saves.get_mut(index) {
            item.protect()?;
            if let Some(name) = new_name {
                if name.trim().is_empty() {
                    return throw(&t!("save_name_empty"));
                }

                self.m_file_operator.rename_save(item.get_name(), &name)?;
                item.modify_name(name);
            }
            if let Some(note) = new_note {
                item.modify_note(note);
            }
            self.write_infos()?;
            Ok(())
        } else {
            throw(&t!("invalid_index"))
        }
    }

    pub fn delete_saves(&mut self, indexes: Vec<usize>) -> NSComResult {
        let mut confirm_msg = t!("delete_save_warning").to_string();
        confirm_msg.push_str(":\n");

        let mut filtered_indexes = Vec::<usize>::new();
        for index in indexes {
            if let Some(item) = self.m_info.saves.get(index)
                && let Ok(()) = item.protect()
            {
                confirm_msg += &format!(
                    "[{}]  {}\t\t{}\n",
                    index + 1,
                    item.get_name(),
                    item.get_note()
                );
                filtered_indexes.push(index);
            }
        }
        if filtered_indexes.is_empty() {
            return throw(&t!("no_save_to_delete"));
        }
        confirm_msg.push_str(&t!("preprocessed_indexes_list_prompt_delete"));
        if self.m_opm.confirm(confirm_msg)? {
            for &index in filtered_indexes.iter().rev() {
                let item = &self.m_info.saves[index];
                self.m_file_operator
                    .remove_save(item.get_name())
                    .explain(&t!("delete_save_fail"))?;
                let save = self.m_info.saves.remove(index);

                self.m_opm.log_green(
                    t!(
                        "save_deleted",
                        index = index + 1,
                        save_name = save.get_name()
                    )
                    .to_string()
                        + "\n",
                );
            }
            self.m_file_operator
                .write_infos(&self.m_info)
                .explain(&t!("fail_modify_info_after_delete"))?;
        }
        Ok(())
    }

    #[inline]
    pub fn quick_delete_save(&mut self) -> NSComResult {
        let index = if self.m_info.saves.is_empty() {
            0
        } else {
            self.m_info.saves.len() - 1
        };
        self.delete_saves(vec![index])
    }

    pub fn lock(&mut self, indexes: Vec<usize>) -> NSComResult {
        let mut suc_msg = t!("lock_suc_msg").to_string();
        suc_msg.push_str(":\n");
        let mut any_valid = false;
        for index in indexes {
            if let Some(item) = self.m_info.saves.get_mut(index) {
                item.lock();
                suc_msg += &format!(
                    "[{}]  {}\t\t{}\n",
                    index + 1,
                    item.get_name(),
                    item.get_note()
                );
                any_valid = true;
            }
        }
        if any_valid {
            suc_msg += &t!("preprocessed_indexes_list_prompt_lock");
            suc_msg.push('\n');
            self.m_opm.log_green(suc_msg);
        } else {
            self.m_opm.warning(t!("no_save_to_lock").to_string() + "\n");
        }
        self.write_infos()?;
        Ok(())
    }

    pub fn unlock(&mut self, indexes: Vec<usize>) -> NSComResult {
        let mut suc_msg = t!("unlock_suc_msg").to_string();
        suc_msg.push_str(":\n");
        let mut any_valid = false;
        for index in indexes {
            if let Some(item) = self.m_info.saves.get_mut(index) {
                item.unlock();
                suc_msg += &format!(
                    "[{}]  {}\t\t{}\n",
                    index + 1,
                    item.get_name(),
                    item.get_note()
                );
                any_valid = true;
            }
        }
        if any_valid {
            suc_msg += &t!("preprocessed_indexes_list_prompt_lock");
            suc_msg.push('\n');
            self.m_opm.log_green(suc_msg);
        } else {
            self.m_opm
                .warning(t!("no_save_to_unlock").to_string() + "\n");
        }
        self.write_infos()?;
        Ok(())
    }

    #[inline]
    pub fn usage_by_mb() -> NSResult<f64> {
        FileOperator::caculate_usage(Path::new(SAVE_FOLDER_PATH))
    }
}
