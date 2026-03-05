use ast::types::CompleteType;
use astoir_hir::ctx::HIRContext;
use astoir_typing::complete::{ComplexType, ConcreteType};
use compiler_errors::errs::{BaseResult};

pub fn lower_ast_type(context: &HIRContext, t: CompleteType) -> BaseResult<ComplexType> {
	let hir_type = context.type_storage.get_type(t.base_type)?;
	let mut type_params = vec![];
	
	for type_param in t.types {
		type_params.push(context.type_storage.get_type(type_param)?.0)
	}


	let concrete = ConcreteType { base: hir_type.1.clone(), pointer: t.pointer, pointer_array: t.pointer_array, type_params, size_params: t.sizes.clone() };
	let complex = Box::new(ComplexType::Concrete(concrete));

	if t.array_sz > 0 {
		return Ok(ComplexType::Array(complex))
	} else {
		return Ok(*complex);
	}
}