use b_rename_cli::args::RawArgs;
use b_rename_core::dir::InputDir;
use std::process;

fn main() {
    let args = match RawArgs::new() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
    };
    let mut input_dir = match InputDir::new(args.base, args.modify, args.output, false) {
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
