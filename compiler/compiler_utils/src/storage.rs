//! Element storage definitions

use std::collections::HashMap;

/// A storage allows to store elments based on key and values whilst preserving the order of both
pub struct Storage<K> {
    pub map: HashMap<String, usize>,
    pub entries: Vec<K>,
    pub keys: Vec<String>,
}

impl<K> Storage<K> {
    pub fn new() -> Self {
        Storage {
            map: HashMap::new(),
            entries: vec![],
            keys: vec![],
        }
    }

    /// Inserts an entry into the storage.
    ///
    /// Returns false if the entry is already present, true if not
    pub fn insert(&mut self, name: String, value: K) -> bool {
        if self.map.contains_key(&name) {
            return false;
        }

        self.map.insert(name.clone(), self.entries.len());
        self.entries.push(value);
        self.keys.push(name);

        true
    }

    pub fn get(&mut self, name: String) -> Option<&K> {
        let ind = self.map.get(&name);

        if let Some(ind) = ind {
            Some(&self.entries[*ind])
        } else {
            None
        }
    }

    pub fn get_from_index(&mut self, ind: usize) -> &K {
        &self.entries[ind]
    }

    pub fn get_key_from_index(&mut self, ind: usize) -> String {
        self.keys[ind].clone()
    }

    pub fn get_index(&mut self, name: String) -> Option<usize> {
        let ind = self.map.get(&name);

        if let Some(ind) = ind {
            Some(*ind)
        } else {
            None
        }
    }
}

impl<K: Clone> Storage<K> {
    pub fn get_as_clone(&mut self, name: String) -> Option<K> {
        let ind = self.map.get(&name);

        if let Some(ind) = ind {
            Some(self.entries[*ind].clone())
        } else {
            None
        }
    }

    pub fn get_clone_from_index(&mut self, ind: usize) -> K {
        self.entries[ind].clone()
    }
}

impl<K: Clone> Clone for Storage<K> {
    fn clone(&self) -> Self {
        Self {
            map: self.map.clone(),
            entries: self.entries.clone(),
            keys: self.keys.clone(),
        }
    }
}
