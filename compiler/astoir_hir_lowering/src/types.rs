use ast::types::ASTType;
use astoir_hir::{context::HIRContext, scope::key::EntryKey};
use compiler_utils::hash::HashedString;
use diagnostics::{
    DiagnosticResult, DiagnosticSpanOrigin,
    builders::{
        make_cannot_find, make_diff_size_specifiers, make_diff_type_specifiers, make_req_type_kind,
    },
};
use typing::{
    TypeParameterContaining,
    container::Type,
    raw::{InformationRawType, RawType},
};

pub fn lower_ast_type<K: DiagnosticSpanOrigin>(
    context: &mut HIRContext,
    t: ASTType,
    curr_t: Option<RawType>,
    origin: &K,
) -> DiagnosticResult<Type> {
    return match t {
        ASTType::Generic(type_id, type_params, size_params, specifier) => {
            let hash = HashedString::new(type_id);
            let key = EntryKey::new(hash.clone());

            let mut hir = match context.scope.get_type(&key, origin) {
                Ok(v) => v.t.clone(),
                Err(_) => {
                    if curr_t.is_some() {
                        let curr_t = curr_t.clone().unwrap();

                        if curr_t.has_param_type(hash.val.clone()) {
                            curr_t.get_type_param_type(hash.val.clone()).get_raw().t
                        } else {
                            return Err(make_cannot_find(origin, &key).into());
                        }
                    } else {
                        return Err(make_cannot_find(origin, &key).into());
                    }
                }
            };

            if let Some(spec) = specifier {
                let container = match &hir {
                    RawType::Enum(container) => container,
                    _ => return Err(make_req_type_kind(origin, &"enum".to_string()).into()),
                };

                let child = match container.children.get(&spec) {
                    Some(v) => v,
                    None => {
                        return Err(make_cannot_find(origin, &format!("{}:{}", key, spec)).into());
                    }
                };

                hir = RawType::EnumChild(child.clone());
            };

            if type_params.len() != hir.get_param_types().len() {
                return Err(make_diff_type_specifiers(
                    origin,
                    &type_params.len(),
                    &hir.get_param_types().len(),
                )
                .into());
            }

            if size_params.len() != hir.get_raw_require_type_parameters() {
                return Err(make_diff_size_specifiers(
                    origin,
                    &size_params.len(),
                    &hir.get_raw_require_type_parameters(),
                )
                .into());
            }

            hir = hir.lower(size_params.clone());

            let mut t_params = vec![];

            for param in type_params {
                t_params.push(Box::new(lower_ast_type(
                    context,
                    *param,
                    curr_t.clone(),
                    origin,
                )?));
            }

            let mut info_raw = InformationRawType::new(hir);

            info_raw.type_parameters = t_params;
            info_raw.sizes = size_params;

            Ok(Type::Raw { raw: info_raw })
        }

        ASTType::Array(a, b) => Ok(Type::Array {
            size: a,
            inner: Box::new(lower_ast_type(context, *b, curr_t, origin)?),
        }),

        ASTType::Pointer(a, b) => Ok(Type::Pointer {
            is_array: a,
            inner: Box::new(lower_ast_type(context, *b, curr_t, origin)?),
        }),

        // TODO: actually add references instead of roughly estimating pointer
        ASTType::Reference(inner) => Ok(Type::Pointer {
            is_array: false,
            inner: Box::new(lower_ast_type(context, *inner, curr_t, origin)?),
        }),
    };
}
