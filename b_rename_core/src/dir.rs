use crate::{file::File, rename::batch_rename};

use core::panic;
use std::{ffi::OsString, fs, io, path::PathBuf};

#[derive(Debug)]
pub struct Dir {
    pub files: Vec<File>,
    // Only use in init InputDir, compare output_dir and modify_dir is same or not
    // WARN: add_new_file is not in dir
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
    pub fn add_new_file(&mut self, file_path: PathBuf) -> io::Result<()> {
        // TIP: this is not in work dir
        let new_file: File = File::new(file_path)?;
        self.files.push(new_file);
        Ok(())
    }
    pub fn sort_files_by_name_unstable(&mut self) {
        self.files
            .sort_unstable_by_key(|file| file.get_file_name().clone());
    }
}
impl Clone for Dir {
    fn clone(&self) -> Self {
        Dir {
            files: self.files.clone(),
            dir_path: self.dir_path.clone(),
            dir_name: self.dir_name.clone(),
        }
    }
}

pub struct InputDir {
    base_dir: Dir,
    modify_dir: Dir,
    output_dir: Option<Dir>,
}

impl InputDir {
    pub fn new(base_dir: Dir, modify_dir: Dir) -> Self {
        Self {
            base_dir,
            modify_dir,
            output_dir: None,
        }
    }
    pub fn set_base_dir(&mut self, dir: Dir) {
        self.base_dir = dir;
    }
    pub fn set_modify_dir(&mut self, dir: Dir) {
        self.modify_dir = dir;
    }
    pub fn output_rename(&mut self, output_dir_path: PathBuf, is_move: bool) -> io::Result<()> {
        // check and move file to output dir
        self.output_dir = if are_same_directory(&self.modify_dir.dir_path, &output_dir_path)? {
            // use clone because modify_dir may be useful in further
            Some(Dir {
                files: self.modify_dir.files.clone(),
                dir_path: self.modify_dir.dir_path.clone(),
                dir_name: self.modify_dir.dir_name.clone(),
            })
        } else {
            // create a new Dir struct
            let mut output_dir = Dir {
                files: Vec::new(),
                dir_name: get_dir_name(&output_dir_path),
                dir_path: output_dir_path,
            };

            // copy or move files to new dir
            for file in &self.modify_dir.files {
                if let Some(file_name) = file.get_file_path().file_name() {
                    let output_file_path = output_dir.dir_path.join(file_name);
                    let operation_result = if is_move {
                        fs::rename(file.get_file_path(), &output_file_path)
                    } else {
                        fs::copy(file.get_file_path(), &output_file_path).map(|_| ())
                    };

                    match operation_result {
                        Ok(_) => {
                            let output_file = File::new(output_file_path)?;
                            output_dir.files.push(output_file);
                        }
                        Err(e) => {
                            eprintln!("文件操作失败: {e}, 文件: {:?}", file.get_file_path());
                            return Err(e);
                        }
                    }
                } else {
                    eprintln!("无法获取文件名: {:?}", file.get_file_path());
                }
            }

            Some(output_dir)
        };

        // start rename
        if self.base_dir.files.len()
            == self
                .output_dir
                .as_ref()
                .expect("output_dir未初始化，程序逻辑错误")
                .files
                .len()
        {
            batch_rename(
                &self.base_dir.files,
                &mut self.output_dir.as_mut().unwrap().files,
            );
        } else {
            eprintln!("重命名文件数量不一致，取消重命名");
        }
        Ok(())
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
