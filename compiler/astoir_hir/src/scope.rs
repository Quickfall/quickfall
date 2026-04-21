//! HIR version of the global scope in order to store descriptors and implementations

use compiler_global_scope::key::EntryKey;
use compiler_typing::{TypedGlobalScope, TypedGlobalScopeEntry, raw::RawType, tree::Type};
use diagnostics::{
    DiagnosticResult, DiagnosticSpanOrigin, MaybeDiagnostic,
    builders::{make_already_in_scope, make_cannot_find},
};

use crate::{
    ctx::{HIRBranchedContext, HIRFunction, HIRFunctionImpl},
    nodes::HIRNode,
};

/// The HIR version of `GlobalScopeStorage`. Contains the descriptors and implementations.
/// Every function to append, gather will automatically handle descriptors and implementations if needed
#[derive(Debug)]
pub struct HIRGlobalScopeStorage {
    pub scope: TypedGlobalScope,
    pub descriptors: Vec<HIRFunction>,
    pub implementations: Vec<HIRFunctionImpl>,
    pub contexts: Vec<HIRBranchedContext>,
}

impl HIRGlobalScopeStorage {
    pub fn new() -> Self {
        HIRGlobalScopeStorage {
            scope: TypedGlobalScope::new(),
            descriptors: vec![],
            implementations: vec![],
            contexts: vec![],
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
        descriptor: HIRFunction,
        implementation: Box<HIRNode>,
        brctx: HIRBranchedContext,
        origin: &K,
    ) -> DiagnosticResult<usize> {
        self.descriptors.push(descriptor);
        self.implementations.push(implementation);
        self.contexts.push(brctx);

        self.scope.append(
            name,
            TypedGlobalScopeEntry::Function {
                descriptor_ind: self.scope.descriptor_counter,
                impl_ind: self.scope.impl_counter,
            },
            origin,
        )
    }

    pub fn append_half_function<K: DiagnosticSpanOrigin>(
        &mut self,
        name: EntryKey,
        descriptor: HIRFunction,
        brctx: HIRBranchedContext,
        origin: &K,
    ) -> DiagnosticResult<usize> {
        self.descriptors.push(descriptor);
        self.contexts.push(brctx);

        self.scope.append(
            name,
            TypedGlobalScopeEntry::HalfImplFunction {
                descriptor_ind: self.scope.descriptor_counter,
                branch_ctx: self.scope.ctx_counter,
            },
            origin,
        )
    }

    pub fn append_implless_function<K: DiagnosticSpanOrigin>(
        &mut self,
        name: EntryKey,
        descriptor: HIRFunction,
        origin: &K,
    ) -> DiagnosticResult<usize> {
        self.descriptors.push(descriptor);

        self.scope.append(
            name,
            TypedGlobalScopeEntry::ImplLessFunction(self.scope.descriptor_counter),
            origin,
        )
    }

    pub fn append_struct_function<K: DiagnosticSpanOrigin>(
        &mut self,
        name: EntryKey,
        descriptor: HIRFunction,
        implementation: Box<HIRNode>,
        brctx: HIRBranchedContext,
        struct_type: RawType,
        origin: &K,
    ) -> DiagnosticResult<usize> {
        self.descriptors.push(descriptor);
        self.implementations.push(implementation);
        self.contexts.push(brctx);

        let ind = self.scope.value_to_ind[&TypedGlobalScopeEntry::Type(struct_type)];

        self.scope.append(
            name,
            TypedGlobalScopeEntry::StructFunction {
                descriptor_ind: self.scope.descriptor_counter,
                impl_ind: self.scope.impl_counter,
                struct_type: ind,
            },
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

    pub fn get_function_base<K: DiagnosticSpanOrigin>(
        &self,
        name: EntryKey,
        origin: &K,
    ) -> DiagnosticResult<HIRFunction> {
        let ind = self.scope.get_function_base(name, origin)?;

        return Ok(self.descriptors[ind].clone());
    }

    pub fn get_function_ctx<K: DiagnosticSpanOrigin>(
        &self,
        name: EntryKey,
        origin: &K,
    ) -> DiagnosticResult<HIRBranchedContext> {
        let ind = self.scope.get_function_ctx(name, origin)?;

        return Ok(self.contexts[ind].clone());
    }

    pub fn get_function_impl<K: DiagnosticSpanOrigin>(
        &self,
        name: EntryKey,
        origin: &K,
    ) -> DiagnosticResult<(HIRFunctionImpl, HIRBranchedContext)> {
        let ind = self.scope.get_function_impl(name, origin)?;

        return Ok((
            self.implementations[ind].clone(),
            self.contexts[ind].clone(),
        ));
    }

    pub fn get_implless_function<K: DiagnosticSpanOrigin>(
        &self,
        name: EntryKey,
        origin: &K,
    ) -> DiagnosticResult<HIRFunction> {
        let ind = self.scope.get_implless_function(name, origin)?;

        return Ok(self.descriptors[ind].clone());
    }

    pub fn get_exact_function<K: DiagnosticSpanOrigin>(
        &self,
        name: EntryKey,
        origin: &K,
    ) -> DiagnosticResult<(HIRFunction, HIRFunctionImpl)> {
        let inds = self.scope.get_exact_function(name, origin)?;

        return Ok((
            self.descriptors[inds.0].clone(),
            self.implementations[inds.1].clone(),
        ));
    }

    pub fn get_exact_struct_function<K: DiagnosticSpanOrigin>(
        &self,
        name: EntryKey,
        origin: &K,
    ) -> DiagnosticResult<(HIRFunction, HIRFunctionImpl, HIRBranchedContext, RawType)> {
        let res = self.scope.get_exact_struct_function(name, origin)?;

        return Ok((
            self.descriptors[res.0].clone(),
            self.implementations[res.1].clone(),
            self.contexts[res.1].clone(),
            res.2,
        ));
    }
}
