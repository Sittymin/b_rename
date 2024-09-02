use b_rename_cli::args::RawArgs;
use b_rename_core::dir::{Dir, InputDir};
use std::process;

fn main() {
    let args = match RawArgs::new() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
    };
    let mut base_dir = Dir::new(args.base);
    let mut modify_dir = Dir::new(args.modify);
    base_dir.sort_files_by_name_unstable();
    modify_dir.sort_files_by_name_unstable();

    let mut input_dir = InputDir::new(base_dir, modify_dir);
    // println!("{input_dir}");
    // 重命名
    match input_dir.output_rename(args.output, false) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{e}");
            // TEMP: if get this need back to set dir
            process::exit(1);
        }
    };
}
