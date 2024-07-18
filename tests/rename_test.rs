use std::{
    env,
    fs::{self, create_dir_all, remove_dir_all},
    io,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

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
#[test]
fn test_files_rename() -> io::Result<()> {
    let base_dir = create_temp_dir()?;
    let modify_dir = create_temp_dir()?;
    let output_dir = create_temp_dir()?;

    fs::write(base_dir.join("file1.mkv"), "shit!")?;
    fs::write(base_dir.join("file2.mkv"), "shit!")?;
    fs::write(base_dir.join("file3.mkv"), "shit!")?;

    fs::write(modify_dir.join("modify_file1.ass"), "Oh! shit!")?;
    fs::write(modify_dir.join("modify_file2.ass"), "Oh! shit!")?;
    fs::write(modify_dir.join("modify_file3.ass"), "Oh! shit!")?;

    let mut input_dir =
        b_rename::InputDir::new(base_dir.clone(), modify_dir.clone(), output_dir.clone())?;
    input_dir.output_rename();
    assert!(output_dir.join("file1.ass").exists());
    assert!(output_dir.join("file2.ass").exists());
    assert!(output_dir.join("file3.ass").exists());

    clean_temp_dir(&base_dir)?;
    clean_temp_dir(&modify_dir)?;
    clean_temp_dir(&output_dir)?;
    Ok(())
}
