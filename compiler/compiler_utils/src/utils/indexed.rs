use std::collections::HashMap;

use crate::hash::SelfHash;

pub struct IndexStorage<K> {
	pub hash_to_ind: HashMap<SelfHash, usize>,
	pub storage: Vec<K>,

	pub curr_ind: usize
}

impl<K> IndexStorage<K> {
	pub fn new() -> Self {
		IndexStorage { hash_to_ind: HashMap::new(), storage: Vec::new(), curr_ind: 0 }
	}

	pub fn append(&mut self, 
}