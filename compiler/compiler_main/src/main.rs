use std::env;

use crate::cmds::astoir::parse_astoir_command;

pub mod cmds;

fn main() {
	let arguments: Vec<String> = env::args().collect();

	if arguments.len() <= 1 {
		println!("Usage: quickfall comp|astoir");
		return;
	}

	match &arguments[1] as &str {
		"astoir" => {
			parse_astoir_command(arguments);
		},

		_ => {
			println!("Invalid subcommand!");
			return;
		}
	}
}