use std::{
    env,
    fs::{self, create_dir_all, remove_dir_all},
    io,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

#[test]
fn test_files_rename_with_move() -> io::Result<()> {
    test_files_rename(true)
}
#[test]
fn test_files_rename_with_copy() -> io::Result<()> {
    test_files_rename(false)
}

fn create_temp_dir() -> io::Result<PathBuf> {
    let mut temp_dir = env::temp_dir();
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_nanos();
    temp_dir.push(format!("test_b_rename_{}", timestamp));
    create_dir_all(&temp_dir)?;
    Ok(temp_dir)
}
fn clean_temp_dir(path: &Path) -> io::Result<()> {
    remove_dir_all(path)
}
fn test_files_rename(is_move: bool) -> io::Result<()> {
    let base_dir_path = create_temp_dir()?;
    let modify_dir_path = create_temp_dir()?;
    let output_dir_path = create_temp_dir()?;

    fs::write(base_dir_path.join("file1.mkv"), "shit!")?;
    fs::write(base_dir_path.join("file2.mkv"), "shit!")?;
    fs::write(base_dir_path.join("file3.mkv"), "shit!")?;

    fs::write(modify_dir_path.join("modify_file1.ass"), "Oh! shit!")?;
    fs::write(modify_dir_path.join("modify_file2.ass"), "Oh! shit!")?;
    fs::write(modify_dir_path.join("modify_file3.ass"), "Oh! shit!")?;

    let base_dir = b_rename_core::dir::Dir::new(base_dir_path.clone());
    let modify_dir = b_rename_core::dir::Dir::new(modify_dir_path.clone());
    let mut input_dir = b_rename_core::dir::InputDir::new(base_dir, modify_dir);
    input_dir.output_rename(output_dir_path.clone(), is_move)?;
    assert!(output_dir_path.join("file1.ass").exists());
    assert!(output_dir_path.join("file2.ass").exists());
    assert!(output_dir_path.join("file3.ass").exists());
    assert!(!output_dir_path.join("modify_file1.ass").exists());
    assert!(!output_dir_path.join("modify_file2.ass").exists());
    assert!(!output_dir_path.join("modify_file3.ass").exists());

    clean_temp_dir(&base_dir_path)?;
    clean_temp_dir(&modify_dir_path)?;
    clean_temp_dir(&output_dir_path)?;
    Ok(())
}
