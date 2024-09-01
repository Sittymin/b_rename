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

    let mut input_dir = match InputDir::new(base_dir, modify_dir, args.output, false) {
        Ok(input_dir) => input_dir,
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
    };
    // println!("{input_dir}");
    // 重命名
    input_dir.output_rename();
}
