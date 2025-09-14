use directories::BaseDirs;
use fs2::FileExt;
use std::fs;
use std::io::{Seek, Write};
use std::path::{Path, PathBuf};

use super::arch_infos::AllInfos;
use super::error::*;

pub const ARCH_FOLDER_PATH: &str = r".\Archives\";
const ARCH_INFO_PATH: &str = r".\Archives\infos.json";
const NOITA_ARCH_PATH_POSTFIX: &str = r"Appdata\LocalLow\Nolla_Games_Noita\save00";

pub struct FileOperator {
    m_file: fs::File,
    m_noita_arch_path: PathBuf,
}

impl FileOperator {
    pub fn new() -> NAResult<Self> {
        Ok(Self {
            m_file: Self::open_info_file()?,
            m_noita_arch_path: Self::get_noita_archive_path()?,
        })
    }

    fn get_noita_archive_path() -> NAResult<PathBuf> {
        if let Some(home_dir) = BaseDirs::new() {
            Ok(home_dir.home_dir().join(NOITA_ARCH_PATH_POSTFIX))
        } else {
            throwfatal(&t!("fail_get_noita_archive_store_path"))
        }
    }

    fn open_info_file() -> NAResult<fs::File> {
        fs::create_dir_all(ARCH_FOLDER_PATH)
            .explain_fatal(&t!("fail_create_archive_storage_folder"))?;
        let mut f = fs::OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(ARCH_INFO_PATH)
            .explain_fatal(&t!("fail_create_archive_storage_folder"))?;
        if f.metadata()
            .explain(&t!("fail_query_info_file_size"))?
            .len()
            == 0
        {
            f.write_all(br#"{ "noita_exe_path":"", "archives":[]}"#)
                .explain_fatal(&t!("fail_initialize_info_file"))?;
            f.rewind()
                .explain(&t!("fail_goto_info_file_start_after_init"))?;
        }
        f.try_lock_exclusive()
            .explain_fatal(&t!("info_file_occupied"))?;
        Ok(f)
    }

    fn copy_dir(src: &Path, dst: &Path) -> NAComResult {
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

    pub fn load_infos(&self) -> NAResult<AllInfos> {
        let infos: AllInfos = serde_json::from_reader(std::io::BufReader::new(&self.m_file))
            .explain_fatal(&t!("fail_parse_archive_info_file"))?;
        Ok(infos)
    }

    pub fn write_infos(&mut self, infos: &AllInfos) -> NAComResult {
        self.m_file
            .rewind()
            .explain(&t!("fail_go_back_to_start_before_writing"))?;
        serde_json::to_writer(&self.m_file, infos).explain(&t!("fail_write_into_info_file"))?;
        let pos = self
            .m_file
            .stream_position()
            .explain(&t!("fail_get_crr_pos_after_writing"))?;
        self.m_file
            .set_len(pos)
            .explain(&t!("fail_set_file_length_after_writing"))?;
        Ok(())
    }

    pub fn save_archive(&self, folder_name: &str) -> NAComResult {
        let dst = PathBuf::from(ARCH_FOLDER_PATH).join(folder_name);
        Self::copy_dir(&self.m_noita_arch_path, &dst).explain(&t!("fail_save_achive"))?;
        Ok(())
    }

    pub fn remove_archive(&self, folder_name: &str) -> NAComResult {
        fs::remove_dir_all(PathBuf::from(ARCH_FOLDER_PATH).join(folder_name))
            .explain(&t!("fail_remove_folder", folder_name = folder_name))?;
        Ok(())
    }

    pub fn load_archive(&self, folder_name: String) -> NAComResult {
        fs::remove_dir_all(&self.m_noita_arch_path)
            .explain(&t!("fail_remove_crr_noita_archive"))?;
        Self::copy_dir(
            &PathBuf::from(ARCH_FOLDER_PATH).join(folder_name),
            &self.m_noita_arch_path,
        )
        .explain(&t!("fail_load_archive"))?;
        Ok(())
    }

    pub fn rename_archive(&self, old_name: &str, new_name: &str) -> NAComResult {
        let arch_folder_path = PathBuf::from(ARCH_FOLDER_PATH);
        fs::rename(
            arch_folder_path.join(old_name),
            arch_folder_path.join(new_name),
        )
        .explain(&t!("fail_rename_archive_folder"))?;
        Ok(())
    }

    pub fn caculate_usage(path: &Path) -> NAResult<f64> {
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
