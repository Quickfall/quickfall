use ast::types::ASTType;
use astoir_hir::ctx::HIRContext;
use compiler_global_scope::{entry::GlobalStorageEntryType, key::EntryKey};
use compiler_typing::{TypeParamType, raw::RawType, references::TypeReference, tree::Type};
use compiler_utils::hash::HashedString;
use diagnostics::{
    DiagnosticResult, DiagnosticSpanOrigin,
    builders::{
        make_cannot_find_type, make_diff_size_specifiers, make_diff_type_specifiers,
        make_req_type_kind,
    },
};

pub fn lower_ast_type<K: DiagnosticSpanOrigin>(
    context: &mut HIRContext,
    t: ASTType,
    origin: &K,
) -> DiagnosticResult<Type> {
    return match t {
        ASTType::Generic(type_id, type_params, size_params, specifier) => {
            let hash = HashedString::new(type_id).hash;

            let mut t = match context
                .global_scope
                .get_type(EntryKey { name_hash: hash }, origin)
            {
                Ok(v) => v,
                Err(_) => return Err(make_cannot_find_type(origin, &hash).into()),
            };

            if specifier.is_some() {
                let container = match t {
                    RawType::Enum(v) => v,
                    _ => return Err(make_req_type_kind(origin, &"enum".to_string()).into()),
                };

                t = container.get_entry(HashedString::new(specifier.unwrap()))?
            }

            if t.get_type_params_count(&context.global_scope, origin)? != type_params.len() {
                return Err(make_diff_type_specifiers(
                    origin,
                    &type_params.len(),
                    &t.get_type_params_count(&context.global_scope, origin)?,
                )
                .into());
            }

            let mut t_params = vec![];

            for type_param in type_params {
                t_params.push(Box::new(lower_ast_type(context, *type_param, origin)?));
            }

            let res = Type::Generic(t.clone(), t_params, size_params);

            if t.is_sized() {
                let lower = lower_sized_base_type(&res, origin)?;

                if context
                    .global_scope
                    .value_to_ind
                    .contains_key(&GlobalStorageEntryType::Type(lower.clone()))
                {
                    return Ok(Type::Generic(t, vec![], vec![]));
                } else {
                    let ind = match context.global_scope.append(
                        EntryKey { name_hash: hash },
                        GlobalStorageEntryType::Type(lower),
                        origin,
                    ) {
                        Ok(v) => v,
                        Err(_) => panic!("Generic lowering type cannot be found on type_to_hash"),
                    };

                    return Ok(Type::Generic(
                        context.global_scope.entries[ind].as_type_unsafe(),
                        vec![],
                        vec![],
                    ));
                }
            }

            return Ok(res);
        }

        ASTType::Pointer(array, inner) => Ok(Type::Pointer(
            array,
            Box::new(lower_ast_type(context, *inner, origin)?),
        )),
        ASTType::Reference(inner) => Ok(Type::Reference(Box::new(lower_ast_type(
            context, *inner, origin,
        )?))),
        ASTType::Array(size, inner) => Ok(Type::Array(
            size,
            Box::new(lower_ast_type(context, *inner, origin)?),
        )),
    };
}

pub fn lower_ast_type_struct<K: DiagnosticSpanOrigin, T: TypeParamType>(
    context: &mut HIRContext,
    t: ASTType,
    container: &T,
    origin: &K,
) -> DiagnosticResult<TypeReference> {
    if let ASTType::Generic(id, _, _, _) = &t {
        let key = HashedString::new(id.clone());

        if container.has_type_param(&key) {
            return Ok(TypeReference::Unresolved(
                container.get_type_param_ind(&key),
            ));
        }
    }

    return Ok(TypeReference::Resolved(lower_ast_type(context, t, origin)?));
}

pub fn lower_sized_base_type<K: DiagnosticSpanOrigin>(
    t: &Type,
    origin: &K,
) -> DiagnosticResult<RawType> {
    let data = t.get_generic_info();

    match t.get_generic() {
        RawType::SizedInteger(e) => {
            if data.1.len() != 1 {
                return Err(make_diff_size_specifiers(origin, &1, &data.1.len()).into());
            }

            return Ok(RawType::Integer(data.1[0], e));
        }

        RawType::SizedFloating(e) => {
            if data.1.len() != 1 {
                return Err(make_diff_size_specifiers(origin, &1, &data.1.len()).into());
            }

            return Ok(RawType::Floating(data.1[0], e));
        }

        RawType::SizedFixedPoint(e) => {
            if data.1.len() != 2 {
                return Err(make_diff_size_specifiers(origin, &1, &data.1.len()).into());
            }

            return Ok(RawType::FixedPoint(data.1[0], data.1[1], e));
        }

        _ => panic!("This is not a sized type"),
    }
}
