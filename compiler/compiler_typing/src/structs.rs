use std::collections::HashMap;

use compiler_utils::utils::indexed::IndexStorage;
use diagnostics::{
    DiagnosticResult,
    builders::{make_cannot_find_type_field, make_cannot_find_type_function},
};

use crate::{
    SizedType, StructuredType, TypeParamType, TypeParameterContainer, TypeReference, TypedFunction,
    TypedGlobalScope,
    enums::{RawEnumEntryContainer, RawEnumTypeContainer},
    tree::Type,
};

/// Container for structure types
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RawStructTypeContainer {
    pub fields: IndexStorage<TypeReference>,
    pub type_params: TypeParameterContainer,
    pub functions: IndexStorage<TypedFunction>,
    pub function_ids: Vec<usize>,
    pub self_ref: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoweredStructTypeContainer {
    pub fields: IndexStorage<Type>,
    pub is_lowered_enum_child: bool,
    pub is_lowered_enum_parent: bool,
    pub lowered_enum_parent: Option<RawEnumTypeContainer>,
    pub lowered_enum_child: Option<RawEnumEntryContainer>,
    pub hir_mir_indexes: HashMap<usize, usize>,
    pub functions: IndexStorage<usize>,
}

impl LoweredStructTypeContainer {
    /// Resolves the given `HIR` field index into the corresponding `MIR` field index if said one has changed.
    pub fn resolve_hir_index(&self, ind: usize) -> usize {
        if self.hir_mir_indexes.contains_key(&ind) {
            return self.hir_mir_indexes[&ind];
        }

        return ind;
    }

    pub fn append_hir_index_conv(&mut self, hir: usize, mir: usize) {
        self.hir_mir_indexes.insert(hir, mir);
    }
}

impl SizedType for LoweredStructTypeContainer {
    fn get_size(&self, t: &Type, compacted_size: bool, storage: &TypedGlobalScope) -> usize {
        let mut size = 0;

        for field in &self.fields.vals {
            size += field.get_size(t, compacted_size, storage)
        }

        return size;
    }
}

impl SizedType for RawStructTypeContainer {
    fn get_size(&self, t: &Type, compacted_size: bool, storage: &TypedGlobalScope) -> usize {
        let mut size = 0;

        for field in &self.fields.vals {
            let base = field.clone().resolve(&t);

            size += base.get_size(t, compacted_size, storage);
        }

        return size;
    }
}

impl StructuredType for RawStructTypeContainer {
    fn get_function(
        &self,
        hash: u64,
        _storage: &TypedGlobalScope,
    ) -> DiagnosticResult<TypedFunction> {
        let k = match self.functions.get_index(hash) {
            Some(v) => v,
            None => {
                return Err(make_cannot_find_type_function(&hash, &"unnamed".to_string()).into());
            }
        };

        return Ok(self.functions.vals[k].clone());
    }

    fn get_fields(&self, _storage: &TypedGlobalScope) -> Vec<u64> {
        return self.fields.entry_keys.clone();
    }

    fn get_functions(&self, _storage: &TypedGlobalScope) -> Vec<u64> {
        return self.functions.entry_keys.clone();
    }

    fn get_function_hash(&self, hash: u64, _storage: &TypedGlobalScope) -> DiagnosticResult<usize> {
        let k = match self.functions.get_index(hash) {
            Some(v) => v,
            None => {
                return Err(make_cannot_find_type_function(&hash, &"unnamed".to_string()).into());
            }
        };

        return Ok(k);
    }

    fn get_field(&self, hash: u64, _storage: &TypedGlobalScope) -> DiagnosticResult<TypeReference> {
        let k = match self.fields.get_index(hash) {
            Some(v) => v,
            None => return Err(make_cannot_find_type_field(&hash, &"unamed".to_string()).into()),
        };

        return Ok(self.fields.vals[k].clone());
    }

    fn get_field_hash(&self, hash: u64, _storage: &TypedGlobalScope) -> DiagnosticResult<usize> {
        let k = match self.fields.get_index(hash) {
            Some(v) => v,
            None => return Err(make_cannot_find_type_field(&hash, &"unamed".to_string()).into()),
        };

        return Ok(k);
    }
}

impl TypeParamType for RawStructTypeContainer {
    fn has_type_param(&self, param: &compiler_utils::hash::HashedString) -> bool {
        self.type_params.contains_key(param)
    }

    fn get_type_param_ind(&self, param: &compiler_utils::hash::HashedString) -> usize {
        self.type_params[param]
    }
}
