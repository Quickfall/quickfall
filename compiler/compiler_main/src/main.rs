use clap::Parser;

use crate::{
    cli::{CLICommand, Cli},
    version::{GIT_HASH, VERSION},
};

pub mod cli;
pub mod version;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        CLICommand::Version => {
            println!("Quickfall v{} (commit {})", VERSION, GIT_HASH);
        }

        _ => todo!(),
    }
}
