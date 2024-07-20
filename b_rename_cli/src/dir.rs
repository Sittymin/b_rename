use crate::{file::File, rename::batch_rename};
use std::{ffi::OsString, fs, io, path::PathBuf};

struct Dir {
    files: Vec<File>,
    dir_path: PathBuf,
    dir_name: OsString,
}
pub struct InputDir {
    base_dir: Dir,
    // modify_dir: Dir,
    output_dir: Dir,
}
// impl fmt::Display for InputDir {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         writeln!(f, "InputDir:")?;

//         // 显示BaseDir信息
//         writeln!(f, "  BaseDir:")?;
//         writeln!(f, "    路径: {}", self.base_dir.dir_path.display())?;
//         writeln!(f, "    名称: {}", self.base_dir.dir_name)?;
//         writeln!(f, "    文件列表:")?;
//         for file in &self.base_dir.files {
//             write!(f, "      {}", file)?;
//         }

//         // 显示ModifyDir信息
//         writeln!(f, "  ModifyDir:")?;
//         writeln!(f, "    路径: {}", self.modify_dir.dir_path.display())?;
//         writeln!(f, "    名称: {}", self.modify_dir.dir_name)?;
//         writeln!(f, "    文件列表:")?;
//         for file in &self.modify_dir.files {
//             write!(f, "      {}", file)?;
//         }

//         // 显示OutputDir信息
//         writeln!(f, "  OutputDir:")?;
//         writeln!(f, "    路径: {}", self.output_dir.dir_path.display())?;
//         writeln!(f, "    名称: {}", self.output_dir.dir_name)?;
//         writeln!(f, "    文件列表:")?;
//         for file in &self.output_dir.files {
//             write!(f, "      {}", file)?;
//         }

//         Ok(())
//     }
// }

impl InputDir {
    pub fn new(
        base_dir_path: PathBuf,
        modify_dir_path: PathBuf,
        output_dir_path: PathBuf,
    ) -> io::Result<Self> {
        let base_dir = create_dir_struct(base_dir_path);
        let modify_dir = create_dir_struct(modify_dir_path);
        let output_dir = if are_same_directory(&modify_dir.dir_path, &output_dir_path)? {
            // 如果修改目录和输出目录相同，直接使用修改目录的信息
            Dir {
                files: modify_dir.files.clone(),
                dir_path: modify_dir.dir_path.clone(),
                dir_name: modify_dir.dir_name.clone(),
            }
        } else {
            // 创建新的输出目录结构
            let mut output_dir = Dir {
                files: Vec::new(),
                dir_name: get_dir_name(&output_dir_path),
                dir_path: output_dir_path,
            };

            // 复制修改目录中的文件到输出目录
            for file in &modify_dir.files {
                // 以下 file_name 包含后缀
                let file_name = file.get_file_path().file_name().unwrap();
                let output_file_path = output_dir.dir_path.join(PathBuf::from(file_name));
                fs::copy(file.get_file_path(), &output_file_path)?;

                // 创建新的 File 对象并添加到输出目录的文件列表中
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

fn create_dir_struct(dir_path: PathBuf) -> Dir {
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

fn are_same_directory(path1: &PathBuf, path2: &PathBuf) -> std::io::Result<bool> {
    let canonical1 = path1.canonicalize()?;
    let canonical2 = path2.canonicalize()?;
    Ok(canonical1 == canonical2)
}

// arg.rs 已经检查过路径合法性了
fn get_dir_name(path: &PathBuf) -> OsString {
    path.components()
        .last()
        .expect("读取目录名失败 -> get_dir_name")
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
