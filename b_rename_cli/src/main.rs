use b_rename_cli::args::RawArgs;
use b_rename_core::dir::{InputDir, Dir};
use std::process;

fn main() {
    let args = match RawArgs::new() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
    };
    let base_dir = Dir::new(args.base);
    let modify_dir = Dir::new(args.modify);
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
