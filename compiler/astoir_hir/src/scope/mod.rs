//! Definitions for the scope based storage

use std::collections::HashMap;

use diagnostics::{DiagnosticResult, DiagnosticSpanOrigin, builders::make_already_in_scope};

use crate::scope::{entry::ScopeEntry, key::EntryKey};

pub mod entry;
pub mod key;

pub struct ScopeStorage {
    key_to_ind: HashMap<EntryKey, usize>,
    entries: Vec<ScopeEntry>,
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
}
