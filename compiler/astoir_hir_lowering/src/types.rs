use ast::types::ASTType;
use astoir_hir::ctx::HIRContext;
use compiler_errors::{IR_EXPECTED_SIZE_SPECIFIED, errs::{BaseResult, base::BaseError}};
use compiler_typing::{raw::RawType, references::TypeReference, structs::RawStructTypeContainer, tree::Type};
use compiler_utils::hash::HashedString;
use diagnostics::{DiagnosticResult, DiagnosticSpanOrigin, builders::{make_cannot_find_type, make_diff_size_specifiers}};

pub fn lower_ast_type<K: DiagnosticSpanOrigin>(context: &mut HIRContext, t: ASTType, origin: &K) -> DiagnosticResult<Type> {
	return match t {
		ASTType::Generic(type_id, type_params, size_params) => {
			let hash = HashedString::new(type_id).hash;

			let t = match context.type_storage.get_type(hash) {
				Ok(v) => v,
				Err(_) => return Err(make_cannot_find_type(origin, &hash).into())
			};

			if t.get_type_params_count(&context.type_storage) != type_params.len() {
				return Err(make_diff_size_specifiers(origin, &type_params.len(), &t.get_type_params_count(&context.type_storage)).into())
			}

			let mut t_params = vec![];

			for type_param in type_params {
				t_params.push(Box::new(lower_ast_type(context, *type_param, origin)?));
			}

			let res = Type::Generic(context.type_storage.types.hash_to_ind[&hash], t_params, size_params);
			
			if t.is_sized() {
				let lower = lower_sized_base_type(context, &res, origin)?;

				if context.type_storage.type_to_ind.contains_key(&lower) {
					return Ok(Type::Generic(context.type_storage.type_to_ind[&lower], vec![], vec![]));
				} else {
					let ind = match context.type_storage.append_with_hash(hash, lower) {
						Ok(v) => v,
						Err(_) => panic!("Generic lowering type cannot be found on type_to_hash")
					};

					return Ok(Type::Generic(ind, vec![], vec![]))
				}
			}

			return Ok(res);
		},

		ASTType::Pointer(array, inner) => Ok(Type::Pointer(array, Box::new(lower_ast_type(context, *inner, origin)?))),
		ASTType::Reference(inner) => Ok(Type::Reference(Box::new(lower_ast_type(context, *inner, origin)?))),
		ASTType::Array(size, inner) => Ok(Type::Array(size, Box::new(lower_ast_type(context, *inner, origin)?)))
	};
}

pub fn lower_ast_type_struct<K: DiagnosticSpanOrigin>(context: &mut HIRContext, t: ASTType, struct_container: &RawStructTypeContainer, origin: &K) -> DiagnosticResult<TypeReference> {
	if let ASTType::Generic(id, _, _) = &t {
		let key = HashedString::new(id.clone());

		if struct_container.type_params.contains_key(&key) {
			return Ok(TypeReference::Unresolved(struct_container.type_params[&key]));
		}
	}

	return Ok(TypeReference::Resolved(lower_ast_type(context, t, origin)?))
}

pub fn lower_sized_base_type<K: DiagnosticSpanOrigin>(context: &HIRContext, t: &Type, origin: &K) -> DiagnosticResult<RawType> {
	let data = t.get_generic_info();
	
	match t.get_generic(&context.type_storage) {
		RawType::SizedInteger(e) => {
			if data.1.len() != 1 {
				return Err(make_diff_size_specifiers(origin, &1, &data.1.len()).into())
			}

			return Ok(RawType::Integer(data.1[0], e))
		},

		RawType::SizedFloating(e) => {
			if data.1.len() != 1  {
				return Err(make_diff_size_specifiers(origin, &1, &data.1.len()).into())
			}

			return Ok(RawType::Floating(data.1[0], e))
		},

		RawType::SizedFixedPoint(e) => {
			if data.1.len() != 2 {
				return Err(make_diff_size_specifiers(origin, &1, &data.1.len()).into())
			}

			return Ok(RawType::FixedPoint(data.1[0], data.1[1], e));
		}, 

		_ => panic!("This is not a sized type")
	}
}