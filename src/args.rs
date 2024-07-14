use crate::error::ArgsError;

use clap::Parser;
use std::fs::{create_dir_all, remove_file, File};
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct RawArgs {
    #[arg(short = 'b', long, required = true)]
    base: PathBuf,

    #[arg(short = 'm', long, required = true)]
    modify: PathBuf,

    #[arg(short = 'o', long)]
    // if none will same with modify
    output: Option<PathBuf>,
    // TODO: if need other args
    // #[command(subcommand)]
    // pub command: Option<Command>,
}

pub struct Args {
    pub base: PathBuf,
    pub modify: PathBuf,
    pub output: PathBuf,
}

impl Args {
    pub fn new() -> Result<Self, ArgsError> {
        // init args
        let raw_args = RawArgs::parse();
        // test dir
        match raw_args.base.try_exists() {
            Ok(true) => {
                if !raw_args.base.is_dir() {
                    return Err(ArgsError::InvalidArgument(
                        "输入基准参数不是一个目录".to_string(),
                    ));
                }
            }
            Ok(false) => return Err(ArgsError::InputDirNotFound(raw_args.base.clone())),
            Err(_) => return Err(ArgsError::InputDirNotAccessible(raw_args.base.clone())),
        }
        match raw_args.modify.try_exists() {
            Ok(true) => {
                if !raw_args.modify.is_dir() {
                    return Err(ArgsError::InvalidArgument(
                        "输入修改路径不是一个目录".to_string(),
                    ));
                }
            }
            Ok(false) => return Err(ArgsError::OutputDirNotFound(raw_args.modify.clone())),
            Err(_) => return Err(ArgsError::InputDirNotAccessible(raw_args.modify.clone())),
        }

        let output = match &raw_args.output {
            Some(path) if path != &raw_args.modify && path != &raw_args.base => {
                // 创建目录, 如果存在不会有任何操作
                create_dir_all(path).map_err(|_| ArgsError::OutputDirNotCreatable(path.clone()))?;
                path.clone()
            }
            _ => raw_args.modify.clone(),
        };

        // test writable
        if !is_directory_writable(&output) {
            return Err(ArgsError::OutputDirNotWritable(output.clone()));
        }
        Ok(Args {
            base: raw_args.base,
            modify: raw_args.modify,
            output,
        })
    }
}

fn is_directory_writable(path: &Path) -> bool {
    let temp_file_path = path.join("temp_test_file");
    match File::create(&temp_file_path) {
        Ok(_) => {
            // create successes, remove temp file
            remove_file(&temp_file_path).ok();
            true
        }
        Err(_) => false,
    }
}
