//! Utilities for indexed storage as this is a common thing used in Quickfall

use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IndexStorage<K> {
	pub hash_to_ind: HashMap<u64, usize>, 
	pub entry_keys: Vec<u64>,
	pub vals: Vec<K>
}

impl<K> IndexStorage<K> {
	pub fn new() -> Self {
		return IndexStorage { hash_to_ind: HashMap::new(), vals: vec![], entry_keys: vec![] }
	}

	pub fn append(&mut self, hash: u64, v: K) -> usize {
		let ind = self.vals.len();

		self.entry_keys.push(hash);

		self.hash_to_ind.insert(hash, ind);
		self.vals.push(v);

		return ind;
	}

	pub fn get_index(&self, hash: u64) -> Option<usize> {
		return self.hash_to_ind.get(&hash).copied();
	}

	pub fn get_ind(&self, ind: usize) -> &K {
		return &self.vals[ind];
	}



}