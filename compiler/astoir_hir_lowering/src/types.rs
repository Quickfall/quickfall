use ast::types::CompleteType;
use astoir_hir::ctx::HIRContext;
use astoir_typing::{base::BaseType, complete::{ComplexType, ConcreteType}};
use compiler_errors::{IR_EXPECTED_SIZE_SPECIFIED, errs::{BaseResult, base::BaseError}};

pub fn lower_ast_type(context: &HIRContext, t: CompleteType) -> BaseResult<ComplexType> {
	for key in context.type_storage.hash_to_ind.keys() {
		println!("- {}", key.hash);
	}

	println!("Seeking {}", t.base_type);

	let hir_type = context.type_storage.get_type(t.base_type)?;

	let mut type_params = vec![];
	
	for type_param in t.types {
		type_params.push(context.type_storage.get_type(type_param)?.0)
	}

	let actual_base;

	if hir_type.1.is_incomplete() {
		actual_base = lower_sized_base_type(context, hir_type.1, &t.sizes)?;

	} else {
		actual_base = hir_type.1.clone();
	}

	let concrete = ConcreteType { base: actual_base, pointer: t.pointer, pointer_array: t.pointer_array, type_params, size_params: t.sizes.clone() };
	let complex = Box::new(ComplexType::Concrete(concrete));

	if t.array_sz > 0 {
		return Ok(ComplexType::Array(complex))
	} else {
		return Ok(*complex);
	}
}

pub fn lower_sized_base_type(_context: &HIRContext, t: &BaseType, size_params: &Vec<usize>) -> BaseResult<BaseType> {
	match t {
		BaseType::IncompleteNumericType(e) => {
			if size_params.is_empty() {
				return Err(BaseError::err(IR_EXPECTED_SIZE_SPECIFIED!().to_string()));
			}

			return Ok(BaseType::NumericIntegerType(size_params[0] as u64, *e))
		},

		BaseType::IncompleteFloatingType(e) => {
			if size_params.len() < 2 {
				return Err(BaseError::err(IR_EXPECTED_SIZE_SPECIFIED!().to_string()));
			}

			return Ok(BaseType::FloatingNumberType(size_params[0] as u64, size_params[1] as u64, *e))
		},

		BaseType::IncompleteFixedPointType(e) => {
			if size_params.len() < 2 {
				return Err(BaseError::err(IR_EXPECTED_SIZE_SPECIFIED!().to_string()));
			}

			return Ok(BaseType::FixedPointNumberType(size_params[0] as u64, size_params[1] as u64, *e));
		}, 

		BaseType::IncompleteArbitraryType => {
			if size_params.is_empty() {
				return Err(BaseError::err(IR_EXPECTED_SIZE_SPECIFIED!().to_string()));
			}

			return Ok(BaseType::ArbitraryType(size_params[0] as u64));
		},

		_ => return Err(BaseError::err("Cannot lower sized base type!".to_string()))
	}
}