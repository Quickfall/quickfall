//! IR Type structures

use std::{cell::Ref, collections::HashMap};

use inkwell::types::IntType;

/// Types of IR variables
pub enum IRType<'a> {
	Signed8(IntType<'a>),
	Signed16(IntType<'a>),
	Signed32(IntType<'a>),
	Signed64(IntType<'a>), 
	Signed128(IntType<'a>),

	Unsigned8(IntType<'a>),
	Unsigned16(IntType<'a>),
	Unsigned32(IntType<'a>),
	Unsigned64(IntType<'a>),
	Unsigned128(IntType<'a>),

	Bool(IntType<'a>),
	
	Struct(HashMap<String, Ref<'a, IRType<'a>>>), // fields
	Layout(HashMap<String, Ref<'a, IRType<'a>>>) // fields
}

impl IRType<'_> {
	/// Gets the size in bits of a given IR element
	pub fn get_bitsize(&self) -> usize {
		match self {
			IRType::Signed8(_) | IRType::Unsigned8(_) | IRType::Bool(_) => return 8, 
			IRType::Signed16(_) | IRType::Unsigned16(_) => return 16,
			IRType::Signed32(_) | IRType::Unsigned32(_) => return 32, 
			IRType::Signed64(_) | IRType::Unsigned64(_) => return 64, 
			IRType::Signed128(_) | IRType::Unsigned128(_) => return 128,

			IRType::Struct(v) => {
				let mut sz: usize = 0;

				// TODO: add bool compacting

				for (_, ir_type) in v {
					sz += ir_type.get_bitsize();					
				}

				return sz;
 			},

			IRType::Layout(v) => {
				let mut sz: usize = 0;

				for (_, ir_type) in v {
					sz += ir_type.get_bitsize();
				}

				return sz;
			}
		}
	}

	/// Determines if the given IR type is a numeric based type
	pub fn is_numeric_type(&self) -> bool {
		match self {
			IRType::Signed8(_) | IRType::Signed16(_) | IRType::Signed32(_) | IRType::Signed64(_) | IRType::Signed128(_) |
			IRType::Unsigned8(_) | IRType::Unsigned16(_) | IRType::Unsigned32(_) | IRType::Unsigned64(_) | IRType::Unsigned128(_) => {
				return true;
			},

			_ => return false
		};
	}

	pub fn is_signed(&self) -> bool {
		match self {
			IRType::Signed8(_) | IRType::Signed16(_) | IRType::Signed32(_) | IRType::Signed64(_) | IRType::Signed128(_) => {
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