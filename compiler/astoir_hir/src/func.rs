//! HIR function related definitions

use compiler_typing::references::TypeReference;

use crate::{ctx::branched::HIRBranchedContext, nodes::HIRNode};

pub type HIRFunctionArgument = (u64, TypeReference);

/// The new way of storing functions
#[doc = "Experimental: will progressively be propagated"]
pub struct HIRNewFunction {
    pub return_type: Option<TypeReference>,
    pub arguments: Vec<HIRFunctionArgument>,

    pub branched_ctx: Option<HIRBranchedContext>,
    pub implementation: Option<Box<HIRNode>>,
}
