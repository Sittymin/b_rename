use std::fmt::Display;

// file error

#[derive(Debug)]
pub enum FileError {
    ExtError(String),
    NameError(String),
    PathError(String),
}

impl Display for FileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileError::ExtError(e) => write!(f, "文件后缀错误: {e}"),
            FileError::NameError(e) => write!(f, "文件名错误: {e}"),
            FileError::PathError(e) => write!(f, "文件路径错误: {e}"),
        }
    }
}

impl std::error::Error for FileError {}
