use crate::error::FileError;

use std::fmt::Display;
use std::path::PathBuf;

pub struct File {
    // Will be modified
    full_path: PathBuf,
    file_name: String,
    file_ext: String,
}
impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "完整路径: {} | 文件名: {} | 文件后缀: {}",
            self.full_path.display(),
            self.file_name,
            self.file_ext
        )
    }
}
impl File {
    // Need to check before passing in the parameters
    fn new(path: PathBuf) -> Result<File, FileError> {
        let file_name = path
            .file_stem()
            .and_then(|name| name.to_str())
            // When return none
            .ok_or_else(|| FileError::NameError("文件名获取失败 -> File::new".to_string()))?
            .to_string();
        let file_ext = path
            .extension()
            .and_then(|ext| ext.to_str())
            .ok_or_else(|| FileError::ExtError("文件后缀获取失败 -> File::new".to_string()))?
            .to_string();
        Ok(File {
            full_path: path,
            file_name,
            file_ext,
        })
    }
}
