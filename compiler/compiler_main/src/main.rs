use std::env;

use compiler_utils::Position;
use diagnostics::diagnostic::{Diagnostic, Level, Span, SpanKind};

use crate::cmds::astoir::parse_astoir_command;

pub mod cmds;

fn main() {
	let arguments: Vec<String> = env::args().collect();

	let diag: Diagnostic = Diagnostic { level: Level::Error, code: 69420, message: "Femboys couldn't be allowed in here".to_string(), primary_span: Span {
		kind: SpanKind::Primary,
		label: "Femboy declared here".to_string(),
		start: Position { line: 1, col: 0, file_path: "./test.qf".to_string() },
		end_col: 5
	}, spans: vec![], note: vec!["It might be time to like femboys now?".to_string()], help: vec!["meoww :2".to_string()] };

	println!("{}", diag);

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