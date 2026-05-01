//! Definitions for the scope entries

use std::{fmt::Display, mem::transmute};

use diagnostics::{
    DiagnosticResult, DiagnosticSpanOrigin, MaybeDiagnostic, builders::make_expected_simple_error,
};

use crate::{func::HIRFunction, types::ScopeStoredType};

/// An entry inside of the global scope.
///
/// # Warn
/// You must never replace entries to modify it, always modify an entry using the given modify functions as 'static references are attached to them
pub enum ScopeEntry {
    Type(ScopeStoredType),

    Function(HIRFunction),
}

impl ScopeEntry {
    pub fn new_type(t: ScopeStoredType) -> Self {
        Self::Type(t)
    }

    pub fn new_function(func: HIRFunction) -> Self {
        Self::Function(func)
    }

    pub fn as_type<K: DiagnosticSpanOrigin>(
        &self,
        origin: &K,
    ) -> DiagnosticResult<&'static ScopeStoredType> {
        if let Self::Type(v) = self {
            return Ok(unsafe { transmute::<&ScopeStoredType, &'static ScopeStoredType>(v) });
        }

        return Err(make_expected_simple_error(origin, &"type".to_string(), self).into());
    }

    pub fn as_function<K: DiagnosticSpanOrigin>(
        &self,
        origin: &K,
    ) -> DiagnosticResult<&'static HIRFunction> {
        if let Self::Function(func) = self {
            return Ok(unsafe { transmute::<&HIRFunction, &'static HIRFunction>(func) });
        }

        return Err(make_expected_simple_error(origin, &"function".to_string(), self).into());
    }

    pub fn modify_as_function<K: DiagnosticSpanOrigin, F>(
        &mut self,
        origin: &K,
        f: F,
    ) -> MaybeDiagnostic
    where
        F: FnOnce(&mut HIRFunction) -> (),
    {
        if let Self::Function(func) = self {
            f(func);
            return Ok(());
        }

        return Err(make_expected_simple_error(origin, &"function".to_string(), self).into());
    }

    pub fn modify_as_type<K: DiagnosticSpanOrigin, F>(
        &mut self,
        origin: &K,
        f: F,
    ) -> MaybeDiagnostic
    where
        F: FnOnce(&mut ScopeStoredType) -> (),
    {
        if let Self::Type(t) = self {
            f(t);
            return Ok(());
        }

        return Err(make_expected_simple_error(origin, &"function".to_string(), self).into());
    }
}

impl Display for ScopeEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Function(_) => "function",
            Self::Type(_) => "type",
        };

        write!(f, "{}", s)
    }
}
