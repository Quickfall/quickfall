//! Storage implementations for types in AstoIR.

use std::collections::HashMap;

use compiler_errors::{IR_ALREADY_EXISTING_ELEM, errs::{BaseResult, base::BaseError}};
use compiler_utils::hash::SelfHash;

use crate::{base::BaseType, hashes::{SIGNED_INTEGER_8, SIGNED_INTEGER_16, SIGNED_INTEGER_32, SIGNED_INTEGER_64, SIGNED_INTEGER_128}};

pub struct TypeStorage {
	pub hash_to_ind: HashMap<SelfHash, usize>,
	pub types: Vec<BaseType>,

	curr_ind: usize
}

impl TypeStorage {
	pub fn new() -> BaseResult<Self> {
		let mut storage = TypeStorage { hash_to_ind: HashMap::new(), types: vec![], curr_ind: 0 };

		storage.register_type(SIGNED_INTEGER_8, BaseType::NumericIntegerType(8, true))?;
		storage.register_type(SIGNED_INTEGER_16, BaseType::NumericIntegerType(16, true))?;
		storage.register_type(SIGNED_INTEGER_32, BaseType::NumericIntegerType(32, true))?;
		storage.register_type(SIGNED_INTEGER_64, BaseType::NumericIntegerType(64, true))?;
		storage.register_type(SIGNED_INTEGER_128, BaseType::NumericIntegerType(128, true))?;

		return Ok(storage);
	}

	pub fn register_type(&mut self, hash: u64, base: BaseType) -> BaseResult<bool> {
		let id = SelfHash { hash };

		if self.hash_to_ind.contains_key(&id) {
			return Err(BaseError::err(IR_ALREADY_EXISTING_ELEM!().to_string()));
		}

		let type_ind = self.curr_ind;
		self.curr_ind += 1;

		self.hash_to_ind.insert(id, type_ind);
		self.types.push(base);
		
		Ok(true)
	}

}