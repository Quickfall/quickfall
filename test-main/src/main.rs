use std::{env};

use lexer::lexer::lexer_parse_file;
use parser::{ast::parse_ast_node, parse_ast_ctx};

fn main() {
	let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];

	let lexer_res = lexer_parse_file(file_path).expect("Bad lexer!");

	let ctx = parse_ast_ctx(&lexer_res);

	println!("{:#?}", ctx);

}
