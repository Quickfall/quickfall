use inkwell::{context::Context, module::Module};
use ir::{irstruct::{funcs::IRFunction, ptr::IRPointer}, types::{UNSIGNED32_TYPE_HASH, storage::IRTypeStorage}, values::IRValue};
use parser::ast::func;

fn main() {
	//let args: Vec<String> = env::args().collect();
    //let file_path: &String = &args[1];

	//let lexer_res = lexer_parse_file(file_path).expect("Bad lexer!");

	//let ctx = parse_ast_ctx(&lexer_res);

	//println!("{:#?}", ctx);

	let context = Context::create();

	let storage = IRTypeStorage::new(&context);


	let module= context.create_module("main_module");
	let builder = context.create_builder();

	let t = storage.get(UNSIGNED32_TYPE_HASH).unwrap();

	let i32_type = context.i32_type();

	let func = IRFunction::create(&context, String::from("main"), &module, t, vec![t, t]).expect("Couldn't make IR function");

	func.prepare_body_filling(&builder);

	let ptr = IRPointer::create(&builder, String::from("test"), t, IRValue::Unsigned32(258)).unwrap();
	
	let val = ptr.load_val_int(&builder).unwrap();
	ptr.store(&builder, val);

	module.print_to_file("test.ll").unwrap();
}
