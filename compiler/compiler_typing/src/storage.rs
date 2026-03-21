use compiler_errors::{IR_ALREADY_EXISTING_ELEM, IR_FIND_TYPE, errs::{BaseResult, base::BaseError}};
use compiler_utils::{hash, utils::indexed::IndexStorage};
use compiler_utils::hash::HashedString;

use crate::{TypeParameterContainer, enums::{RawEnumEntryContainer, RawEnumTypeContainer}, raw::RawType, references::TypeReference};

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
	pub types: IndexStorage<RawType>
}

impl TypeStorage {
	pub fn new() -> BaseResult<Self> {
		let mut storage = TypeStorage { types: IndexStorage::new() };

		storage.append(SIGNED_INTEGER_8, RawType::Integer(8, true))?;
		storage.append(SIGNED_INTEGER_16, RawType::Integer(16, true))?;
		storage.append(SIGNED_INTEGER_32, RawType::Integer(32, true))?;
		storage.append(SIGNED_INTEGER_64, RawType::Integer(64, true))?;
		storage.append(SIGNED_INTEGER_128, RawType::Integer(128, true))?;

		storage.append(UNSIGNED_INTEGER_8, RawType::Integer(8, false))?;
		storage.append(UNSIGNED_INTEGER_16, RawType::Integer(16, false))?;
		storage.append(UNSIGNED_INTEGER_32, RawType::Integer(32, false))?;
		storage.append(UNSIGNED_INTEGER_64, RawType::Integer(64, false))?;
		storage.append(UNSIGNED_INTEGER_128, RawType::Integer(128, false))?;

		storage.append(SIGNED_FLOATING_POINT_8, RawType::Floating(8, true))?;
		storage.append(SIGNED_FLOATING_POINT_16, RawType::Floating(16, true))?;
		storage.append(SIGNED_FLOATING_POINT_32, RawType::Floating(32, true))?;
		storage.append(SIGNED_FLOATING_POINT_64, RawType::Floating(64, true))?;
		storage.append(SIGNED_FLOATING_POINT_128, RawType::Floating(128, true))?;

		storage.append(UNSIGNED_FLOATING_POINT_8, RawType::Floating(8, false))?;
		storage.append(UNSIGNED_FLOATING_POINT_16, RawType::Floating(16, false))?;
		storage.append(UNSIGNED_FLOATING_POINT_32, RawType::Floating(32, false))?;
		storage.append(UNSIGNED_FLOATING_POINT_64, RawType::Floating(64, false))?;
		storage.append(UNSIGNED_FLOATING_POINT_128, RawType::Floating(128, false))?;

		storage.append(SIGNED_FIXED_POINT_8, RawType::FixedPoint(4, 4, true))?;
		storage.append(SIGNED_FIXED_POINT_16, RawType::FixedPoint(8, 8, true))?;
		storage.append(SIGNED_FIXED_POINT_32, RawType::FixedPoint(16, 16, true))?;
		storage.append(SIGNED_FIXED_POINT_64, RawType::FixedPoint(32, 32, true))?;
		storage.append(SIGNED_FIXED_POINT_128, RawType::FixedPoint(64, 64, true))?;

		storage.append(UNSIGNED_FIXED_POINT_8, RawType::FixedPoint(4, 4, false))?;
		storage.append(UNSIGNED_FIXED_POINT_16, RawType::FixedPoint(8, 8, false))?;
		storage.append(UNSIGNED_FIXED_POINT_32, RawType::FixedPoint(16, 16, false))?;
		storage.append(UNSIGNED_FIXED_POINT_64, RawType::FixedPoint(32, 32, false))?;
		storage.append(UNSIGNED_FIXED_POINT_128, RawType::FixedPoint(64, 64, false))?;
		
		storage.append(BOOLEAN_TYPE, RawType::Boolean)?;
		storage.append(POINTER_TYPE, RawType::Pointer)?;
		storage.append(STATIC_STR, RawType::StaticString)?;

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

	pub fn append(&mut self, hash: u64, base: RawType) -> BaseResult<usize> {
		if self.types.hash_to_ind.contains_key(&hash) {
			return Err(BaseError::err(IR_ALREADY_EXISTING_ELEM!().to_string()))
		}

		return Ok(self.types.append(hash, base));
	}

	pub fn get_type(&self, hash: u64) -> BaseResult<RawType> {
		if let Some(v) = self.types.get_index(hash) {
			return Ok(self.types.get_ind(v).clone());
		}

		return Err(BaseError::err(IR_FIND_TYPE!().to_string()))
	}
}