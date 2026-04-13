use std::fs;

use ast_parser::parse_ast_ctx;
use astoir::{IRLevel, run_astoir_hir, run_astoir_mir};
use diagnostics::{DiagnosticResult, dump_diagnostics};
use lexer::lexer::lexer_parse_file;

use std::process::exit;

#[cfg(feature = "llvm_ir_bridge")]
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

		dump_diagnostics();

		match level {
			IRLevel::HIR => {
				let ctx = run_astoir_hir(ast.unwrap());
				let res_path = arguments[i].clone() + ".qfhir";

				dump_diagnostics();

				fs::write(res_path, format!("{:#?}", ctx.unwrap())).unwrap()
			},

			IRLevel::MIR => {
				let ctx = run_astoir_mir(ast.unwrap());
				let res_path = arguments[i].clone() + ".qfmir";

				dump_diagnostics();

				fs::write(res_path, format!("{}", ctx.unwrap()));
			},

			IRLevel::LLVM => {
				#[cfg(feature = "llvm_ir_bridge")] {
					let ctx = run_astoir_mir(ast.unwrap());
					let res_path = arguments[i].clone() + ".llvm";
	
					dump_diagnostics();
	
					let ctx = bridge_llvm(&ctx.unwrap());
	
					dump_diagnostics();
	
					ctx.module.print_to_file(res_path);
				}

				#[cfg(not(feature = "llvm_ir_bridge"))] {
					println!("LLVM target is not bundled!");

					exit(0);
				}
			}
		}
	}

}

fn parse_astoir_level(str: &String) -> DiagnosticResult<IRLevel> {
	match str as &str {
		"hir" | "HIR" | "h" | "H" => return Ok(IRLevel::HIR),
		"mir" | "MIR" | "m" | "M" => return Ok(IRLevel::MIR),
		"llvm" | "LLVM" => return Ok(IRLevel::LLVM),

		_ => {
			println!("Invalid level");
			std::process::exit(0);
		}
	};

}