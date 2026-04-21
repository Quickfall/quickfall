use std::{
    fs,
    path::{Path, PathBuf},
    process::Output,
    time::Instant,
};

use clap::Parser;

use crate::{
    cli::{CLICommand, Cli, OutputFormat, Platform},
    cmds::{build::build_mir, check::run_check},
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

        CLICommand::Build {
            out,
            platform,
            format,
            linker,
            input,
        } => {
            let needs_dir = input.len() > 1 && format != OutputFormat::Executable;

            if input.len() > 1 && out.extension().is_some() && format != OutputFormat::Executable {
                soft_panic!(
                    "Output must be a directory if theres more than one input and that the target isn't an executable"
                );
            }

            if needs_dir && !out.exists() {
                fs::create_dir_all(out.clone()).unwrap();
            }

            if platform == Platform::AstoIR && format != OutputFormat::IR {
                soft_panic!("Only IR target is supported by AstoIR platform!");
            }

            match platform {
                Platform::AstoIR => {
                    for i in input {
                        let mut outfile = PathBuf::from(i.file_name().unwrap());
                        outfile.add_extension("air");

                        let output_path = out.join(outfile);

                        build_mir(i.to_str().unwrap().to_string(), output_path);
                    }
                }

                _ => todo!(),
            }
        }

        _ => todo!(),
    }
}
