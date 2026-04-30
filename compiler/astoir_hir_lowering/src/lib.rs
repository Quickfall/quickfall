use ast::{ctx::ParserCtx, types::ASTType};
use astoir_hir::{context::HIRContext, scope::key::EntryKey};
use compiler_utils::hash::HashedString;
use diagnostics::{
    DiagnosticResult, DiagnosticSpanOrigin,
    builders::{make_cannot_find, make_req_type_kind},
};
use typing::raw::RawType;

pub mod funcs;

pub fn lower_ast_hir(ctx: ParserCtx) -> DiagnosticResult<HIRContext> {
    let hir = HIRContext::new();

    Ok(hir)
}

pub fn lower_ast_type<K: DiagnosticSpanOrigin>(
    context: &mut HIRContext,
    t: ASTType,
    origin: &K,
) -> DiagnosticResult<Type> {
    return match t {
        ASTType::Generic(type_id, type_params, size_params, specifier) => {
            let hash = HashedString::new(type_id);
            let key = EntryKey::new(hash.clone());

            let mut hir = context.scope.get_type(&key, origin)?.t;

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
        }
    };
}
