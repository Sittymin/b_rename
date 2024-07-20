use std::{
    ffi::OsString,
    fs,
    io::Result,
    path::{Path, PathBuf},
};

use crate::file::File;

pub fn batch_rename(base_files: &[File], output_files: &mut [File]) {
    let mut error_occurred = false;

    for (base_file, output_file) in base_files.iter().zip(output_files.iter_mut()) {
        println!("文件后缀读取: {:?}", output_file.get_file_ext());

        match rename_file(
            output_file.get_file_path(),
            base_file.get_file_name(),
            output_file.get_file_ext(),
        ) {
            Ok(new_path) => {
                if let Err(e) = output_file.update_info(new_path) {
                    eprintln!("更新文件信息失败: {}", e);
                    error_occurred = true;
                }
            }
            Err(e) => {
                eprintln!("重命名文件失败: {}", e);
                error_occurred = true;
            }
        }
    }

    if error_occurred {
        eprintln!("批量重命名过程中发生了一个或多个错误");
    } else {
        println!("批量重命名成功完成");
    }
}
fn rename_file(old_path: &Path, new_name: &OsString, new_ext: &OsString) -> Result<PathBuf> {
    // TODO: 可能需要优化不必每次求父目录
    let parent = old_path.parent().unwrap_or(Path::new(""));

    let mut new_file_name = new_name.clone();
    new_file_name.push(".");
    new_file_name.push(new_ext);

    let new_path = parent.join(new_file_name);
    fs::rename(old_path, &new_path)?;
    Ok(new_path)
}
