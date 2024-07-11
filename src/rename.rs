use std::{
    fs,
    io::Result,
    path::{Path, PathBuf},
};

use crate::file::File;

pub fn batch_rename(base_files: &[File], output_files: &mut [File]) -> () {
    // WARN: 文件需要重新排序吗？
    for (base_file, output_file) in base_files.iter().zip(output_files.iter_mut()) {
        println!("文件后缀读取: {}", output_file.get_file_ext());
        output_file.update_info(
            // TODO: 需要传入扩展名
            rename_file(
                output_file.get_file_path(),
                base_file.get_file_name(),
                output_file.get_file_ext(),
            )
            // 就不返回错误了, 传入参数已经经过校验
            .expect("重命名失败 -> rename_file"),
        )
    }
}
fn rename_file(old_path: &Path, new_name: &String, new_ext: &String) -> Result<PathBuf> {
    // TODO: 可能需要优化不必每次求父目录
    let parent = old_path.parent().unwrap_or(Path::new(""));
    // let old_name = old_path.file_name().map(|n| n.to_str().unwrap()).unwrap();

    let ext = new_ext;

    let new_file_name = format!("{}.{}", new_name, ext);

    let new_path = parent.join(new_file_name);
    fs::rename(old_path, &new_path)?;
    Ok(new_path)
}
