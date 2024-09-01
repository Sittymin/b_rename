use crate::{file::File, rename::batch_rename};

use core::panic;
use std::{ffi::OsString, fs, io, path::PathBuf};

#[derive(Debug)]
pub struct Dir {
    pub files: Vec<File>,
    pub dir_path: PathBuf,
    pub dir_name: OsString,
}

impl Dir {
    pub fn new(dir_path: PathBuf) -> Self {
        let files = read_dir(&dir_path).unwrap_or_else(|e| {
            eprintln!("读取目录发生错误: {e}");
            Vec::new()
        });
        let dir_name = get_dir_name(&dir_path);
        Dir {
            files,
            dir_path,
            dir_name,
        }
    }
    pub fn get_files_name(&self) -> Vec<&OsString> {
        self.files.iter().map(|file| file.get_file_name()).collect()
    }
    pub fn get_files_full_name(&self) -> Vec<&OsString> {
        self.files
            .iter()
            .map(|file| file.get_file_full_name())
            .collect()
    }
    pub fn sort_files_by_name_unstable(&mut self) {
        self.files
            .sort_unstable_by_key(|file| file.get_file_name().clone());
    }
}

pub struct InputDir {
    pub base_dir: Dir,
    // modify_dir: Dir,
    pub output_dir: Dir,
}

impl InputDir {
    pub fn new(
        base_dir: Dir,
        modify_dir: Dir,
        output_dir_path: PathBuf,
        is_move: bool,
    ) -> io::Result<Self> {
        let output_dir = if are_same_directory(&modify_dir.dir_path, &output_dir_path)? {
            Dir {
                files: modify_dir.files.clone(),
                dir_path: modify_dir.dir_path.clone(),
                dir_name: modify_dir.dir_name.clone(),
            }
        } else {
            // create a new Dir struct
            let mut output_dir = Dir {
                files: Vec::new(),
                dir_name: get_dir_name(&output_dir_path),
                dir_path: output_dir_path,
            };

            // copy or move files to new dir
            for file in &modify_dir.files {
                // file_name like file.ass
                let file_name = file.get_file_path().file_name().unwrap();
                let output_file_path = output_dir.dir_path.join(PathBuf::from(file_name));
                if is_move {
                    fs::rename(file.get_file_path(), &output_file_path).map_err(|e| {
                        io::Error::new(
                            io::ErrorKind::PermissionDenied,
                            format!("无法移动文件, 可能是权限问题: {e}"),
                        )
                    })?;
                } else {
                    fs::copy(file.get_file_path(), &output_file_path).map_err(|e| {
                        io::Error::new(
                            io::ErrorKind::PermissionDenied,
                            format!("无法复制到指定目录, 可能是权限问题: {e}"),
                        )
                    })?;
                }
                // create File struct to Dir.files
                let output_file = File::new(output_file_path)
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                output_dir.files.push(output_file);
            }

            output_dir
        };

        Ok(Self {
            base_dir,
            // modify_dir,
            output_dir,
        })
    }
    pub fn output_rename(&mut self) {
        batch_rename(&self.base_dir.files, &mut self.output_dir.files);
    }
}

fn are_same_directory(path1: &PathBuf, path2: &PathBuf) -> std::io::Result<bool> {
    let canonical1 = path1.canonicalize()?;
    let canonical2 = path2.canonicalize()?;
    Ok(canonical1 == canonical2)
}

// arg.rs is check before
fn get_dir_name(path: &PathBuf) -> OsString {
    path.components()
        .last()
        .unwrap_or_else(|| panic!("读取目录名失败 -> get_dir_name: {}", path.display()))
        .as_os_str()
        .to_owned()
}

fn read_dir(path: &PathBuf) -> io::Result<Vec<File>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            match File::new(path) {
                Ok(file) => files.push(file),
                Err(e) => {
                    eprintln!("read_dir 在构建 File 是失败");
                    eprintln!("错误: {e}");
                }
            }
        }
    }
    Ok(files)
}
