use std::time::Instant;

use clap::Parser;

use crate::{
    cli::{CLICommand, Cli},
    cmds::check::run_check,
    version::{GIT_HASH, VERSION},
};

pub mod cli;
pub mod cmds;
pub mod version;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        CLICommand::Version => {
            println!("Quickfall v{} (commit {})", VERSION, GIT_HASH);
        }

        CLICommand::Check { input, layer } => {
            let start = Instant::now();
            let count = input.len();

            for file in input {
                run_check(file, layer);
            }

            println!(
                "No problems could be found in the {} provided files! Checked in {:?}",
                count,
                start.elapsed()
            )
        }

        _ => todo!(),
    }
}
