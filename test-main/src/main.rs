use core::fmt;
use std::{env, hash::{DefaultHasher, Hash, Hasher}, rc::Rc};

use inkwell::{context::Context, module::Module};
use ir::{conv::parse_ir_node_toplevel, ctx::IRContext, irstruct::{funcs::IRFunction, ptr::IRPointer, staticvars::IRStaticVariable, structs::IRStructuredType}, refs::IRValueRef, types::{POINTER_TYPE_HASH, SIGNED32_TYPE_HASH, UNSIGNED32_TYPE_HASH, storage::IRTypeStorage, typing::IRType}, values::IRValue};
use lexer::lexer::lexer_parse_file;
use parser::{ast::func, parse_ast_ctx};

fn main() {
	let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];

	let lexer_res = lexer_parse_file(file_path).expect("Bad lexer!");

	let ctx = parse_ast_ctx(&lexer_res).unwrap();

	println!("{:#?}", ctx);

	let context = Rc::new(Context::create());
	
	let mut irctx = IRContext::new(context);

	for entry in ctx.iter_order {
		parse_ir_node_toplevel(&mut irctx, ctx.map.get(&entry).unwrap().clone()).unwrap();
	}

	irctx.module.print_to_file("output.ll").unwrap();

}
