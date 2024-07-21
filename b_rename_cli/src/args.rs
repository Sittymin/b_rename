use b_rename_core::path::InputPath;

use clap::Parser;
use std::{io, path::PathBuf};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct RawArgs {
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

impl RawArgs {
    pub fn new() -> io::Result<InputPath> {
        let raw_args = RawArgs::parse();
        let output = match raw_args.output {
            Some(output) => output,
            None => raw_args.modify.clone(),
        };
        InputPath::new(raw_args.base, raw_args.modify, output)
    }
}
