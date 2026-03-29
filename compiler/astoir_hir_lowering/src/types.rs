use ast::types::ASTType;
use astoir_hir::ctx::HIRContext;
use compiler_errors::{IR_EXPECTED_SIZE_SPECIFIED, TYPE_TYPE_PARAMETERS, errs::{BaseResult, base::BaseError}};
use compiler_typing::{raw::RawType, references::TypeReference, structs::RawStructTypeContainer, tree::Type};
use compiler_utils::hash::HashedString;

pub fn lower_ast_type(context: &mut HIRContext, t: ASTType) -> BaseResult<Type> {
	return match t {
		ASTType::Generic(type_id, type_params, size_params) => {
			let hash = HashedString::new(type_id).hash;

			let t = context.type_storage.get_type(hash)?;

			if t.get_type_params_count(&context.type_storage) != type_params.len() {
				return Err(BaseError::err(TYPE_TYPE_PARAMETERS!().to_string()))
			}

			let mut t_params = vec![];

			for type_param in type_params {
				t_params.push(Box::new(lower_ast_type(context, *type_param)?));
			}

			let res = Type::Generic(context.type_storage.types.hash_to_ind[&hash], t_params, size_params);
			
			if t.is_sized() {
				let lower = lower_sized_base_type(context, &res)?;

				if context.type_storage.type_to_ind.contains_key(&lower) {
					return Ok(Type::Generic(context.type_storage.type_to_ind[&lower], vec![], vec![]));
				} else {
					let ind = context.type_storage.append_with_hash(hash, lower)?;

					return Ok(Type::Generic(ind, vec![], vec![]))
				}
			}

			return Ok(res);
		},

		ASTType::Pointer(array, inner) => Ok(Type::Pointer(array, Box::new(lower_ast_type(context, *inner)?))),
		ASTType::Reference(inner) => Ok(Type::Reference(Box::new(lower_ast_type(context, *inner)?))),
		ASTType::Array(size, inner) => Ok(Type::Array(size, Box::new(lower_ast_type(context, *inner)?)))
	};
}

pub fn lower_ast_type_struct(context: &mut HIRContext, t: ASTType, struct_container: &RawStructTypeContainer) -> BaseResult<TypeReference> {
	if let ASTType::Generic(id, _, _) = &t {
		let key = HashedString::new(id.clone());

		if struct_container.type_params.contains_key(&key) {
			return Ok(TypeReference::Unresolved(struct_container.type_params[&key]));
		}
	}

	return Ok(TypeReference::Resolved(lower_ast_type(context, t)?))
}

pub fn lower_sized_base_type(context: &HIRContext, t: &Type) -> BaseResult<RawType> {
	let data = t.get_generic_info();
	
	match t.get_generic(&context.type_storage) {
		RawType::SizedInteger(e) => {
			if data.1.is_empty() {
				return Err(BaseError::err(IR_EXPECTED_SIZE_SPECIFIED!().to_string()));
			}

			return Ok(RawType::Integer(data.1[0], e))
		},

		RawType::SizedFloating(e) => {
			if data.1.is_empty() {
				return Err(BaseError::err(IR_EXPECTED_SIZE_SPECIFIED!().to_string()));
			}

			return Ok(RawType::Floating(data.1[0], e))
		},

		RawType::SizedFixedPoint(e) => {
			if data.1.len() < 2 {
				return Err(BaseError::err(IR_EXPECTED_SIZE_SPECIFIED!().to_string()));
			}

			return Ok(RawType::FixedPoint(data.1[0], data.1[1], e));
		}, 

		_ => return Err(BaseError::err("Cannot lower sized base type!".to_string()))
	}
}