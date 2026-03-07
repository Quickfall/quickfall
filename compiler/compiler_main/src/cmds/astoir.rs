use std::fs;

use ast_parser::parse_ast_ctx;
use astoir::{IRLevel, run_astoir_hir};
use compiler_errors::errs::{BaseResult, base::BaseError};
use lexer::lexer::lexer_parse_file;

pub fn parse_astoir_command(arguments: Vec<String>) {
	if arguments.len() <= 2 {
		println!("quickfall astoir <HIR> paths...");
		return;
	}

	let level = match parse_astoir_level(&arguments[2]) {
		Ok(v) => v,
		Err(_) => return
	};

	for i in 3..arguments.len() {
		let lexer = lexer_parse_file(&arguments[i]).unwrap();
		let ast = parse_ast_ctx(&lexer).unwrap();

		match level {
			IRLevel::HIR => {
				let ctx = run_astoir_hir(ast).unwrap();
				let res_path = arguments[i].clone() + ".qfhir";

				fs::write(res_path, format!("{:#?}", ctx)).unwrap()
			}
		}
	}

}

fn parse_astoir_level(str: &String) -> BaseResult<IRLevel> {
	match str as &str {
		"hir" | "HIR" | "h" | "H" => return Ok(IRLevel::HIR),

		_ => return Err(BaseError::critical("Cannot parse AstoIR level".to_string()))
	};

}