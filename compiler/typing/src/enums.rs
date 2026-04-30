//! Declarations for enums in Quickfall

use std::mem::transmute;

use compiler_utils::{hash::HashedString, storage::Storage};

use crate::{
    TypeParameterContaining,
    constraints::TypeParameter,
    container::Type,
    structs::{StructuredField, StructuredFunction},
};

#[derive(Clone)]
pub struct ParentEnumContainer {
    pub self_id: usize,
    pub name: HashedString,

    pub type_parameters: Storage<TypeParameter>,

    pub children: Storage<ChildEnumContainer>,
    pub functions: Storage<StructuredFunction>,
}

#[derive(Clone)]
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

    pub fn append_function(
        &mut self,
        name: String,
        return_type: Option<Type>,
        arguments: Vec<Type>,
    ) -> bool {
        self.functions.insert(
            name.clone(),
            StructuredFunction::new(name, return_type, arguments),
        )
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

    pub fn append_field(&mut self, field: String, t: Type) -> bool {
        self.fields
            .insert(field.clone(), StructuredField::new(field, t))
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

    fn get_param_types(&self) -> Vec<String> {
        self.type_parameters.keys.clone()
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

    fn get_param_types(&self) -> Vec<String> {
        self.parent.get_param_types().clone()
    }
}
