//! Declarations for structs in Quickfall

use compiler_utils::{hash::HashedString, storage::Storage};

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
    pub return_type: Option<Type>,
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

impl StructuredFunction {
    pub fn new(name: String, return_type: Option<Type>, arguments: Vec<Type>) -> Self {
        StructuredFunction {
            name: HashedString::new(name),
            return_type,
            arguments,
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

impl TypeParameterContaining for StructContainer {
    fn get_type_param_type(&self, param: String) -> Type {
        let param = self.type_parameters.get(&param).unwrap();

        Type::GenericTypeParam {
            constraints: param.constraint.clone(),
            name: param.name.clone(),
        }
    }

    fn has_param_type(&self, param: String) -> bool {
        self.type_parameters.has_key(&param)
    }

    fn append_type_parameter(
        &mut self,
        param: String,
        constraint: crate::constraints::TypeConstraintContainer,
    ) {
        self.type_parameters
            .insert(param.clone(), TypeParameter::new(param, constraint));
    }

    fn get_param_types(&self) -> Vec<String> {
        self.type_parameters.keys.clone()
    }
}
