use crate::error::ArgsError;
use crate::file::File;
use std::{fmt::Display, path::PathBuf};

pub struct OrigDir {
    files: Vec<File>,
    // not need to edit
    dir_path: PathBuf,
    dir_name: String,
}
impl OrigDir {
    fn print_files(&self) {
        for (index, file) in self.files.iter().enumerate() {
            println!("{index} | {file}");
        }
    }
    fn new() -> Result<OrigDir, ArgsError> {}
}
impl Display for OrigDir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "目录路径 {}", self.dir_path.display())
            .and_then(|_| writeln!(f, "目录名   {}", self.dir_name))
            .and_then(|_| writeln!(f, "文件数   {}", self.files.len()))
    }
}
pub struct EditDir {
    files: Vec<File>,
    dir_path: PathBuf,
    dir_name: String,
}
impl EditDir {
    fn print_files(&self) {
        for (index, file) in self.files.iter().enumerate() {
            println!("{index} | {file}");
        }
    }
}
impl Display for EditDir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "目录路径 {}", self.dir_path.display())
            .and_then(|_| writeln!(f, "目录名   {}", self.dir_name))
            .and_then(|_| writeln!(f, "文件数   {}", self.files.len()))
    }
}
pub enum InputDir {
    OrigDir,
    EditDir,
}
impl InputDir {
    // print for dev
    fn print_type(&self) {
        match self {
            InputDir::OrigDir => println!("此为输入的基准目录"),
            InputDir::EditDir => println!("此为输入的修改目录"),
        }
    }
}
