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
            throwfatal("Fail to get the path where noita store the archive")
        }
    }

    fn open_info_file() -> NAResult<fs::File> {
        fs::create_dir_all(ARCH_FOLDER_PATH)
            .explain_fatal("Fail to create folder to store archives")?;
        let mut f = fs::OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(ARCH_INFO_PATH)
            .explain_fatal("Fail to create/open archives info file")?;
        if f.metadata()
            .explain("Fail to query the size of archive info file")?
            .len()
            == 0
        {
            f.write_all(br#"{ "noita_exe_path":"", "archives":[]}"#)
                .explain_fatal("Fail to initialize archive info file")?;
            f.rewind().explain(
                "Fail to go back to the beginning after initialize the archive info file",
            )?;
        }
        f.try_lock_exclusive().explain_fatal("Archive Info file(Archives/infos.json) has already been occupied.Maybe another Noitarchiver is running.")?;
        Ok(f)
    }

    fn copy_dir(src: &Path, dst: &Path) -> NAComResult {
        if !src.exists() {
            return throw(&format!(
                "The source path (\"{}\") needs to copy does not exists",
                src.to_str().unwrap()
            ));
        }
        fs::create_dir_all(dst).explain_fatal(&format!(
            "Fail to create destination path (\"{}\") while copying",
            dst.to_str().unwrap()
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
            .explain_fatal("Fail to parse the archive info file")?;
        Ok(infos)
    }

    pub fn write_infos(&mut self, infos: &AllInfos) -> NAComResult {
        self.m_file
            .rewind()
            .explain("Failt to go back to the beginning before writing into archive info file")?;
        serde_json::to_writer(&self.m_file, infos)
            .explain("Fail to write archives info into \"info.json\"")?;
        let pos = self
            .m_file
            .stream_position()
            .explain("Fail to get the current position after writing into archive info file")?;
        self.m_file
            .set_len(pos)
            .explain("Fail to set file length after writing into archive info file")?;
        Ok(())
    }

    pub fn save_archive(&self, folder_name: &str) -> NAComResult {
        let dst = PathBuf::from(ARCH_FOLDER_PATH).join(folder_name);
        Self::copy_dir(&self.m_noita_arch_path, &dst).explain("Fail to save the archive file")?;
        Ok(())
    }

    pub fn remove_archive(&self, folder_name: &str) -> NAComResult {
        fs::remove_dir_all(PathBuf::from(ARCH_FOLDER_PATH).join(folder_name))
            .explain(&format!("Fail to remove forlder \"{folder_name}\""))?;
        Ok(())
    }

    pub fn load_archive(&self, folder_name: String) -> NAComResult {
        fs::remove_dir_all(&self.m_noita_arch_path)
            .explain("Fail to remove existing noita archive")?;
        Self::copy_dir(
            &PathBuf::from(ARCH_FOLDER_PATH).join(folder_name),
            &self.m_noita_arch_path,
        )
        .explain("Fail to load archive")?;
        Ok(())
    }

    pub fn rename_archive(&self, old_name: &str, new_name: &str) -> NAComResult {
        let arch_folder_path = PathBuf::from(ARCH_FOLDER_PATH);
        fs::rename(
            arch_folder_path.join(old_name),
            arch_folder_path.join(new_name),
        )
        .explain("Fail to rename archive folder")?;
        Ok(())
    }

    pub fn caculate_usage(path: &Path) -> NAResult<f64> {
        let mut size = 0f64;
        for entry in fs::read_dir(path).explain(&format!(
            "Error ocurred when caculating the size of \"{:?}\"",
            path,
        ))? {
            let dir = entry.explain(&format!(
                "Error ocurred when caculating the size of \"{:?}\"",
                path,
            ))?;
            let metadata = dir.metadata().explain(&format!(
                "Error ocurred when caculating the size of \"{:?}\"",
                path,
            ))?;
            size += if metadata.is_dir() {
                Self::caculate_usage(&dir.path())?
            } else {
                metadata.len() as f64 / 1_048_576f64
            }
        }
        Ok(size)
    }
}
