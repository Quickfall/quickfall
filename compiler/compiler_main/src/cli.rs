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

#[derive(Clone, Copy, PartialEq, Eq, ValueEnum, Debug)]
pub enum IRLayer {
    HIR,
    MIR,
}

#[derive(Clone, Copy, PartialEq, Eq, ValueEnum, Debug)]
pub enum OutputFormat {
    #[value(alias = "exec")]
    Executable,

    #[value(alias = "o")]
    Object,

    IR,
}

#[derive(Subcommand)]
pub enum CLICommand {
    #[command(visible_alias = "b", about = "Builds the given file(s)")]
    Build {
        #[arg(short = 'o')]
        out: String,

        #[arg(long, value_enum, default_value = "llvm")]
        platform: Platform,

        #[arg(short = 't', value_enum, long, default_value = "o")]
        format: OutputFormat,

        #[arg(short = 'l', default_value = "ld")]
        linker: String,

        #[arg(required = true)]
        input: Vec<String>,
    },

    #[command(visible_alias = "ver", about = "Displays the version")]
    Version,

    #[command(about = "Checks if a file is valid")]
    Check {
        #[arg(required = true)]
        input: Vec<String>,

        #[arg(long, value_enum, default_value = "mir")]
        layer: IRLayer,
    },
}
