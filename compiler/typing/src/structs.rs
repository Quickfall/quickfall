//! Declarations for structs in Quickfall

use std::collections::HashMap;

use compiler_utils::{hash::HashedString, storage::Storage, utils::indexed::IndexStorage};

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
    pub type_parameters: Storage<TypeParameter>,

    pub fields: Storage<StructuredField>,
    pub functions: Storage<StructuredFunction>,
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
            type_parameters: Storage::new(),
            fields: Storage::new(),
            functions: Storage::new(),
        }
    }

    pub fn append_field(&mut self, field: String, t: Type) -> bool {
        self.fields
            .insert(field.clone(), StructuredField::new(field, t))
    }
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
