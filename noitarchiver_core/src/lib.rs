mod utils;
use utils::arch_infos::{AllInfos, SingleArch};
use utils::error::throw;
use utils::error::NAchError;
use utils::file_operator::{FileOperator, ARCH_FOLDER_PATH};
pub use utils::output_manager;
use utils::output_manager::OutputManager;

use chrono::{Datelike, Local, Timelike};

use std::os::windows::process::CommandExt;
use std::path::Path;
use std::process::Command;

/*
help(h)       帮助及注意事项  4.quit(q)       退出程序

12.modarch(ma)  修改存档信息
13.del(d)       删除指定存档    14.qdel(qd)     删除最新存档
15.favor(f)     收藏存档        16.unfavor(unf) 取消收藏

17.usage(use)   查看占用空间
*/

pub struct Core<Opm: OutputManager> {
    m_file_operator: FileOperator,
    m_info: AllInfos,
    m_opm: Opm,
}

impl<Opm: OutputManager> Core<Opm> {
    pub fn new(opm: Opm) -> Result<Self, NAchError> {
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

    pub fn startgame(&self) -> Result<(), NAchError> {
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

    pub fn save(&mut self, archive_name: String, archive_note: String) -> Result<(), NAchError> {
        if self
            .m_info
            .archives
            .iter()
            .any(|item| item.get_name() == archive_name)
        {
            return throw("The archive name is the same with another.Please change one");
        }

        self.m_file_operator.save_archive(&archive_name)?;
        self.m_info.archives.push(SingleArch::new(
            Self::get_data(),
            Self::get_time(),
            archive_name,
            archive_note,
        ));
        self.m_file_operator.write_infos(&self.m_info)?;
        Ok(())
    }

    pub fn quick_save(&mut self) -> Result<(), NAchError> {
        let now = Local::now();
        let hash = |mut src: u32, hashed: &mut String| {
            src %= 100;
            while src > 0 {
                match (src % 62) as u8 {
                    n @ 0..=25 => *hashed += &String::from((n + b'a') as char),
                    n @ 26..=51 => *hashed += &String::from((n + b'A') as char),
                    n @ 52..=61 => *hashed += &(n - 52).to_string(),
                    _ => (),
                }
                src -= 62;
            }
        };

        let mut name = String::new();

        hash((now.year() % 100) as u32, &mut name);
        hash(now.month(), &mut name);
        hash(now.day(), &mut name);
        hash(now.hour(), &mut name);
        hash(now.minute(), &mut name);
        hash(now.second(), &mut name);

        self.save(name, "".to_string())?;
        Ok(())
    }

    pub fn replace_save(&mut self) -> Result<(), NAchError> {
        if let Some(arch) = self.m_info.archives.last_mut() {
            arch.protect()?;
            let name = arch.get_name();
            self.m_file_operator.remove_archive(name)?;
            self.m_file_operator.save_archive(name)?;

            arch.modify_data(Self::get_data());
            arch.modify_time(Self::get_time());
            Ok(())
        } else {
            throw("There's no archives to replace")
        }
    }

    pub fn load_archive(&self, index: usize) -> Result<(), NAchError> {
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
    pub fn quick_load(&self) -> Result<(), NAchError> {
        self.load_archive(self.m_info.archives.len() - 1)
    }

    pub fn modify_arch_info(
        &mut self,
        index: usize,
        new_name: Option<String>,
        new_note: Option<String>,
    ) -> Result<(), NAchError> {
        if let Some(item) = self.m_info.archives.get_mut(index) {
            item.protect()?;
            if let Some(name) = new_name {
                item.modify_name(name);
            }
            if let Some(note) = new_note {
                item.modify_note(note);
            }
            Ok(())
        } else {
            throw("The index of the archive need to modify is invalid")
        }
    }

    pub fn delete_archive(&mut self, index: usize) -> Result<(), NAchError> {
        if let Some(item) = self.m_info.archives.get(index) {
            item.protect()?;
            self.m_file_operator.remove_archive(item.get_name())?;
            self.m_info.archives.remove(index);
            self.m_file_operator.write_infos(&self.m_info)?;
            Ok(())
        } else {
            throw("The index of the archive need to delete is invalid")
        }
    }

    #[inline]
    pub fn quick_delete_archive(&mut self) -> Result<(), NAchError> {
        self.delete_archive(self.m_info.archives.len() - 1)
    }

    pub fn lock(&mut self, index: usize) -> Result<(), NAchError> {
        if let Some(item) = self.m_info.archives.get_mut(index) {
            item.lock();
            Ok(())
        } else {
            throw("The index of the archive need to lock is invalid")
        }
    }

    pub fn unlock(&mut self, index: usize) -> Result<(), NAchError> {
        if let Some(item) = self.m_info.archives.get_mut(index) {
            item.unlock();
            Ok(())
        } else {
            throw("The index of the archive need to unlock is invalid")
        }
    }

    #[inline]
    pub fn usage() -> Result<f64, NAchError> {
        FileOperator::caculate_usage(Path::new(ARCH_FOLDER_PATH))
    }
}
