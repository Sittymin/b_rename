use crate::error::FileError;

use std::ffi::OsString;
// use std::fmt::Display;
use std::path::{Path, PathBuf};

pub struct File {
    // Will be modified
    full_path: PathBuf,
    file_name: OsString,
    file_ext: OsString,
}
// impl Display for File {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         writeln!(
//             f,
//             "完整路径: {} | 文件名: {} | 文件后缀: {}",
//             self.full_path.display(),
//             self.file_name,
//             self.file_ext
//         )
//     }
// }
impl File {
    pub fn new(path: PathBuf) -> Result<File, FileError> {
        let file_name = path
            .file_stem()
            .ok_or(FileError::NameError("文件名读取失败".to_string()))?
            .to_os_string();

        let file_ext = match path.extension() {
            Some(ext) => ext.to_os_string(),
            None => {
                if let Some(name_str) = file_name.to_str() {
                    // 检查是否是隐藏文件
                    if name_str.starts_with(".") {
                        OsString::from(&name_str[1..])
                    } else {
                        OsString::new()
                    }
                } else {
                    // 转换 str 失败
                    OsString::new()
                }
            }
        };

        Ok(File {
            full_path: path,
            file_name,
            file_ext,
        })
    }
    pub fn get_file_name(&self) -> &OsString {
        &self.file_name
    }
    pub fn get_file_ext(&self) -> &OsString {
        &self.file_ext
    }
    pub fn get_file_path(&self) -> &Path {
        Path::new(&self.full_path)
    }
    pub fn update_info(&mut self, new_file: PathBuf) -> Result<(), FileError> {
        self.full_path = new_file.to_path_buf();
        // 双重后缀可能不好用 like "1.zst.tar"
        self.file_name = new_file
            .file_stem()
            .ok_or(FileError::NameError("更新文件名信息时失败".to_string()))?
            .to_os_string();
        self.file_ext = new_file
            .extension()
            .ok_or(FileError::ExtError("更新文件后缀信息时失败".to_string()))?
            .to_os_string();
        Ok(())
    }
}
impl Clone for File {
    fn clone(&self) -> Self {
        File {
            full_path: self.full_path.clone(),
            file_name: self.file_name.clone(),
            file_ext: self.file_ext.clone(),
        }
    }
}
