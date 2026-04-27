//! Declarations for enums in Quickfall

use std::mem::transmute;

use compiler_utils::{hash::HashedString, storage::Storage};

use crate::{
    TypeParameterContaining,
    constraints::TypeParameter,
    container::Type,
    structs::{StructuredField, StructuredFunction},
};

pub struct ParentEnumContainer {
    pub self_id: usize,
    pub name: HashedString,

    pub type_parameters: Storage<TypeParameter>,

    pub children: Storage<ChildEnumContainer>,
    pub functions: Storage<StructuredFunction>,
}

pub struct ChildEnumContainer {
    pub parent: &'static ParentEnumContainer,

    pub child_index: usize,
    pub fields: Storage<StructuredField>,
}

impl ParentEnumContainer {
    pub fn new(self_id: usize, name: String) -> Self {
        ParentEnumContainer {
            self_id,
            name: HashedString::new(name),
            type_parameters: Storage::new(),
            children: Storage::new(),
            functions: Storage::new(),
        }
    }
}

impl ChildEnumContainer {
    pub fn new(parent: &ParentEnumContainer, child_index: usize) -> Self {
        ChildEnumContainer {
            parent: unsafe {
                transmute::<&ParentEnumContainer, &'static ParentEnumContainer>(parent)
            },
            child_index,
            fields: Storage::new(),
        }
    }
}

impl TypeParameterContaining for ParentEnumContainer {
    fn append_type_parameter(
        &mut self,
        param: String,
        constraint: crate::constraints::TypeConstraintContainer,
    ) {
        self.type_parameters
            .insert(param.clone(), TypeParameter::new(param, constraint));
    }

    fn get_type_param_type(&self, param: String) -> crate::container::Type {
        let param = self.type_parameters.get(&param).unwrap();

        Type::GenericTypeParam {
            constraints: param.constraint.clone(),
            name: param.name.clone(),
        }
    }

    fn has_param_type(&self, param: String) -> bool {
        self.type_parameters.has_key(&param)
    }
}

impl TypeParameterContaining for ChildEnumContainer {
    fn append_type_parameter(
        &mut self,
        _param: String,
        _constraint: crate::constraints::TypeConstraintContainer,
    ) {
        panic!("Cannot add type parameter from child");
    }

    fn get_type_param_type(&self, param: String) -> Type {
        self.parent.get_type_param_type(param)
    }

    fn has_param_type(&self, param: String) -> bool {
        self.parent.has_param_type(param)
    }
}
