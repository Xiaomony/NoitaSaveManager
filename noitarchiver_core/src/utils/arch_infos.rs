use super::error::*;

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

const MSG_FAVORED_UNTOUCHABLE: &str = "Locked archives can't be touched";

#[derive(Serialize, Deserialize)]
pub struct SingleArch {
    m_data: String,
    m_time: String,
    m_name: String,
    m_note: String,
    m_islocked: bool,
}

#[derive(Serialize, Deserialize)]
pub struct AllInfos {
    noita_exe_path: PathBuf,
    pub archives: Vec<SingleArch>,
}

impl SingleArch {
    pub fn new(data: String, time: String, name: String, note: String) -> Self {
        Self {
            m_data: data,
            m_time: time,
            m_name: name,
            m_note: note,
            m_islocked: false,
        }
    }

    #[inline]
    pub fn modify_data(&mut self, new_data: String) {
        self.m_data = new_data;
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
    pub fn get_data(&self) -> &str {
        &self.m_data
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
    pub fn protect(&self) -> NAComResult {
        if self.m_islocked {
            throw(MSG_FAVORED_UNTOUCHABLE)
        } else {
            Ok(())
        }
    }
}

impl AllInfos {
    #[inline]
    pub fn get_exe_path(&self) -> &Path {
        &self.noita_exe_path
    }
}
