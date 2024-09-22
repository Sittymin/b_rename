use std::ffi::OsString;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct File {
    // Will be modified
    full_path: PathBuf,
    file_name: OsString,
    file_full_name: OsString,
    file_ext: OsString,
}
impl File {
    pub fn new(path: PathBuf) -> io::Result<File> {
        let file_name = path
            // like .gitignore returns the full name.
            .file_stem()
            .ok_or(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("不存在文件名: {}", path.display()),
            ))?
            .to_os_string();

        let file_ext = match path.extension() {
            Some(ext) => ext.to_os_string(),
            _ => {
                if let Some(name_str) = file_name.to_str() {
                    // like ".gitignore"
                    if name_str.starts_with(".") {
                        OsString::from(&name_str[1..])
                    } else {
                        OsString::new()
                    }
                } else {
                    // conversions str fail
                    OsString::new()
                }
            }
        };
        let mut file_full_name = OsString::from(file_name.clone());
        file_full_name.push(".");
        file_full_name.push(file_ext.clone());

        Ok(File {
            full_path: path,
            file_name,
            file_full_name,
            file_ext,
        })
    }
    pub fn get_file_name(&self) -> &OsString {
        &self.file_name
    }
    pub fn get_file_full_name(&self) -> &OsString {
        &self.file_full_name
    }
    pub fn get_file_ext(&self) -> &OsString {
        &self.file_ext
    }
    pub fn get_file_path(&self) -> &Path {
        Path::new(&self.full_path)
    }
    pub fn update_info(&mut self, new_file: PathBuf) -> io::Result<()> {
        self.full_path = new_file.to_path_buf();
        // like "1.zst.tar" is not will
        self.file_name = new_file
            .file_stem()
            .ok_or(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("更新文件名信息失败: {}", new_file.display()),
            ))?
            .to_os_string();
        self.file_ext = new_file
            .extension()
            .ok_or(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("更新文件后缀信息时失败: {}", new_file.display()),
            ))?
            .to_os_string();
        Ok(())
    }
}
impl Clone for File {
    fn clone(&self) -> Self {
        File {
            full_path: self.full_path.clone(),
            file_name: self.file_name.clone(),
            file_full_name: self.file_full_name.clone(),
            file_ext: self.file_ext.clone(),
        }
    }
}
