use compiler_global_scope::key::EntryKey;
use compiler_typing::tree::Type;
use compiler_utils::hash::SelfHash;
use diagnostics::{DiagnosticResult, DiagnosticSpanOrigin};

use crate::{ctx::branched::HIRBranchedContext, scope::HIRGlobalScopeStorage};

pub mod branched;

#[derive(Debug)]
pub struct HIRContext {
    pub global_scope: HIRGlobalScopeStorage,
}

#[derive(PartialEq)]
pub enum VariableKind {
    STATIC,
    LOCAL,
}

impl HIRContext {
    pub fn new() -> Self {
        return HIRContext {
            global_scope: HIRGlobalScopeStorage::new(),
        };
    }
}

pub fn get_variable<K: DiagnosticSpanOrigin>(
    context: &HIRContext,
    curr_ctx: &mut HIRBranchedContext,
    hash: u64,
    origin: &K,
) -> DiagnosticResult<(VariableKind, Type, usize)> {
    if curr_ctx.hash_to_ind.contains_key(&SelfHash { hash }) {
        let ind = curr_ctx.obtain(hash, origin)?;

        return Ok((
            VariableKind::LOCAL,
            curr_ctx.variables[ind].variable_type.clone(),
            ind,
        ));
    }

    let name = EntryKey { name_hash: hash };

    let ind = context.global_scope.scope.value_to_ind
        [&context.global_scope.get_base(name.clone(), origin)?];

    let t = context.global_scope.get_static_variable(name, origin)?;

    Ok((VariableKind::STATIC, t, ind))
}
