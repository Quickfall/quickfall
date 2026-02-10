//! IR Type structures

use std::{cell::Ref, collections::HashMap};

/// Types of IR variables
pub enum IRType<'a> {
	Signed8,
	Signed16,
	Signed32,
	Signed64, 
	Signed128,

	Unsigned8,
	Unsigned16,
	Unsigned32,
	Unsigned64,
	Unsigned128,

	Bool,
	
	Struct(HashMap<String, Ref<'a, IRType<'a>>>), // fields
	Layout(HashMap<String, Ref<'a, IRType<'a>>>) // fields
}

impl IRType<'_> {
	/// Gets the size in bits of a given IR element
	pub fn get_bitsize(&self) -> usize {
		match self {
			IRType::Signed8 | IRType::Unsigned8 | IRType::Bool => return 8, 
			IRType::Signed16 | IRType::Unsigned16 => return 16,
			IRType::Signed32 | IRType::Unsigned32 => return 32, 
			IRType::Signed64 | IRType::Unsigned64 => return 64, 
			IRType::Signed128 | IRType::Unsigned128 => return 128,

			IRType::Struct(v) => {
				let mut sz: usize = 0;

				// TODO: add bool compacting

				for (field_name, ir_type) in v {
					sz += ir_type.get_bitsize();					
				}

				return sz;
 			},

			IRType::Layout(v) => {
				let mut sz: usize = 0;

				for (field_name, ir_type) in v {
					sz += ir_type.get_bitsize();
				}

				return sz;
			}
		}
	}

	/// Determines if the given IR type is a numeric based type
	pub fn is_numeric_type(&self) -> bool {
		match self {
			IRType::Signed8 | IRType::Signed16 | IRType::Signed32 | IRType::Signed64 | IRType::Signed128 |
			IRType::Unsigned8 | IRType::Unsigned16 | IRType::Unsigned32 | IRType::Unsigned64 | IRType::Unsigned128 => {
				return true;
			},

			_ => return false
		};
	}

	pub fn is_signed(&self) -> bool {
		match self {
			IRType::Signed8 | IRType::Signed16 | IRType::Signed32 | IRType::Signed64 | IRType::Signed128 => {
				return true;
			},

			_ => return false
		};
	}

	pub fn get_numeric_high_bound(&self) -> i128 {
		if !self.is_numeric_type() {
			return 0;
		}

		if self.is_signed() {
			return 2_i128.pow((self.get_bitsize() - 1) as u32) - 1;
		}

		return 2_i128.pow(self.get_bitsize() as u32) - 1;
	}

	pub fn get_numeric_low_bound(&self) -> i128 {
		if !self.is_numeric_type() {
			return 0;
		}

		if self.is_signed() {
			return  -2_i128.pow((self.get_bitsize() - 1) as u32) - 1;
		}

		return -2_i128.pow(self.get_bitsize() as u32) - 1;
	}
}