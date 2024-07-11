use crate::error::FileError;

use std::fmt::Display;
use std::path::{Path, PathBuf};

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
    pub fn new(path: PathBuf) -> Result<File, FileError> {
        let file_name = path
            .file_stem()
            .and_then(|name| name.to_str())
            .ok_or_else(|| FileError::NameError("文件名获取失败 -> File::new".to_string()))?
            .to_string();

        let file_ext = path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_string())
            // 没有扩展名是做以下处理
            .unwrap_or_else(|| {
                // 如果没有扩展名，检查文件名是否以点开头
                if file_name.starts_with('.') {
                    // 将整个文件名（不包括开头的点）作为扩展名
                    file_name[1..].to_string()
                } else {
                    // 如果既没有扩展名，文件名也不以点开头，则将扩展名设为空字符串
                    String::new()
                }
            });

        Ok(File {
            full_path: path,
            file_name,
            file_ext,
        })
    }
    pub fn get_file_name(&self) -> &String {
        &self.file_name
    }
    pub fn get_file_ext(&self) -> &String {
        &self.file_ext
    }
    pub fn get_file_path(&self) -> &Path {
        Path::new(&self.full_path)
    }
    pub fn update_info(&mut self, new_file: PathBuf) {
        self.full_path = new_file.to_path_buf();
        // 双重后缀可能不好用 like "1.zst.tar"
        self.file_name = new_file
            .file_stem()
            .and_then(|name| name.to_str())
            .map(|s| s.to_string())
            .expect("文件名竟然含有非 Unicode 字符！");
        self.file_ext = new_file
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_string())
            .expect("文件后缀竟然含有非 Unicode 字符！")
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
