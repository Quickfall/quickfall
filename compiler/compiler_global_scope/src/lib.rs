use std::{collections::HashMap, hash::Hash};

use diagnostics::{
    DiagnosticResult, DiagnosticSpanOrigin,
    builders::{make_already_in_scope, make_cannot_find, make_expected_simple_error},
};

use crate::{
    entry::{GlobalStorageEntry, GlobalStorageEntryType},
    key::EntryKey,
};

pub mod entry;
pub mod key;

pub type GlobalStorageIdentifier = usize;

#[derive(Debug)]
pub struct GlobalScopeStorage<T: Hash, R: Hash> {
    pub entry_to_ind: HashMap<EntryKey, usize>,
    pub entries: Vec<GlobalStorageEntry<T, R>>,

    pub value_to_ind: HashMap<GlobalStorageEntryType<T, R>, usize>,

    pub function_counter: usize,
}

/// The global storage for every element inside of the scope.
///
/// This stores the following:
/// - Functions (with or without implementations)
/// - Static variables
/// - Struct functions
/// - Types
///
/// # Safety
/// The `GlobalScopeStorage` enforces correctness for global scope types and strictly allows only one entry per name. globally.
impl<T: Clone + Hash + Eq, R: Clone + Hash + Eq> GlobalScopeStorage<T, R> {
    pub fn new() -> Self {
        GlobalScopeStorage {
            entry_to_ind: HashMap::new(),
            value_to_ind: HashMap::new(),
            entries: vec![],
            function_counter: 0,
        }
    }

    pub fn append<K: DiagnosticSpanOrigin>(
        &mut self,
        name: EntryKey,
        entry: GlobalStorageEntryType<T, R>,
        origin: &K,
    ) -> DiagnosticResult<usize> {
        if self.entry_to_ind.contains_key(&name) {
            return Err(make_already_in_scope(origin, &name.name_hash).into());
        }

        if let GlobalStorageEntryType::Function { .. } = entry {
            self.function_counter += 1;
        }

        let parent_index = self.entries.len();

        self.value_to_ind.insert(entry.clone(), parent_index);

        let entry = GlobalStorageEntry {
            entry_type: entry,
            parent_index,
        };

        self.entries.push(entry);
        self.entry_to_ind.insert(name, parent_index);

        Ok(parent_index)
    }

    pub fn get_base<K: DiagnosticSpanOrigin>(
        &self,
        name: EntryKey,
        origin: &K,
    ) -> DiagnosticResult<GlobalStorageEntryType<T, R>> {
        if !self.entry_to_ind.contains_key(&name) {
            return Err(make_cannot_find(origin, &name.name_hash).into());
        }

        return Ok(self.entries[self.entry_to_ind[&name]].entry_type.clone());
    }

    pub fn get_type<K: DiagnosticSpanOrigin>(
        &self,
        name: EntryKey,
        origin: &K,
    ) -> DiagnosticResult<R> {
        let base = self.get_base(name, origin)?;

        return match base {
            GlobalStorageEntryType::Type(t) => Ok(t.clone()),
            _ => Err(make_expected_simple_error(origin, &"type".to_string(), &base).into()),
        };
    }

    pub fn get_static_variable<K: DiagnosticSpanOrigin>(
        &self,
        name: EntryKey,
        origin: &K,
    ) -> DiagnosticResult<T> {
        let base = self.get_base(name, origin)?;

        return match base {
            GlobalStorageEntryType::StaticVariable(t) => Ok(t.clone()),
            _ => Err(
                make_expected_simple_error(origin, &"static variable".to_string(), &base).into(),
            ),
        };
    }

    pub fn get_function<K: DiagnosticSpanOrigin>(
        &self,
        name: EntryKey,
        origin: &K,
    ) -> DiagnosticResult<usize> {
        let base = self.get_base(name, origin)?;

        return match base {
            GlobalStorageEntryType::Function(ind) => Ok(ind),
            _ => Err(make_expected_simple_error(origin, &"function".to_string(), &base).into()),
        };
    }
}
