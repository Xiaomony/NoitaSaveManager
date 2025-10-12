use directories::BaseDirs;
use fs2::FileExt;
use std::fs;
use std::io::{Seek, Write};
use std::path::{Path, PathBuf};

use super::error::*;
use super::save_infos::AllInfos;

pub const SAVE_FOLDER_PATH: &str = r".\Saves\";
const SAVE_INFO_PATH: &str = r".\Saves\infos.json";
const NOITA_SAVE_PATH_POSTFIX: &str = r"Appdata\LocalLow\Nolla_Games_Noita\save00";

#[derive(Debug)]
pub struct FileOperator {
    m_file: fs::File,
    m_noita_save_path: PathBuf,
}

impl FileOperator {
    pub fn new() -> NSResult<Self> {
        Ok(Self {
            m_file: Self::open_info_file()?,
            m_noita_save_path: Self::get_noita_save_path()?,
        })
    }

    fn get_noita_save_path() -> NSResult<PathBuf> {
        if let Some(home_dir) = BaseDirs::new() {
            Ok(home_dir.home_dir().join(NOITA_SAVE_PATH_POSTFIX))
        } else {
            throwfatal(&t!("fail_get_noita_save_store_path"))
        }
    }

    fn open_info_file() -> NSResult<fs::File> {
        fs::create_dir_all(SAVE_FOLDER_PATH)
            .explain_fatal(&t!("fail_create_save_storage_folder"))?;
        let mut f = fs::OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(SAVE_INFO_PATH)
            .explain_fatal(&t!("fail_create_save_storage_folder"))?;
        if f.metadata()
            .explain(&t!("fail_query_info_file_size"))?
            .len()
            == 0
        {
            f.write_all(br#"{ "noita_exe_path":"", "saves":[]}"#)
                .explain_fatal(&t!("fail_initialize_info_file"))?;
            f.rewind()
                .explain(&t!("fail_goto_info_file_start_after_init"))?;
        }
        f.try_lock_exclusive()
            .explain_fatal(&t!("info_file_occupied"))?;
        Ok(f)
    }

    fn copy_dir(src: &Path, dst: &Path) -> NSComResult {
        if !src.exists() {
            return throw(&t!("source_path_not_exist", path = src.to_str().unwrap()));
        }
        fs::create_dir_all(dst).explain_fatal(&t!(
            "fail_create_destination_path",
            path = dst.to_str().unwrap()
        ))?;
        for entry in fs::read_dir(src)? {
            let dir_entry = entry?;
            if dir_entry.file_type()?.is_dir() {
                Self::copy_dir(
                    &dir_entry.path(),
                    &dst.join(dir_entry.path().file_name().unwrap()),
                )?;
            } else {
                fs::copy(
                    dir_entry.path(),
                    dst.join(dir_entry.path().file_name().unwrap()),
                )?;
            }
        }
        Ok(())
    }

    pub fn load_infos(&self) -> NSResult<AllInfos> {
        let infos: AllInfos = serde_json::from_reader(std::io::BufReader::new(&self.m_file))
            .explain_fatal(&t!("fail_parse_save_info_file"))?;
        Ok(infos)
    }

    pub fn write_infos(&mut self, infos: &AllInfos) -> NSComResult {
        self.m_file
            .rewind()
            .explain(&t!("fail_go_back_to_start_before_writing"))?;
        serde_json::to_writer_pretty(&self.m_file, infos)
            .explain(&t!("fail_write_into_info_file"))?;
        let pos = self
            .m_file
            .stream_position()
            .explain(&t!("fail_get_crr_pos_after_writing"))?;
        self.m_file
            .set_len(pos)
            .explain(&t!("fail_set_file_length_after_writing"))?;
        Ok(())
    }

    pub fn save(&self, folder_name: &str) -> NSComResult {
        let dst = PathBuf::from(SAVE_FOLDER_PATH).join(folder_name);
        Self::copy_dir(&self.m_noita_save_path, &dst).explain(&t!("fail_save_achive"))?;
        Ok(())
    }

    pub fn remove_save(&self, folder_name: &str) -> NSComResult {
        fs::remove_dir_all(PathBuf::from(SAVE_FOLDER_PATH).join(folder_name))
            .explain(&t!("fail_remove_folder", folder_name = folder_name))?;
        Ok(())
    }

    pub fn load_save(&self, folder_name: String) -> NSComResult {
        fs::remove_dir_all(&self.m_noita_save_path).explain(&t!("fail_remove_crr_noita_save"))?;
        Self::copy_dir(
            &PathBuf::from(SAVE_FOLDER_PATH).join(folder_name),
            &self.m_noita_save_path,
        )
        .explain(&t!("fail_load_save"))?;
        Ok(())
    }

    pub fn rename_save(&self, old_name: &str, new_name: &str) -> NSComResult {
        let save_folder_path = PathBuf::from(SAVE_FOLDER_PATH);
        fs::rename(
            save_folder_path.join(old_name),
            save_folder_path.join(new_name),
        )
        .explain(&t!("fail_rename_save_folder"))?;
        Ok(())
    }

    pub fn caculate_usage(path: &Path) -> NSResult<f64> {
        let mut size = 0f64;
        for entry in
            fs::read_dir(path).explain(&t!("fail_caculate_size", path = format!("{path:?}")))?
        {
            let dir = entry.explain(&t!("fail_caculate_size", path = format!("{path:?}")))?;
            let metadata = dir
                .metadata()
                .explain(&t!("fail_caculate_size", path = format!("{path:?}")))?;
            size += if metadata.is_dir() {
                Self::caculate_usage(&dir.path())?
            } else {
                metadata.len() as f64 / 1_048_576f64
            }
        }
        Ok(size)
    }
}
