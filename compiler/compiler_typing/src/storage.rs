use std::collections::HashMap;

use compiler_utils::{hash, utils::indexed::IndexStorage};
use compiler_utils::hash::HashedString;

use crate::{TypeParameterContainer, enums::{RawEnumTypeContainer}, raw::RawType, references::TypeReference};

pub const SIGNED_INTEGER_8: u64 = hash!("s8");
pub const SIGNED_INTEGER_16: u64 = hash!("s16");
pub const SIGNED_INTEGER_32: u64 = hash!("s32");
pub const SIGNED_INTEGER_64: u64 = hash!("s64");
pub const SIGNED_INTEGER_128: u64 = hash!("s128");
pub const SIGNED_INTEGER: u64 = hash!("s");

pub const UNSIGNED_INTEGER_8: u64 = hash!("u8");
pub const UNSIGNED_INTEGER_16: u64 = hash!("u16");
pub const UNSIGNED_INTEGER_32: u64 = hash!("u32");
pub const UNSIGNED_INTEGER_64: u64 = hash!("u64");
pub const UNSIGNED_INTEGER_128: u64 = hash!("u128");
pub const UNSIGNED_INTEGER: u64 = hash!("u");

pub const SIGNED_FLOATING_POINT_8: u64 = hash!("f8");
pub const SIGNED_FLOATING_POINT_16: u64 = hash!("f16");
pub const SIGNED_FLOATING_POINT_32: u64 = hash!("f32");
pub const SIGNED_FLOATING_POINT_64: u64 = hash!("f64");
pub const SIGNED_FLOATING_POINT_80: u64 = hash!("f80");
pub const SIGNED_FLOATING_POINT_128: u64 = hash!("f128");
pub const SIGNED_FLOATING_POINT: u64 = hash!("f");

pub const SIGNED_FIXED_POINT_8: u64 = hash!("x8");
pub const SIGNED_FIXED_POINT_16: u64 = hash!("x16");
pub const SIGNED_FIXED_POINT_32: u64 = hash!("x32");
pub const SIGNED_FIXED_POINT_64: u64 = hash!("x64");
pub const SIGNED_FIXED_POINT_128: u64 = hash!("x128");
pub const SIGNED_FIXED_POINT: u64 = hash!("x");

pub const UNSIGNED_FLOATING_POINT_8: u64 = hash!("uf8");
pub const UNSIGNED_FLOATING_POINT_16: u64 = hash!("uf16");
pub const UNSIGNED_FLOATING_POINT_32: u64 = hash!("uf32");
pub const UNSIGNED_FLOATING_POINT_64: u64 = hash!("uf64");
pub const UNSIGNED_FLOATING_POINT_80: u64 = hash!("uf80");
pub const UNSIGNED_FLOATING_POINT_128: u64 = hash!("uf128");
pub const UNSIGNED_FLOATING_POINT: u64 = hash!("uf");

pub const UNSIGNED_FIXED_POINT_8: u64 = hash!("ux8");
pub const UNSIGNED_FIXED_POINT_16: u64 = hash!("ux16");
pub const UNSIGNED_FIXED_POINT_32: u64 = hash!("ux32");
pub const UNSIGNED_FIXED_POINT_64: u64 = hash!("ux64");
pub const UNSIGNED_FIXED_POINT_128: u64 = hash!("ux128");
pub const UNSIGNED_FIXED_POINT: u64 = hash!("ux");

pub const STATIC_STR: u64 = hash!("staticstr");

pub const POINTER_TYPE: u64 = hash!("ptr");
pub const BOOLEAN_TYPE: u64 = hash!("bool");

/// Experimental
pub const RESULT_TYPE: u64 = hash!("result");

#[derive(Debug)]
pub struct TypeStorage {
	pub types: IndexStorage<RawType>,
	pub type_to_ind: HashMap<RawType, usize>
}

