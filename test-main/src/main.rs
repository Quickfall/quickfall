use std::{env};

use lexer::lexer::lexer_parse_file;
use parser::ast::parse_ast_node;

fn main() {
	let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];

	let lexer_res = lexer_parse_file(file_path).expect("Bad lexer!");

	let mut ind = 0;

	let parser = parse_ast_node(&lexer_res, &mut ind);

	println!("Parse result: {:#?}", parser.expect("E"));
}
