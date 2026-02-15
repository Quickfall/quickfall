//! IR Type structures

use std::{cell::Ref, collections::HashMap, ops::Add};

use commons::err::{PositionlessError, PositionlessResult};
use inkwell::{AddressSpace, builder::Builder, context::Context, types::{BasicMetadataTypeEnum, BasicType, FunctionType, IntType, PointerType, StringRadix}, values::PointerValue};

use crate::values::IRValue;

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

	Pointer(PointerType<'a>),

	Bool(IntType<'a>),
	
	Struct(HashMap<String, Ref<'a, IRType<'a>>>), // fields
	Layout(HashMap<String, Ref<'a, IRType<'a>>>) // fields
}

impl<'a> IRType<'a> {
	/// Gets the size in bits of a given IR element
	pub fn get_bitsize(&self) -> usize {
		match self {
			IRType::Signed8(_) | IRType::Unsigned8(_) | IRType::Bool(_) => return 8, 
			IRType::Signed16(_) | IRType::Unsigned16(_) => return 16,
			IRType::Signed32(_) | IRType::Unsigned32(_) => return 32, 
			IRType::Signed64(_) | IRType::Unsigned64(_) => return 64, 
			IRType::Signed128(_) | IRType::Unsigned128(_) => return 128,
			IRType::Pointer(_) => return 64,

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

	pub fn get_inkwell_basetype(&self) -> PositionlessResult<BasicMetadataTypeEnum<'a>> {
		match self {
			IRType::Unsigned8(v) => Ok(BasicMetadataTypeEnum::from(*v)),
			IRType::Unsigned16(v) => Ok(BasicMetadataTypeEnum::from(*v)),
			IRType::Unsigned32(v) => Ok(BasicMetadataTypeEnum::from(*v)),
			IRType::Unsigned64(v) => Ok(BasicMetadataTypeEnum::from(*v)),
			IRType::Unsigned128(v) => Ok(BasicMetadataTypeEnum::from(*v)),
			IRType::Signed8(v) => Ok(BasicMetadataTypeEnum::from(*v)),
			IRType::Signed16(v) => Ok(BasicMetadataTypeEnum::from(*v)),
			IRType::Signed32(v) => Ok(BasicMetadataTypeEnum::from(*v)),
			IRType::Signed64(v) => Ok(BasicMetadataTypeEnum::from(*v)),
			IRType::Signed128(v) => Ok(BasicMetadataTypeEnum::from(*v)),

			IRType::Pointer(v) => Ok(BasicMetadataTypeEnum::from(*v)),
			
			_ => Err(PositionlessError::new("Given IR type doesn't have any Inkwell type!!!"))
		}
	}

	pub fn get_inkwell_inttype(&self) -> PositionlessResult<&IntType<'a>> {
		match self {
			IRType::Unsigned8(v) => Ok(v),
			IRType::Unsigned16(v) => Ok(v),
			IRType::Unsigned32(v) => Ok(v),
			IRType::Unsigned64(v) => Ok(v),
			IRType::Unsigned128(v) => Ok(v),
			IRType::Signed8(v) => Ok(v),
			IRType::Signed16(v) => Ok(v),
			IRType::Signed32(v) => Ok(v),
			IRType::Signed64(v) => Ok(v),
			IRType::Signed128(v) => Ok(v),

			_ => return Err(PositionlessError::new("get_inkwell_inttype was used with a non int typed IRType!"))
		}
	}

	pub fn get_inkwell_pointertype(ctx: &'a Context) -> PointerType<'a> {
		return ctx.ptr_type(AddressSpace::from(0u16));
	}

	pub fn make_numeric_stackvar(&self, builder: &Builder<'a>, name: String, initial_val: IRValue) -> PositionlessResult<PointerValue<'a>> {
		let t = *self.get_inkwell_inttype()?;
		let alloca = match builder.build_alloca(t, &name) {
			Ok(v) => v,
			Err(_) => return Err(PositionlessError::new("build_alloca failed!!"))
		};

		let v = initial_val.expects_numeric_value(self.get_bitsize(), self.is_signed())?;

		let val = match t.const_int_from_string(&v.to_string(), StringRadix::Decimal) {
			Some(v) => v,
			None => return Err(PositionlessError::new("const_int_from_string failed!!"))
		};
	
		
	 	if builder.build_store(alloca, val).is_err() {
			return Err(PositionlessError::new("build_store failed!!"));
		}

		return Ok(alloca);
	}

}