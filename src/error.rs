use std::{fmt::Display, path::PathBuf};

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

// arg error

#[derive(Debug)]
pub enum ArgsError {
    InputDirNotFound(PathBuf),
    InputDirNotAccessible(PathBuf),
    OutputDirNotFound(PathBuf),
    OutputDirNotWritable(PathBuf),
    OutputDirNotCreatable(PathBuf),
    NotADir(PathBuf),
    InvalidArgument(String),
}

impl Display for ArgsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgsError::InputDirNotFound(path) => write!(
                f,
                "输入目录不存在: {}",
                path.to_str().unwrap_or("错误参数不存在")
            ),
            ArgsError::InputDirNotAccessible(path) => write!(
                f,
                "输入目录无法读取: {}",
                path.to_str().unwrap_or("错误参数不存在")
            ),
            ArgsError::OutputDirNotFound(path) => write!(
                f,
                "输出目录不存在: {}",
                path.to_str().unwrap_or("错误参数不存在")
            ),
            ArgsError::OutputDirNotWritable(path) => write!(
                f,
                "输出目录不可写: {}",
                path.to_str().unwrap_or("错误参数不存在")
            ),
            ArgsError::OutputDirNotCreatable(path) => {
                write!(
                    f,
                    "无法创建目录: {}",
                    path.to_str().unwrap_or("错误参数不存在")
                )
            }
            ArgsError::NotADir(path) => write!(
                f,
                "不是一个目录: {}",
                path.to_str().unwrap_or("错误参数不存在")
            ),
            ArgsError::InvalidArgument(msg) => {
                write!(f, "参数不存在: {}", msg)
            }
        }
    }
}

impl std::error::Error for ArgsError {}
