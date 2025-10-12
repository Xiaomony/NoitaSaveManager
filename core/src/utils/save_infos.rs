use super::error::*;

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SingleSave {
    pub m_date: String,
    pub m_time: String,
    pub m_name: String,
    pub m_note: String,
    m_islocked: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AllInfos {
    noita_exe_path: PathBuf,
    pub saves: Vec<SingleSave>,
}

impl SingleSave {
    pub fn new(date: String, time: String, name: String, note: String) -> Self {
        Self {
            m_date: date,
            m_time: time,
            m_name: name,
            m_note: note,
            m_islocked: false,
        }
    }

    #[inline]
    pub fn modify_date(&mut self, new_date: String) {
        self.m_date = new_date;
    }

    #[inline]
    pub fn modify_time(&mut self, new_time: String) {
        self.m_time = new_time;
    }

    #[inline]
    pub fn modify_name(&mut self, new_name: String) {
        self.m_name = new_name;
    }

    #[inline]
    pub fn modify_note(&mut self, new_note: String) {
        self.m_note = new_note;
    }

    #[inline]
    pub fn lock(&mut self) {
        self.m_islocked = true;
    }

    #[inline]
    pub fn unlock(&mut self) {
        self.m_islocked = false;
    }

    #[inline]
    pub fn get_date(&self) -> &str {
        &self.m_date
    }

    #[inline]
    pub fn get_time(&self) -> &str {
        &self.m_time
    }

    #[inline]
    pub fn get_name(&self) -> &str {
        &self.m_name
    }

    #[inline]
    pub fn get_note(&self) -> &str {
        &self.m_note
    }

    #[inline]
    pub fn is_locked(&self) -> bool {
        self.m_islocked
    }

    #[inline]
    pub fn protect(&self) -> NSComResult {
        if self.m_islocked {
            throw(&t!("locked_untouchable_msg"))
        } else {
            Ok(())
        }
    }
}

impl std::fmt::Display for SingleSave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}  {}\t{}\t\t\t{}",
            self.get_date(),
            self.get_time(),
            self.get_name(),
            self.get_note()
        )
    }
}

impl AllInfos {
    #[inline]
    pub fn get_exe_path(&self) -> &Path {
        &self.noita_exe_path
    }

    #[inline]
    pub fn set_noita_path(&mut self, new_path: String) {
        self.noita_exe_path = PathBuf::from(new_path);
    }
}
