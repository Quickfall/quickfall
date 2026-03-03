//! Storage implementations for types in AstoIR.

use std::collections::HashMap;

use compiler_errors::{IR_ALREADY_EXISTING_ELEM, errs::{BaseResult, base::BaseError}};
use compiler_utils::hash::SelfHash;

use crate::{base::BaseType, hashes::{BOOLEAN_TYPE, INTERNAL_8, POINTER_TYPE, SIGNED_FIXED_POINT_8, SIGNED_FIXED_POINT_16, SIGNED_FIXED_POINT_32, SIGNED_FIXED_POINT_64, SIGNED_FIXED_POINT_128, SIGNED_FLOATING_POINT_8, SIGNED_FLOATING_POINT_16, SIGNED_FLOATING_POINT_32, SIGNED_FLOATING_POINT_64, SIGNED_FLOATING_POINT_128, SIGNED_INTEGER_8, SIGNED_INTEGER_16, SIGNED_INTEGER_32, SIGNED_INTEGER_64, SIGNED_INTEGER_128, UNSIGNED_FIXED_POINT_8, UNSIGNED_FIXED_POINT_16, UNSIGNED_FIXED_POINT_32, UNSIGNED_FIXED_POINT_64, UNSIGNED_FIXED_POINT_128, UNSIGNED_FLOATING_POINT_8, UNSIGNED_FLOATING_POINT_16, UNSIGNED_FLOATING_POINT_32, UNSIGNED_FLOATING_POINT_64, UNSIGNED_FLOATING_POINT_128, UNSIGNED_INTEGER_8, UNSIGNED_INTEGER_16, UNSIGNED_INTEGER_32, UNSIGNED_INTEGER_64, UNSIGNED_INTEGER_128}};

pub struct TypeStorage {
	pub hash_to_ind: HashMap<SelfHash, usize>,
	pub types: Vec<BaseType>,
	pub unsupported: Vec<bool>,

	curr_ind: usize
}

impl TypeStorage {
	pub fn new() -> BaseResult<Self> {
		let mut storage = TypeStorage { hash_to_ind: HashMap::new(), types: vec![], unsupported: vec![], curr_ind: 0 };

		storage.register_type(BOOLEAN_TYPE, BaseType::Boolean)?;
		storage.register_type(POINTER_TYPE, BaseType::Pointer)?;
		storage.register_type(INTERNAL_8, BaseType::ArbitraryType(8))?;

		storage.register_type(SIGNED_INTEGER_8, BaseType::NumericIntegerType(8, true))?;
		storage.register_type(SIGNED_INTEGER_16, BaseType::NumericIntegerType(16, true))?;
		storage.register_type(SIGNED_INTEGER_32, BaseType::NumericIntegerType(32, true))?;
		storage.register_type(SIGNED_INTEGER_64, BaseType::NumericIntegerType(64, true))?;
		storage.register_type(SIGNED_INTEGER_128, BaseType::NumericIntegerType(128, true))?;

		storage.register_type(UNSIGNED_INTEGER_8, BaseType::NumericIntegerType(8, false))?;
		storage.register_type(UNSIGNED_INTEGER_16, BaseType::NumericIntegerType(16, false))?;
		storage.register_type(UNSIGNED_INTEGER_32, BaseType::NumericIntegerType(32, false))?;
		storage.register_type(UNSIGNED_INTEGER_64, BaseType::NumericIntegerType(64, false))?;
		storage.register_type(UNSIGNED_INTEGER_128, BaseType::NumericIntegerType(128, false))?;	

		storage.register_type(SIGNED_FLOATING_POINT_8, BaseType::FloatingNumberType(4, 4, true))?;
		storage.register_type(SIGNED_FLOATING_POINT_16, BaseType::FloatingNumberType(5, 11, true))?;
		storage.register_type(SIGNED_FLOATING_POINT_32, BaseType::FloatingNumberType(8, 25, true))?;
		storage.register_type(SIGNED_FLOATING_POINT_64, BaseType::FloatingNumberType(11, 53, true))?;
		storage.register_type(SIGNED_FLOATING_POINT_128, BaseType::FloatingNumberType(15, 113, true))?;

		storage.register_type(UNSIGNED_FLOATING_POINT_8, BaseType::FloatingNumberType(4, 4, false))?;
		storage.register_type(UNSIGNED_FLOATING_POINT_16, BaseType::FloatingNumberType(5, 11, false))?;
		storage.register_type(UNSIGNED_FLOATING_POINT_32, BaseType::FloatingNumberType(8, 25, false))?;
		storage.register_type(UNSIGNED_FLOATING_POINT_64, BaseType::FloatingNumberType(11, 53, false))?;
		storage.register_type(UNSIGNED_FLOATING_POINT_128, BaseType::FloatingNumberType(15, 113, false))?;

		storage.register_type(SIGNED_FIXED_POINT_8, BaseType::FixedPointNumberType(4, 4, true))?;
		storage.register_type(SIGNED_FIXED_POINT_16, BaseType::FixedPointNumberType(8, 8, true))?;
		storage.register_type(SIGNED_FIXED_POINT_32, BaseType::FixedPointNumberType(16, 16, true))?;
		storage.register_type(SIGNED_FIXED_POINT_64, BaseType::FixedPointNumberType(32, 32, true))?;
		storage.register_type(SIGNED_FIXED_POINT_128, BaseType::FixedPointNumberType(64, 64, true))?;

		storage.register_type(UNSIGNED_FIXED_POINT_8, BaseType::FixedPointNumberType(4, 4, false))?;
		storage.register_type(UNSIGNED_FIXED_POINT_16, BaseType::FixedPointNumberType(8, 8, false))?;
		storage.register_type(UNSIGNED_FIXED_POINT_32, BaseType::FixedPointNumberType(16, 16, false))?;
		storage.register_type(UNSIGNED_FIXED_POINT_64, BaseType::FixedPointNumberType(32, 32, false))?;
		storage.register_type(UNSIGNED_FIXED_POINT_128, BaseType::FixedPointNumberType(64, 64, false))?;

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