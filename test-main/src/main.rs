use std::hash::{DefaultHasher, Hash, Hasher};

use inkwell::{context::Context, module::Module};
use ir::{ctx::IRContext, irstruct::{funcs::IRFunction, ptr::IRPointer, structs::IRStructuredType}, refs::IRValueRef, types::{POINTER_TYPE_HASH, SIGNED32_TYPE_HASH, UNSIGNED32_TYPE_HASH, storage::IRTypeStorage, typing::IRType}, values::IRValue};
use parser::ast::func;

fn main() {
	//let args: Vec<String> = env::args().collect();
    //let file_path: &String = &args[1];

	//let lexer_res = lexer_parse_file(file_path).expect("Bad lexer!");

	//let ctx = parse_ast_ctx(&lexer_res);

	//println!("{:#?}", ctx);

	let context = Context::create();

	let mut storage = IRTypeStorage::new(&context);

	let module= context.create_module("main_module");
	let builder = context.create_builder();

	let irctx = IRContext::new(builder, &context);

	let t = storage.get(UNSIGNED32_TYPE_HASH).unwrap();
	let int_type = storage.get(SIGNED32_TYPE_HASH).unwrap();
	let ptr_type = storage.get(POINTER_TYPE_HASH).unwrap();

	let sample_substruct = IRType::Struct(
		IRStructuredType::new(&irctx, String::from("uwuStruct"), true, vec![(8417845746417243860, int_type)]).unwrap()
	);

	let sample_struct = IRType::Struct(
		IRStructuredType::new(&irctx, String::from("myTestStruct"), true, vec![(8417845746417243860, &sample_substruct)]).unwrap()
	);

	//storage.insert(15869126390205824132, sample_struct);


	let i32_type = context.i32_type();
	
	let printf_func = IRFunction::create_shadow(String::from("printf"), &module, int_type, vec![ptr_type]).unwrap();

	let func = IRFunction::create(&context, String::from("main"), &module, t, vec![t, t]).expect("Couldn't make IR function");
	func.prepare_body_filling(&irctx.builder);

	let fmt_str = &irctx.builder.build_global_string_ptr("Haiiiii, the value is %d\n", "fmt_str").unwrap();

	// Struct test
	let structInstance = IRPointer::create(&irctx, String::from("test"), &sample_struct, None).unwrap();

	let subStructPtr = sample_struct.get_structured_type_descriptor().unwrap().get_pointer_for_field_index(&irctx, &structInstance, 0).unwrap();
	let subStructVarPtr = sample_substruct.get_structured_type_descriptor().unwrap().get_pointer_for_field_index(&irctx, &subStructPtr, 0).unwrap();

	subStructVarPtr.store(&irctx.builder, int_type.get_inkwell_inttype().unwrap().const_int(1288, false));

	let val = subStructVarPtr.load(&irctx, int_type).unwrap().obtain();

	//firstFieldPointer.store(&irctx.builder, int_type.get_inkwell_inttype().unwrap().const_int(125, false));
	
	// End struct test
	
	let _ = irctx.builder.build_call(
		printf_func.inkwell_func, 
   &[
			fmt_str.as_pointer_value().into(),
			val.into(),
		], 
		"printf_call"
	);

	irctx.builder.build_return(Some(&i32_type.const_zero()));

	module.print_to_file("output.ll").unwrap();

}
