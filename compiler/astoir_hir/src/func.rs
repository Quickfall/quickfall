//! HIR function related definitions

use compiler_typing::{references::TypeReference, tree::Type};
use compiler_utils::utils::maybe::Maybe;

use crate::{ctx::branched::HIRBranchedContext, nodes::HIRNode};

pub type HIRFunctionArgument = (u64, TypeReference);

/// The new way of storing functions in the HIR layer.
///
/// # Advantages
/// This approach allows for us to easily add components like HIRBranchedContexts or impls to existing functions.
/// Allowing for clean recursion handling
#[doc = "Experimental: will progressively be propagated"]
#[derive(Debug)]
pub struct HIRNewFunction {
    pub return_type: Option<Type>,
    pub arguments: Vec<HIRFunctionArgument>,

    pub branched_ctx: Maybe<HIRBranchedContext>,
    pub implementation: Maybe<Box<HIRNode>>,

    /// The number of times this function is called, allows for dead code elimination
    pub usage_count: usize,
}

impl HIRNewFunction {
    /// Creates a new shadow function instance.
    pub fn new_shadow(return_type: Option<TypeReference>, args: Vec<HIRFunctionArgument>) -> Self {
        HIRNewFunction {
            return_type,
            arguments: args,
            branched_ctx: Maybe::None,
            implementation: Maybe::None,
            usage_count: 0,
        }
    }

    /// Creates a function that includes basic information (eg return type, arguments) and an HIR branched ctx
    pub fn new_half(
        return_type: Option<TypeReference>,
        args: Vec<HIRFunctionArgument>,
        branched_ctx: HIRBranchedContext,
    ) -> Self {
        HIRNewFunction {
            return_type,
            arguments: args,
            branched_ctx: Maybe::Some(branched_ctx),
            implementation: Maybe::None,
            usage_count: 0,
        }
    }

    /// Creates a function with every information:
    /// - Basic information
    /// - Branched CTX
    /// - Implementation Node
    ///
    /// This is usually for late-stage implementation registration.
    pub fn new_full(
        return_type: Option<TypeReference>,
        args: Vec<HIRFunctionArgument>,
        branched_ctx: HIRBranchedContext,
        implementation: Box<HIRNode>,
    ) -> Self {
        HIRNewFunction {
            return_type,
            arguments: args,
            branched_ctx: Maybe::Some(branched_ctx),
            implementation: Maybe::Some(implementation),
            usage_count: 0,
        }
    }
}
