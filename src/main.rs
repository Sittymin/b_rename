use std::ffi::OsStr;
use std::io::{self, ErrorKind};
use std::path::{Path, PathBuf};
use std::process;
use std::{env, fs};

// 先是视频文件夹 然后是字幕文件夹

fn main() {
    let config = Config::build_path(env::args().skip(1)).unwrap_or_else(|err| {
        eprintln!("参数错误：{err}");
        process::exit(1);
    });

    let ref_dir_path = Path::new(&config.ref_dir_path);
    let edit_dir_path = Path::new(&config.edit_dir_path);

    let file_path = FilePath::new(ref_dir_path, edit_dir_path).unwrap_or_else(|err| {
        eprintln!("文件路径错误：{}", err);
        process::exit(1);
    });

    // println!("以下是file_path:\n{:#?}", file_path);
    let ref_names = file_path.get_file_names(&file_path.ref_file_paths);
    println!("{:?}", file_path.edit_file_paths);
    if let Ok(file_names) = ref_names {
        for (file_name, edit_file_name) in file_names.iter().zip(&file_path.edit_file_paths) {
            // println!("文件名为：{:#?}", file_name);
            let mut file_path = PathBuf::from(edit_dir_path);
            let file_name = Path::new(file_name);
            if let Some(file_name) = file_name.file_stem() {
                let file_name = Path::new(file_name).with_extension("ass");
                file_path.push(file_name);
                // println!("目标PathBuf：{:?}", file_path);
                // println!("原始PathBuf：{:?}", edit_file_name);
                if let Ok(msg) = fs::rename(edit_file_name, file_path) {
                    println!("成功{:?}", msg);
                } else {
                    eprintln!("失败")
                }
            }
        }
    } else {
        eprintln!("循环文件输出出现问题");
    }
}

// #[derive(Debug)]
struct FilePath {
    ref_file_paths: Vec<PathBuf>,
    edit_file_paths: Vec<PathBuf>,
}

impl FilePath {
    fn new(ref_dir_path: &Path, edit_dir_path: &Path) -> Result<FilePath, io::Error> {
        Ok(FilePath {
            ref_file_paths: Self::visit_dir(ref_dir_path)?,
            edit_file_paths: Self::visit_dir(edit_dir_path)?,
        })
    }

    fn visit_dir(dir: &Path) -> Result<Vec<PathBuf>, io::Error> {
        if !dir.is_dir() {
            return Err(io::Error::new(
                ErrorKind::NotFound,
                "指定的路径不是一个目录或目录不存在",
            ));
        }

        // 依次push到Vec
        // TODO: 可能文件夹中没有文件
        let mut file_paths = Vec::new();
        for entry in fs::read_dir(dir)? {
            // TODO: 可能无文件在这里处理
            let entry = entry?;
            let path = entry.path();
            // 过滤文件夹
            if path.is_file() {
                file_paths.push(path);
            }
        }
        // 按照文件名排序
        file_paths.sort_by(|a, b| {
            let name_a = a.file_name().unwrap();
            let name_b = b.file_name().unwrap();
            name_a.cmp(name_b)
        });

        Ok(file_paths)
    }

    fn get_file_names<'a>(&'a self, file_paths: &'a [PathBuf]) -> io::Result<Vec<&'a OsStr>> {
        let mut result = Vec::new();
        for path in file_paths {
            if let Some(file_name) = path.file_name() {
                // println!("文件名: {}", file_name.to_string_lossy());
                result.push(file_name);
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "路径不是一个文件",
                ));
            }
        }
        Ok(result)
    }
}

struct Config {
    ref_dir_path: String,
    edit_dir_path: String,
}

impl Config {
    pub fn build_path(mut args: impl Iterator<Item = String>) -> Result<Config, io::Error> {
        let ref_dir_path = args
            .next()
            .ok_or_else(|| io::Error::new(ErrorKind::InvalidInput, "没有第一个参数"))?;
        let edit_dir_path = args
            .next()
            .ok_or_else(|| io::Error::new(ErrorKind::InvalidInput, "没有第二个参数"))?;

        Ok(Config {
            ref_dir_path,
            edit_dir_path,
        })
    }
}
