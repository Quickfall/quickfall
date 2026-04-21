use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: CLICommand,
}

#[derive(Clone, Copy, PartialEq, Eq, ValueEnum, Debug)]
pub enum Platform {
    AstoIR,
    LLVM,
}

#[derive(Subcommand)]
pub enum CLICommand {
    #[command(visible_alias = "b")]
    Build {
        #[arg(short = 'o')]
        out: String,

        #[arg(long, value_enum, default_value = "llvm")]
        platform: Platform,

        #[arg(required = true)]
        input: Vec<String>,
    },
}
