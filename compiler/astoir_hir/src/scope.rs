//! HIR version of the global scope in order to store descriptors and implementations

use compiler_global_scope::key::EntryKey;
use compiler_typing::{TypedGlobalScope, TypedGlobalScopeEntry, raw::RawType, tree::Type};
use diagnostics::{
    DiagnosticResult, DiagnosticSpanOrigin, MaybeDiagnostic,
    builders::{make_already_in_scope, make_cannot_find},
};

use crate::func::HIRNewFunction;

/// The HIR version of `GlobalScopeStorage`. Contains the descriptors and implementations.
/// Every function to append, gather will automatically handle descriptors and implementations if needed
#[derive(Debug)]
pub struct HIRGlobalScopeStorage {
    pub scope: TypedGlobalScope,
    pub functions: Vec<HIRNewFunction>,
}

impl HIRGlobalScopeStorage {
    pub fn new() -> Self {
        HIRGlobalScopeStorage {
            scope: TypedGlobalScope::new(),
            functions: vec![],
        }
    }

    pub fn enforce_not_here<K: DiagnosticSpanOrigin>(
        &mut self,
        name: EntryKey,
        origin: &K,
    ) -> MaybeDiagnostic {
        if self.scope.entry_to_ind.contains_key(&name) {
            return Err(make_already_in_scope(origin, &name.name_hash).into());
        }

        Ok(())
    }

    /// This doesn't automatically handle descriptors and implementations
    pub fn append<K: DiagnosticSpanOrigin>(
        &mut self,
        name: EntryKey,
        entry: TypedGlobalScopeEntry,
        origin: &K,
    ) -> DiagnosticResult<usize> {
        self.scope.append(name, entry, origin)
    }

    pub fn append_func<K: DiagnosticSpanOrigin>(
        &mut self,
        name: EntryKey,
        func: HIRNewFunction,
        origin: &K,
    ) -> DiagnosticResult<usize> {
        self.functions.push(func);

        self.scope.append(
            name,
            TypedGlobalScopeEntry::Function(self.scope.function_counter),
            origin,
        )
    }

    pub fn append_type<K: DiagnosticSpanOrigin>(
        &mut self,
        name: EntryKey,
        t: RawType,
        origin: &K,
    ) -> DiagnosticResult<usize> {
        self.scope
            .append(name, TypedGlobalScopeEntry::Type(t), origin)
    }

    pub fn append_type_binding<K: DiagnosticSpanOrigin>(
        &mut self,
        name: EntryKey,
        t: Type,
        origin: &K,
    ) -> DiagnosticResult<usize> {
        self.scope
            .append(name, TypedGlobalScopeEntry::TypeAlias(t), origin)
    }

    pub fn get_base<K: DiagnosticSpanOrigin>(
        &self,
        name: EntryKey,
        origin: &K,
    ) -> DiagnosticResult<TypedGlobalScopeEntry> {
        self.scope.get_base(name, origin)
    }

    pub fn get_ind<K: DiagnosticSpanOrigin>(
        &self,
        name: EntryKey,
        origin: &K,
    ) -> DiagnosticResult<usize> {
        if self.scope.entry_to_ind.contains_key(&name) {
            return Ok(self.scope.entry_to_ind[&name]);
        }

        return Err(make_cannot_find(origin, &name.name_hash).into());
    }

    pub fn get_type<K: DiagnosticSpanOrigin>(
        &self,
        name: EntryKey,
        origin: &K,
    ) -> DiagnosticResult<RawType> {
        self.scope.get_type(name, origin)
    }

    pub fn get_static_variable<K: DiagnosticSpanOrigin>(
        &self,
        name: EntryKey,
        origin: &K,
    ) -> DiagnosticResult<Type> {
        self.scope.get_static_variable(name, origin)
    }

    pub fn get_function<K: DiagnosticSpanOrigin>(
        &self,
        name: EntryKey,
        origin: &K,
    ) -> DiagnosticResult<&HIRNewFunction> {
        let ind = self.scope.get_function(name, origin)?;

        return Ok(&self.functions[ind]);
    }
}
