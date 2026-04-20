//! Definitions for entries

use std::fmt::Display;

use diagnostics::{DiagnosticResult, DiagnosticSpanOrigin, builders::make_expected_simple_error};

use crate::GlobalStorageIdentifier;

#[derive(Clone, Debug)]
pub enum GlobalStorageEntryType<T, R> {
    Function {
        descriptor_ind: usize,
        impl_ind: usize,
    },
    ImplLessFunction(usize),
    StructFunction {
        descriptor_ind: usize,
        impl_ind: usize,
        struct_type: GlobalStorageIdentifier,
    },

    StaticVariable(T),

    TypeAlias(T),
    Type(R),
}

impl<T: Clone, R: Clone> GlobalStorageEntry<T, R> {
    pub fn as_function<K: DiagnosticSpanOrigin>(
        &self,
        origin: &K,
    ) -> DiagnosticResult<(usize, usize)> {
        match self.entry_type {
            GlobalStorageEntryType::Function {
                descriptor_ind,
                impl_ind,
            } => Ok((descriptor_ind, impl_ind)),

            _ => Err(
                make_expected_simple_error(origin, &"function".to_string(), &self.entry_type)
                    .into(),
            ),
        }
    }

    pub fn as_function_unsafe(&self) -> (usize, usize) {
        match self.entry_type {
            GlobalStorageEntryType::Function {
                descriptor_ind,
                impl_ind,
            } => (descriptor_ind, impl_ind),

            _ => panic!(),
        }
    }

    pub fn as_implless_function<K: DiagnosticSpanOrigin>(
        &self,
        origin: &K,
    ) -> DiagnosticResult<usize> {
        match self.entry_type {
            GlobalStorageEntryType::ImplLessFunction(ind) => Ok(ind),

            _ => Err(make_expected_simple_error(
                origin,
                &"implless function".to_string(),
                &self.entry_type,
            )
            .into()),
        }
    }

    pub fn as_implless_function_unsafe(&self) -> usize {
        match self.entry_type {
            GlobalStorageEntryType::ImplLessFunction(ind) => ind,

            _ => panic!(),
        }
    }

    pub fn as_struct_function<K: DiagnosticSpanOrigin>(
        &self,
        origin: &K,
    ) -> DiagnosticResult<(usize, usize, GlobalStorageIdentifier)> {
        match self.entry_type {
            GlobalStorageEntryType::StructFunction {
                descriptor_ind,
                impl_ind,
                struct_type,
            } => Ok((descriptor_ind, impl_ind, struct_type)),

            _ => Err(make_expected_simple_error(
                origin,
                &"struct function".to_string(),
                &self.entry_type,
            )
            .into()),
        }
    }

    pub fn as_struct_function_unsafe(&self) -> (usize, usize, GlobalStorageIdentifier) {
        match self.entry_type {
            GlobalStorageEntryType::StructFunction {
                descriptor_ind,
                impl_ind,
                struct_type,
            } => (descriptor_ind, impl_ind, struct_type),

            _ => panic!(),
        }
    }

    pub fn as_static_variable<K: DiagnosticSpanOrigin>(&self, origin: &K) -> DiagnosticResult<T> {
        match &self.entry_type {
            GlobalStorageEntryType::StaticVariable(t) => Ok(t.clone()),

            _ => Err(make_expected_simple_error(
                origin,
                &"static variable".to_string(),
                &self.entry_type,
            )
            .into()),
        }
    }

    pub fn as_static_variable_unsafe(&self) -> T {
        match &self.entry_type {
            GlobalStorageEntryType::StaticVariable(t) => t.clone(),

            _ => panic!(),
        }
    }

    pub fn as_type_alias<K: DiagnosticSpanOrigin>(&self, origin: &K) -> DiagnosticResult<T> {
        match &self.entry_type {
            GlobalStorageEntryType::TypeAlias(t) => Ok(t.clone()),

            _ => {
                Err(
                    make_expected_simple_error(origin, &"type alias".to_string(), &self.entry_type)
                        .into(),
                )
            }
        }
    }

    pub fn as_type_alias_unsafe(&self) -> T {
        match &self.entry_type {
            GlobalStorageEntryType::TypeAlias(t) => t.clone(),

            _ => panic!(),
        }
    }

    pub fn as_type<K: DiagnosticSpanOrigin>(&self, origin: &K) -> DiagnosticResult<R> {
        match &self.entry_type {
            GlobalStorageEntryType::Type(t) => Ok(t.clone()),

            _ => Err(
                make_expected_simple_error(origin, &"type".to_string(), &self.entry_type).into(),
            ),
        }
    }

    pub fn as_type_unsafe(&self) -> R {
        match &self.entry_type {
            GlobalStorageEntryType::Type(t) => t.clone(),

            _ => panic!(),
        }
    }
}

#[derive(Debug)]
pub struct GlobalStorageEntry<T, R> {
    pub entry_type: GlobalStorageEntryType<T, R>,
    pub parent_index: usize,
}

impl<T, R> Display for GlobalStorageEntryType<T, R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Function { .. } => "function",
            Self::ImplLessFunction(_) => "function",
            Self::StructFunction { .. } => "function",
            Self::StaticVariable(_) => "static variable",
            Self::Type(_) => "type",
            Self::TypeAlias(_) => "type (alias)",
        };

        write!(f, "{}", s)?;

        Ok(())
    }
}
