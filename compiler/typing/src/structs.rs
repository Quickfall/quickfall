//! Declarations for structs in Quickfall

use std::collections::HashMap;

use compiler_utils::{hash::HashedString, utils::indexed::IndexStorage};

use crate::{TypeParameterContaining, constraints::TypeParameter, container::Type};

/// A field inside of a struct-like type
#[derive(Clone)]
pub struct StructuredField {
    pub name: HashedString,
    pub t: Type,
}

#[derive(Clone)]
pub struct StructuredFunction {
    pub name: HashedString,
    pub return_type: Type,
    pub arguments: Vec<Type>,
}

#[derive(Clone)]
pub struct StructContainer {
    pub self_id: usize,
    pub name: String,
    pub type_parameters: IndexStorage<String, TypeParameter>,

    pub fields: IndexStorage<String, StructuredField>,
    pub functions: Vec<StructuredFunction>,
}

impl StructuredField {
    pub fn new(name: String, t: Type) -> Self {
        StructuredField {
            name: HashedString::new(name),
            t,
        }
    }
}

impl StructContainer {
    pub fn new(name: String, self_id: usize) -> Self {
        StructContainer {
            self_id,
            name,
            type_parameters: HashMap::new(),
            fields: vec![],
            functions: vec![],
        }
    }

	pub fn append_field(&mut self, field: )

}

impl TypeParameterContaining for StructContainer {
    fn get_type_param_type(&self, param: String) -> Type {
        let param = &self.type_parameters[&param];

        Type::GenericTypeParam {
            constraints: param.constraint.clone(),
            name: param.name.clone(),
        }
    }

    fn has_param_type(&self, param: String) -> bool {
        self.type_parameters.contains_key(&param)
    }
}
