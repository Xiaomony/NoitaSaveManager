mod utils;

use utils::arch_infos::{AllInfos, SingleArch};
pub use utils::error::*;
use utils::file_operator::{FileOperator, ARCH_FOLDER_PATH};
pub use utils::output_manager;
use utils::output_manager::OutputManager;

use chrono::{Datelike, Local, Timelike};

use std::os::windows::process::CommandExt;
use std::path::Path;
use std::process::Command;

pub struct Core<Opm: OutputManager> {
    m_file_operator: FileOperator,
    m_info: AllInfos,
    m_opm: Opm,
}

impl<Opm: OutputManager> Core<Opm> {
    pub fn new(opm: Opm) -> NAResult<Self> {
        let file_operator = FileOperator::new()?;
        Ok(Self {
            m_info: file_operator.load_infos()?,
            m_file_operator: file_operator,
            m_opm: opm,
        })
    }

    #[inline]
    fn get_data() -> String {
        Local::now().format("%Y-%m-%d").to_string()
    }

    #[inline]
    fn get_time() -> String {
        Local::now().format("%H:%M:%S").to_string()
    }

    #[inline]
    pub fn get_arch_infos(&self) -> &AllInfos {
        &self.m_info
    }

    #[inline]
    pub fn write_infos(&mut self) -> NAComResult {
        self.m_file_operator.write_infos(&self.m_info)
    }

    pub fn startgame(&self) -> NAComResult {
        let noipath = self.m_info.get_exe_path();
        if !(noipath.exists() && noipath.ends_with("noita.exe")) {
            return throw("Please set a proper noita.exe path (ends with \"noita.exe\")");
        } else {
            self.m_opm.warning("Starting noita while Steam is NOT running will causes the mods added by Steam unloaded.".to_string());
            Command::new(noipath)
                .creation_flags(0x00000008) // run noita in detached process
                .current_dir(noipath.parent().unwrap())
                .spawn()?;
        }
        Ok(())
    }

    pub fn set_noita_path(&mut self, path: String) -> NAComResult {
        self.m_info.set_noita_path(path);
        self.write_infos()
    }

    pub fn save(&mut self, archive_name: String, archive_note: String) -> NAComResult {
        if self
            .m_info
            .archives
            .iter()
            .any(|item| item.get_name() == archive_name)
        {
            return throw("The archive name is the same with another.Please change one");
        }

        if archive_name.is_empty() {
            return throw("Archive name can't be empty");
        }

        self.m_file_operator.save_archive(&archive_name)?;
        self.m_info.archives.push(SingleArch::new(
            Self::get_data(),
            Self::get_time(),
            archive_name,
            archive_note,
        ));
        self.write_infos()?;
        Ok(())
    }

    pub fn quick_save(&mut self) -> NAComResult {
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

        let mut name = String::from("qsave_");

        hash((now.year() % 100) as u32, &mut name);
        hash(now.month(), &mut name);
        hash(now.day(), &mut name);
        hash(now.hour(), &mut name);
        hash(now.minute(), &mut name);
        hash(now.second(), &mut name);

        self.save(name, "".to_string())?;
        Ok(())
    }

    pub fn replace_save(&mut self) -> NAComResult {
        if let Some(arch) = self.m_info.archives.last_mut() {
            arch.protect()?;
            let name = arch.get_name();

            if !self.m_opm.confirm(format!(
                "This will cause the archive \"{}\" be replaced",
                name
            ))? {
                return Ok(());
            }

            self.m_file_operator.remove_archive(name)?;
            self.m_file_operator.save_archive(name)?;

            arch.modify_data(Self::get_data());
            arch.modify_time(Self::get_time());

            self.write_infos()?;
            Ok(())
        } else {
            throw("There's no archives to replace")
        }
    }

    pub fn load_archive(&self, index: usize) -> NAComResult {
        if let Some(item) = self.m_info.archives.get(index) {
            self.m_file_operator
                .load_archive(item.get_name().to_string())?;
            Ok(())
        } else {
            if self.m_info.archives.is_empty() {
                return throw("No archives to load");
            }
            throw("The index of the archive need to load is invalid")
        }
    }

    #[inline]
    pub fn quick_load(&self) -> NAComResult {
        if self.m_info.archives.is_empty() {
            return throw("No archives to load");
        }
        self.load_archive(self.m_info.archives.len() - 1)
    }

    pub fn modify_arch_info(
        &mut self,
        index: usize,
        new_name: Option<String>,
        new_note: Option<String>,
    ) -> NAComResult {
        if let Some(item) = self.m_info.archives.get_mut(index) {
            item.protect()?;
            if let Some(name) = new_name {
                if name.trim().is_empty() {
                    return throw("Archive name can't be empty");
                }

                self.m_file_operator
                    .rename_archive(item.get_name(), &name)?;
                item.modify_name(name);
            }
            if let Some(note) = new_note {
                item.modify_note(note);
            }
            self.write_infos()?;
            Ok(())
        } else {
            throw("The index of the archive need to modify is invalid")
        }
    }

    pub fn delete_archives(&mut self, indexes: Vec<usize>) -> NAComResult {
        let mut confirm_msg = "This will cause the following archives to be deleted:\n".to_string();

        let mut filtered_indexes = Vec::<usize>::new();
        for index in indexes {
            if let Some(item) = self.m_info.archives.get(index) &&
                let Ok(()) = item.protect() {
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
            return throw(
                "No archives to delete after invalid index and locked archives are filtered",
            );
        }
        confirm_msg += "Invalid indexes and locked archives are filtered";
        if self.m_opm.confirm(confirm_msg)? {
            for &index in filtered_indexes.iter().rev() {
                let item = &self.m_info.archives[index];
                self.m_file_operator
                    .remove_archive(item.get_name())
                    .explain("Fail to delete archive")?;
                let arch = self.m_info.archives.remove(index);

                self.m_opm.log_green(format!(
                    "\"[{}] {}\" has been deleted\n",
                    index + 1,
                    arch.get_name()
                ));
            }
            self.m_file_operator
                .write_infos(&self.m_info)
                .explain("Fail to modify archive info file after the archive is deleted")?;
        }
        Ok(())
    }

    #[inline]
    pub fn quick_delete_archive(&mut self) -> NAComResult {
        self.delete_archives(vec![self.m_info.archives.len() - 1])
    }

    pub fn lock(&mut self, indexes: Vec<usize>) -> NAComResult {
        let mut suc_msg = "The following archives have been locked:\n".to_string();
        let mut any_valid = false;
        for index in indexes {
            if let Some(item) = self.m_info.archives.get_mut(index) {
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
            suc_msg += "Invalid indexes are filtered\n";
            self.m_opm.log_green(suc_msg);
        } else {
            self.m_opm
                .warning("No archives to lock after invalid indexes are filtered\n".to_string());
        }
        self.write_infos()?;
        Ok(())
    }

    pub fn unlock(&mut self, indexes: Vec<usize>) -> NAComResult {
        let mut suc_msg = "The following archives have been unlocked:\n".to_string();
        let mut any_valid = false;
        for index in indexes {
            if let Some(item) = self.m_info.archives.get_mut(index) {
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
            suc_msg += "Invalid indexes are filtered\n";
            self.m_opm.log_green(suc_msg);
        } else {
            self.m_opm
                .warning("No archives to lock after invalid indexes are filtered\n".to_string());
        }
        self.write_infos()?;
        Ok(())
    }

    #[inline]
    pub fn usage_by_mb() -> NAResult<f64> {
        FileOperator::caculate_usage(Path::new(ARCH_FOLDER_PATH))
    }
}
