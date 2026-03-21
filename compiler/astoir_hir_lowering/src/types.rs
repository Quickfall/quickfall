use ast::types::ASTType;
use astoir_hir::ctx::HIRContext;
use compiler_errors::{IR_EXPECTED_SIZE_SPECIFIED, errs::{BaseResult, base::BaseError}};
use compiler_typing::tree::Type;
use compiler_utils::hash::HashedString;

pub fn lower_ast_type(context: &HIRContext, t: ASTType) -> BaseResult<Type> {
	return match t {
		ASTType::Generic(type_id, type_params, size_params) => {
			let hash = HashedString::new(type_id).hash;

			let t = context.type_storage.get_type(hash)?;

			let mut t_params = vec![];

			for type_param in type_params {
				t_params.push(lower_ast_type(context, *type_param)?);
			}

			Ok(Type::Generic(, (), ()))
		
		},

		ASTType::Pointer(array, inner) => Ok(Type::Pointer(array, Box::new(lower_ast_type(context, *inner)?))),
		ASTType::Array(size, inner) => Ok(Type::Array(size, Box::new(lower_ast_type(context, *inner)?)))
	};
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