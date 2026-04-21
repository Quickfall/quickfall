use clap::Parser;

use crate::cli::{CLICommand, Cli};

pub mod cli;

fn main() {
    let cli = Cli::parse();
}
