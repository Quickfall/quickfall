//! HIR version of the global scope in order to store descriptors and implementations

use compiler_typing::TypedGlobalScope;

use crate::{ctx::HIRFunction, nodes::HIRNode};

/// The HIR version of `GlobalScopeStorage`. Contains the descriptors and implementations.
/// Every function to append, gather will automatically handle descriptors and implementations if needed
pub struct HIRGlobalScopeStorage {
    pub scope: TypedGlobalScope,
    pub descriptors: Vec<HIRFunction>,
    pub implementations: Vec<Box<HIRNode>>,
}
