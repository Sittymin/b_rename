use crate::{file::File, rename::batch_rename};
use std::{fmt, fs, io, path::PathBuf};

struct BaseDir {
    files: Vec<File>,
    // not need to edit
    dir_path: PathBuf,
    dir_name: String,
}
struct ModifyDir {
    files: Vec<File>,
    dir_path: PathBuf,
    dir_name: String,
}
struct OutputDir {
    files: Vec<File>,
    dir_path: PathBuf,
    dir_name: String,
}
pub struct InputDir {
    base_dir: BaseDir,
    modify_dir: ModifyDir,
    output_dir: OutputDir,
}
impl fmt::Display for InputDir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "InputDir:")?;

        // 显示BaseDir信息
        writeln!(f, "  BaseDir:")?;
        writeln!(f, "    路径: {}", self.base_dir.dir_path.display())?;
        writeln!(f, "    名称: {}", self.base_dir.dir_name)?;
        writeln!(f, "    文件列表:")?;
        for file in &self.base_dir.files {
            write!(f, "      {}", file)?;
        }

        // 显示ModifyDir信息
        writeln!(f, "  ModifyDir:")?;
        writeln!(f, "    路径: {}", self.modify_dir.dir_path.display())?;
        writeln!(f, "    名称: {}", self.modify_dir.dir_name)?;
        writeln!(f, "    文件列表:")?;
        for file in &self.modify_dir.files {
            write!(f, "      {}", file)?;
        }

        // 显示OutputDir信息
        writeln!(f, "  OutputDir:")?;
        writeln!(f, "    路径: {}", self.output_dir.dir_path.display())?;
        writeln!(f, "    名称: {}", self.output_dir.dir_name)?;
        writeln!(f, "    文件列表:")?;
        for file in &self.output_dir.files {
            write!(f, "      {}", file)?;
        }

        Ok(())
    }
}

impl InputDir {
    pub fn new(
        base_dir_path: PathBuf,
        modify_dir_path: PathBuf,
        output_dir_path: PathBuf,
    ) -> io::Result<Self> {
        let base_dir = BaseDir {
            files: read_dir(&base_dir_path)?,
            dir_path: base_dir_path.clone(),
            dir_name: get_dir_name(&base_dir_path)
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "无效的基础目录路径"))?,
        };
        let modify_dir = ModifyDir {
            files: read_dir(&modify_dir_path)?,
            dir_path: modify_dir_path.clone(),
            dir_name: get_dir_name(&modify_dir_path)
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "无效的修改目录路径"))?,
        };
        let output_dir = if are_same_directory(&modify_dir_path, &output_dir_path)? {
            // 如果修改目录和输出目录相同，直接使用修改目录的信息
            OutputDir {
                files: modify_dir.files.clone(),
                dir_path: modify_dir.dir_path.clone(),
                dir_name: modify_dir.dir_name.clone(),
            }
        } else {
            // 创建新的输出目录结构
            let mut output_dir = create_output_dir(&output_dir_path)?;

            // 确保输出目录存在
            fs::create_dir_all(&output_dir.dir_path)?;

            // 复制修改目录中的文件到输出目录
            for file in &modify_dir.files {
                // 以下 file_name 包含后缀
                let file_name = format!("{}.{}", file.get_file_name(), file.get_file_ext());
                let output_file_path = output_dir.dir_path.join(&file_name);
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
            modify_dir,
            output_dir,
        })
    }
    pub fn output_rename(&mut self) {
        batch_rename(&self.base_dir.files, &mut self.output_dir.files);
    }
}

fn create_output_dir(path: &PathBuf) -> io::Result<OutputDir> {
    Ok(OutputDir {
        files: Vec::new(),
        dir_path: path.clone(),
        dir_name: get_dir_name(path).ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidInput, format!("无效的输出目录路径"))
        })?,
    })
}

fn are_same_directory(path1: &PathBuf, path2: &PathBuf) -> std::io::Result<bool> {
    let canonical1 = path1.canonicalize()?;
    let canonical2 = path2.canonicalize()?;
    Ok(canonical1 == canonical2)
}

fn get_dir_name(path: &PathBuf) -> Option<String> {
    path.components()
        .last()
        .and_then(|comp| comp.as_os_str().to_str())
        .map(|s| s.to_string())
}

fn read_dir(path: &PathBuf) -> io::Result<Vec<File>> {
    Ok(fs::read_dir(path)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.is_file() {
                match File::new(path) {
                    Ok(file) => Some(file),
                    Err(e) => {
                        eprintln!("读取目录文件错误: {}", e);
                        None
                    }
                }
            } else {
                None
            }
        })
        .collect::<Vec<File>>())
}
