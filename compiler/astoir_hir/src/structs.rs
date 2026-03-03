//! AstoIR HIR structures related to HIR nodes

use std::collections::HashMap;

use astoir_typing::complete::CompleteType;
use compiler_errors::{IR_FIELD, errs::{BaseResult, base::BaseError}};
use compiler_utils::hash::SelfHash;

pub struct StructTypeContainer {
	pub hash_to_ind: HashMap<SelfHash, usize>,
	pub fields: Vec<CompleteType>
}

impl StructTypeContainer {
	pub fn new() -> Self {
		return StructTypeContainer { hash_to_ind: HashMap::new(), fields: vec![] }
	}

	pub fn append(&mut self, hash: u64, v: CompleteType) {
		let sz = self.fields.len();

		self.hash_to_ind.insert(SelfHash { hash }, sz);
		self.fields.push(v);
	}

	pub fn get(&mut self, hash: u64) -> BaseResult<&CompleteType> {
		let index = match self.hash_to_ind.get(&SelfHash { hash }) {
			Some(v) => *v,
			None => return Err(BaseError::err(IR_FIELD!().to_string()))
		};

		return Ok(&self.fields[index])
	}

}