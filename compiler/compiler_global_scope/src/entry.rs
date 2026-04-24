//! Definitions for entries

use std::{fmt::Display, hash::Hash};

use diagnostics::{DiagnosticResult, DiagnosticSpanOrigin, builders::make_expected_simple_error};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum GlobalStorageEntryType<T: Hash, R: Hash> {
    Function(usize),
    StaticVariable(T),

    TypeAlias(T),
    Type(R),
}

impl<T: Clone + Hash, R: Clone + Hash> GlobalStorageEntry<T, R> {
    pub fn as_function<K: DiagnosticSpanOrigin>(&self, origin: &K) -> DiagnosticResult<usize> {
        match self.entry_type {
            GlobalStorageEntryType::Function(ind) => Ok(ind),

            _ => Err(
                make_expected_simple_error(origin, &"function".to_string(), &self.entry_type)
                    .into(),
            ),
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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct GlobalStorageEntry<T: Hash, R: Hash> {
    pub entry_type: GlobalStorageEntryType<T, R>,
    pub parent_index: usize,
}

impl<T: Hash, R: Hash> Display for GlobalStorageEntryType<T, R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Function(_) => "function",
            Self::StaticVariable(_) => "static variable",
            Self::Type(_) => "type",
            Self::TypeAlias(_) => "type (alias)",
        };

        write!(f, "{}", s)?;

        Ok(())
    }
}
