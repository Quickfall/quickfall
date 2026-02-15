use std::hash::{DefaultHasher, Hash, Hasher};

use inkwell::{context::Context, module::Module};
use ir::{irstruct::{funcs::IRFunction, ptr::IRPointer}, types::{POINTER_TYPE_HASH, SIGNED32_TYPE_HASH, UNSIGNED32_TYPE_HASH, storage::IRTypeStorage, typing::IRType}, values::{IRValue}};
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
	let int_type = storage.get(SIGNED32_TYPE_HASH).unwrap();
	let ptr_type = storage.get(POINTER_TYPE_HASH).unwrap();


	let i32_type = context.i32_type();
	
	let printf_func = IRFunction::create_shadow(String::from("printf"), &module, int_type, vec![ptr_type]).unwrap();

	let func = IRFunction::create(&context, String::from("main"), &module, t, vec![t, t]).expect("Couldn't make IR function");
	func.prepare_body_filling(&builder);


	let fmt_str = builder.build_global_string_ptr("Haiiiii, the value is %d\n", "fmt_str").unwrap();

	let ptr = IRPointer::create(&builder, String::from("test"), t, IRValue::from_unsigned(t, 286).unwrap()).unwrap();
	
	let val = ptr.load_val_int(&builder).unwrap();

	let _ = builder.build_call(
		printf_func.inkwell_func, 
   &[
			fmt_str.as_pointer_value().into(),
			val.into(),
		], 
		"printf_call"
);

	builder.build_return(Some(&i32_type.const_zero()));

	module.print_to_file("output.ll").unwrap();

}
