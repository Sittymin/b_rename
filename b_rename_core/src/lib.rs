use std::{fs, io, path::PathBuf};

pub struct InputPath {
    pub base: PathBuf,
    pub modify: PathBuf,
    pub output: PathBuf,
}
impl InputPath {
    pub fn new(base: PathBuf, modify: PathBuf, output: PathBuf) -> io::Result<Self> {
        let input_path = InputPath {
            base,
            modify,
            output,
        };
        input_path.check_base()?;
        input_path.check_modify()?;
        input_path.check_output()?;
        Ok(input_path)
    }
    fn check_base(&self) -> io::Result<()> {
        check_path(&self.base)?;
        is_dir(&self.base)?;
        Ok(())
    }
    fn check_modify(&self) -> io::Result<()> {
        check_path(&self.modify)?;
        is_dir(&self.modify)?;
        Ok(())
    }
    fn check_output(&self) -> io::Result<()> {
        check_path(&self.output)?;
        is_dir(&self.output)?;
        create_dir_all(&self.output)?;
        is_directory_writable(&self.output)?;
        Ok(())
    }
}

fn check_path(path: &PathBuf) -> io::Result<()> {
    match path.try_exists() {
        Ok(true) => Ok(()),
        Ok(false) => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("路径不存在: {}", path.display()),
            ))
        }
        Err(e) => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("无法检查路径 {}: {}", path.display(), e),
            ))
        }
    }
}
fn is_dir(path: &PathBuf) -> io::Result<()> {
    if path.is_dir() {
        Ok(())
    } else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("路径非目录: {}", path.display()),
        ));
    }
}
fn create_dir_all(path: &PathBuf) -> io::Result<()> {
    match fs::create_dir_all(path) {
        Ok(()) => Ok(()),
        Err(e) => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("创建目录时发生错误: {e}"),
            ))
        }
    }
}
fn is_directory_writable(path: &PathBuf) -> io::Result<()> {
    let temp_file_path = path.join("temp_test_file");
    match fs::File::create(&temp_file_path) {
        Ok(_) => {
            // create successes, remove temp file
            fs::remove_file(&temp_file_path).ok();
            Ok(())
        }
        Err(_) => {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                format!("目录不可写: {}", path.display()),
            ))
        }
    }
}
