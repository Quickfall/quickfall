use inkwell::{context::Context, module::Module};
use ir::{types::{UNSIGNED32_TYPE_HASH, storage::IRTypeStorage}, values::IRValue};
use parser::ast::func;

fn main() {
	//let args: Vec<String> = env::args().collect();
    //let file_path: &String = &args[1];

	//let lexer_res = lexer_parse_file(file_path).expect("Bad lexer!");

	//let ctx = parse_ast_ctx(&lexer_res);

	//println!("{:#?}", ctx);

	let context = Context::create();
	let module= context.create_module("main_module");
	let builder = context.create_builder();

	let storage = IRTypeStorage::new(&context);

	let i32_type = context.i32_type();

	let fn_type = i32_type.fn_type(&[], false);
	let function = module.add_function("main", fn_type, None);

	let entry = context.append_basic_block(function, "entry");
	builder.position_at_end(entry);

	let testvar= match storage.get(UNSIGNED32_TYPE_HASH).unwrap().make_numeric_stackvar(&builder, String::from("test_var"), IRValue::Unsigned32(44575445_u32)) {
		Ok(v) => v,
		Err(_) => panic!("Stop")
	};

	module.print_to_file("test.ll").unwrap();
}
