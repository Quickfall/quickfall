//! Definitions for the scope based storage

use std::collections::HashMap;

use diagnostics::{
    DiagnosticResult, DiagnosticSpanOrigin, MaybeDiagnostic,
    builders::{make_already_in_scope, make_cannot_find},
};

use crate::{
    func::HIRFunction,
    scope::{entry::ScopeEntry, key::EntryKey},
    types::ScopeStoredType,
};

pub mod entry;
pub mod key;

pub struct ScopeStorage {
    key_to_ind: HashMap<EntryKey, usize>,
    pub entries: Vec<ScopeEntry>,
}

impl ScopeStorage {
    pub fn new() -> Self {
        ScopeStorage {
            key_to_ind: HashMap::new(),
            entries: vec![],
        }
    }

    pub fn append<K: DiagnosticSpanOrigin>(
        &mut self,
        key: EntryKey,
        val: ScopeEntry,
        origin: &K,
    ) -> DiagnosticResult<usize> {
        if self.key_to_ind.contains_key(&key) {
            return Err(make_already_in_scope(origin, &key).into());
        }

        let ind = self.entries.len();

        self.key_to_ind.insert(key, ind);
        self.entries.push(val);

        Ok(ind)
    }

    pub fn get<K: DiagnosticSpanOrigin>(
        &self,
        key: &EntryKey,
        origin: &K,
    ) -> DiagnosticResult<&ScopeEntry> {
        if !self.key_to_ind.contains_key(key) {
            return Err(make_cannot_find(origin, key).into());
        }

        let ind = &self.key_to_ind[key];

        Ok(&self.entries[*ind])
    }

    pub fn get_function<K: DiagnosticSpanOrigin>(
        &self,
        key: &EntryKey,
        origin: &K,
    ) -> DiagnosticResult<&'static HIRFunction> {
        self.get(key, origin)?.as_function(origin)
    }

    pub fn get_type<K: DiagnosticSpanOrigin>(
        &self,
        key: &EntryKey,
        origin: &K,
    ) -> DiagnosticResult<&'static ScopeStoredType> {
        self.get(key, origin)?.as_type(origin)
    }

    fn get_modify<K: DiagnosticSpanOrigin>(
        &mut self,
        key: &EntryKey,
        origin: &K,
    ) -> DiagnosticResult<&mut ScopeEntry> {
        if !self.key_to_ind.contains_key(key) {
            return Err(make_cannot_find(origin, key).into());
        }

        let ind = &self.key_to_ind[key];

        Ok(&mut self.entries[*ind])
    }

    pub fn modify_function<K: DiagnosticSpanOrigin, F>(
        &mut self,
        key: &EntryKey,
        origin: &K,
        f: F,
    ) -> MaybeDiagnostic
    where
        F: FnOnce(&mut HIRFunction) -> (),
    {
        self.get_modify(key, origin)?.modify_as_function(origin, f)
    }

    pub fn modify_type<K: DiagnosticSpanOrigin, F>(
        &mut self,
        key: &EntryKey,
        origin: &K,
        f: F,
    ) -> MaybeDiagnostic
    where
        F: FnOnce(&mut ScopeStoredType) -> (),
    {
        self.get_modify(key, origin)?.modify_as_type(origin, f)
    }
}
