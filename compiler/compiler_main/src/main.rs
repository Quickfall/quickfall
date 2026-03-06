use std::env;

fn main() {
	let arguments: Vec<String> = env::args().collect();

	if arguments.is_empty() {
		println!("Usage: quickfall comp|astoir");
		return;
	}

	match &arguments[0] as &str {
		"astoir" => {
			handle
		},

		_ => {
			println!("Invalid subcommand!");
			return;
		}
	}
}