impl TypeStorage {
	#[must_use = "must handle errors outside"]
	pub fn new() -> Result<Self, ()> {
		let mut storage = TypeStorage { types: IndexStorage::new(), type_to_ind: HashMap::new() };

		storage.append_with_hash(SIGNED_INTEGER_8, RawType::Integer(8, true))?;
		storage.append_with_hash(SIGNED_INTEGER_16, RawType::Integer(16, true))?;
		storage.append_with_hash(SIGNED_INTEGER_32, RawType::Integer(32, true))?;
		storage.append_with_hash(SIGNED_INTEGER_64, RawType::Integer(64, true))?;
		storage.append_with_hash(SIGNED_INTEGER_128, RawType::Integer(128, true))?;

		storage.append_with_hash(UNSIGNED_INTEGER_8, RawType::Integer(8, false))?;
		storage.append_with_hash(UNSIGNED_INTEGER_16, RawType::Integer(16, false))?;
		storage.append_with_hash(UNSIGNED_INTEGER_32, RawType::Integer(32, false))?;
		storage.append_with_hash(UNSIGNED_INTEGER_64, RawType::Integer(64, false))?;
		storage.append_with_hash(UNSIGNED_INTEGER_128, RawType::Integer(128, false))?;

		storage.append_with_hash(SIGNED_FLOATING_POINT_8, RawType::Floating(8, true))?;
		storage.append_with_hash(SIGNED_FLOATING_POINT_16, RawType::Floating(16, true))?;
		storage.append_with_hash(SIGNED_FLOATING_POINT_32, RawType::Floating(32, true))?;
		storage.append_with_hash(SIGNED_FLOATING_POINT_64, RawType::Floating(64, true))?;
		storage.append_with_hash(SIGNED_FLOATING_POINT_128, RawType::Floating(128, true))?;

		storage.append_with_hash(UNSIGNED_FLOATING_POINT_8, RawType::Floating(8, false))?;
		storage.append_with_hash(UNSIGNED_FLOATING_POINT_16, RawType::Floating(16, false))?;
		storage.append_with_hash(UNSIGNED_FLOATING_POINT_32, RawType::Floating(32, false))?;
		storage.append_with_hash(UNSIGNED_FLOATING_POINT_64, RawType::Floating(64, false))?;
		storage.append_with_hash(UNSIGNED_FLOATING_POINT_128, RawType::Floating(128, false))?;

		storage.append_with_hash(SIGNED_FIXED_POINT_8, RawType::FixedPoint(4, 4, true))?;
		storage.append_with_hash(SIGNED_FIXED_POINT_16, RawType::FixedPoint(8, 8, true))?;
		storage.append_with_hash(SIGNED_FIXED_POINT_32, RawType::FixedPoint(16, 16, true))?;
		storage.append_with_hash(SIGNED_FIXED_POINT_64, RawType::FixedPoint(32, 32, true))?;
		storage.append_with_hash(SIGNED_FIXED_POINT_128, RawType::FixedPoint(64, 64, true))?;

		storage.append_with_hash(UNSIGNED_FIXED_POINT_8, RawType::FixedPoint(4, 4, false))?;
		storage.append_with_hash(UNSIGNED_FIXED_POINT_16, RawType::FixedPoint(8, 8, false))?;
		storage.append_with_hash(UNSIGNED_FIXED_POINT_32, RawType::FixedPoint(16, 16, false))?;
		storage.append_with_hash(UNSIGNED_FIXED_POINT_64, RawType::FixedPoint(32, 32, false))?;
		storage.append_with_hash(UNSIGNED_FIXED_POINT_128, RawType::FixedPoint(64, 64, false))?;
		
		storage.append_with_hash(BOOLEAN_TYPE, RawType::Boolean)?;
		storage.append_with_hash(POINTER_TYPE, RawType::Pointer)?;
		storage.append_with_hash(STATIC_STR, RawType::StaticString)?;

		{

			let mut type_params = TypeParameterContainer::new();

			type_params.insert(HashedString::new("V".to_string()), 0);
			type_params.insert(HashedString::new("E".to_string()), 1);

			let mut result_enum = RawEnumTypeContainer::new(storage.types.vals.len(), type_params);

			result_enum.append_entry(HashedString::new("value".to_string()), vec![(hash!("val"), TypeReference::make_unresolved(0))]);
			result_enum.append_entry(HashedString::new("error".to_string()), vec![(hash!("err"), TypeReference::make_unresolved(1))]);

			storage.append(RESULT_TYPE, RawType::Enum(result_enum))?;
		}
		
		return Ok(storage);
	}

	#[must_use = "must handle errors outside"]
	pub fn append_with_hash(&mut self, hash: u64, base: RawType) -> Result<usize, ()> {
		if self.types.hash_to_ind.contains_key(&hash) {
			return Err(())
		}

		let res = self.types.append(hash, base.clone());

		self.type_to_ind.insert(base, res);

		return Ok(res);
	}

	#[must_use = "must handle errors outside"]
	pub fn append(&mut self, hash: u64, base: RawType) -> Result<usize, ()> {
		if self.types.hash_to_ind.contains_key(&hash) {
			return Err(())
		}

		let res = self.types.append(hash, base.clone());

		return Ok(res);
	}

	#[must_use = "must handle errors outside"]
	pub fn get_type(&self, hash: u64) -> Result<RawType, ()> {
		if let Some(v) = self.types.get_index(hash) {
			return Ok(self.types.get_ind(v).clone());
		}

		return Err(())
	}
}