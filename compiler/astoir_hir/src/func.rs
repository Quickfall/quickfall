//! Definitions for HIR functions

use compiler_utils::{hash::HashedString, storage::Storage};
use typing::{constraints::TypeParameter, container::Type, raw::RawType};

use crate::{context::local::BranchedContext, nodes::HIRNode};

pub struct HIRFunction {
    pub name: HashedString,
    pub self_id: usize,

    pub type_parameters: Storage<TypeParameter>,

    pub return_type: Option<Type>,
    pub arguments: Vec<(HashedString, Type)>,

    pub struct_depending: Option<RawType>,

    pub ctx: Option<BranchedContext>,
    pub implementation: Option<Box<HIRNode>>,
}

impl HIRFunction {
    pub fn new_shadow(
        name: String,
        return_type: Option<Type>,
        arguments: Vec<(HashedString, Type)>,
    ) -> Self {
        HIRFunction {
            name: HashedString::new(name),
            type_parameters: Storage::new(),
            return_type,
            arguments,
            struct_depending: None,
            ctx: None,
            implementation: None,
            self_id: 0,
        }
    }

    pub fn new_pre_full(
        name: String,
        return_type: Option<Type>,
        arguments: Vec<(HashedString, Type)>,
        ctx: BranchedContext,
    ) -> Self {
        HIRFunction {
            name: HashedString::new(name),
            type_parameters: Storage::new(),
            return_type,
            arguments,
            struct_depending: None,
            ctx: Some(ctx),
            implementation: None,
            self_id: 0,
        }
    }

    pub fn new_full(
        name: String,
        return_type: Option<Type>,
        arguments: Vec<(HashedString, Type)>,
        ctx: BranchedContext,
        implementation: Box<HIRNode>,
    ) -> Self {
        HIRFunction {
            name: HashedString::new(name),
            type_parameters: Storage::new(),
            return_type,
            arguments,
            struct_depending: None,
            ctx: Some(ctx),
            implementation: Some(implementation),
            self_id: 0,
        }
    }

    pub fn change_self_id(&mut self, id: usize) {
        self.self_id = id;
    }
}
