use b_rename_cli::Args;
use b_rename_cli::InputDir;
use std::process;

fn main() {
    let args = match Args::new() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
    };
    let mut input_dir = match InputDir::new(args.base, args.modify, args.output) {
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
