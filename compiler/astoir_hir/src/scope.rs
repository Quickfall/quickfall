//! HIR version of the global scope in order to store descriptors and implementations

use compiler_global_scope::key::EntryKey;
use compiler_typing::{TypedGlobalScope, TypedGlobalScopeEntry, raw::RawType, tree::Type};
use diagnostics::{DiagnosticResult, DiagnosticSpanOrigin};

use crate::{ctx::HIRFunction, nodes::HIRNode};

/// The HIR version of `GlobalScopeStorage`. Contains the descriptors and implementations.
/// Every function to append, gather will automatically handle descriptors and implementations if needed
pub struct HIRGlobalScopeStorage {
    pub scope: TypedGlobalScope,
    pub descriptors: Vec<HIRFunction>,
    pub implementations: Vec<Box<HIRNode>>,
}

impl HIRGlobalScopeStorage {
    pub fn new() -> Self {
        HIRGlobalScopeStorage {
            scope: TypedGlobalScope::new(),
            descriptors: vec![],
            implementations: vec![],
        }
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
        origin: &K,
    ) -> DiagnosticResult<usize> {
        self.descriptors.push(descriptor);
        self.implementations.push(implementation);

        self.scope.append(
            name,
            TypedGlobalScopeEntry::Function {
                descriptor_ind: self.scope.descriptor_counter,
                impl_ind: self.scope.impl_counter,
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
        struct_type: RawType,
        origin: &K,
    ) -> DiagnosticResult<usize> {
        self.descriptors.push(descriptor);
        self.implementations.push(implementation);

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
}
