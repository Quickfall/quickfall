use std::fs;

use ast_parser::parse_ast_ctx;
use astoir::{IRLevel, run_astoir_hir, run_astoir_mir};
use compiler_errors::errs::{BaseResult, base::BaseError, dump_errors};
use lexer::lexer::lexer_parse_file;
use llvm_ir_bridge::bridge_llvm;

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
		let ast = parse_ast_ctx(&lexer);

		dump_errors();

		match level {
			IRLevel::HIR => {
				let ctx = run_astoir_hir(ast.unwrap());
				let res_path = arguments[i].clone() + ".qfhir";

				dump_errors();

				fs::write(res_path, format!("{:#?}", ctx.unwrap())).unwrap()
			},

			IRLevel::MIR => {
				let ctx = run_astoir_mir(ast.unwrap());
				let res_path = arguments[i].clone() + ".ll";

				dump_errors();

				//let bridge = bridge_llvm(&ctx.unwrap());

				dump_errors();

				fs::write(res_path, format!("{}", ctx.unwrap()));

				//let ctx = bridge.unwrap();

				//ctx.module.print_to_file(res_path).unwrap();
			}
		}
	}

}

fn parse_astoir_level(str: &String) -> BaseResult<IRLevel> {
	match str as &str {
		"hir" | "HIR" | "h" | "H" => return Ok(IRLevel::HIR),
		"mir" | "MIR" | "m" | "M" => return Ok(IRLevel::MIR),

		_ => return Err(BaseError::critical("Cannot parse AstoIR level".to_string()))
	};

}