//! IR Type structures

use std::{cell::Ref, collections::HashMap, mem::transmute, ops::Add, rc::Rc};

use commons::err::{PositionlessError, PositionlessResult};
use inkwell::{AddressSpace, builder::Builder, context::Context, types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum, FunctionType, IntType, PointerType, StringRadix}, values::{BasicValueEnum, GlobalValue, IntValue, PointerValue}};

use crate::{ctx::IRContext, irstruct::structs::IRStructuredType, utils::OwnedType, values::IRValue};

pub type OwnedIntType = OwnedType<IntType<'static>>;
pub type OwnedPointerType = OwnedType<PointerType<'static>>;
pub type OwnedTypeEnum = OwnedType<BasicTypeEnum<'static>>;
pub type OwnedMetadataTypeEnum = OwnedType<BasicMetadataTypeEnum<'static>>;

pub type OwnedValueEnum = OwnedType<BasicValueEnum<'static>>;
pub type OwnedIntValue = OwnedType<IntValue<'static>>;
pub type OwnedPointerValue = OwnedType<PointerValue<'static>>;

pub type OwnedGlobalValue = OwnedType<GlobalValue<'static>>;

/// Types of IR variables
#[derive(Clone)]
pub enum IRType {
	Signed8(OwnedIntType),
	Signed16(OwnedIntType),
	Signed32(OwnedIntType),
	Signed64(OwnedIntType), 
	Signed128(OwnedIntType),

	Unsigned8(OwnedIntType),
	Unsigned16(OwnedIntType),
	Unsigned32(OwnedIntType),
	Unsigned64(OwnedIntType),
	Unsigned128(OwnedIntType),

	Pointer(OwnedPointerType),

	Bool(OwnedIntType),
	
	Struct(Rc<IRStructuredType>),
	Layout(Rc<IRStructuredType>)
}

impl IRType {
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

				for t in &v.field_types {
					sz += t.get_bitsize();
				}

				return sz;
 			},

			IRType::Layout(v) => {
				let mut sz: usize = 0;

				for t in &v.field_types {
					sz += t.get_bitsize();
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

	pub fn get_inkwell_basetype(&self) -> PositionlessResult<OwnedTypeEnum> {
		match self {
			IRType::Unsigned8(v) => Ok(OwnedTypeEnum::new(&v.owned, BasicTypeEnum::from(v.inner))),
			IRType::Unsigned16(v) => Ok(OwnedTypeEnum::new(&v.owned, BasicTypeEnum::from(v.inner))),
			IRType::Unsigned32(v) => Ok(OwnedTypeEnum::new(&v.owned, BasicTypeEnum::from(v.inner))),
			IRType::Unsigned64(v) => Ok(OwnedTypeEnum::new(&v.owned, BasicTypeEnum::from(v.inner))),
			IRType::Unsigned128(v) => Ok(OwnedTypeEnum::new(&v.owned, BasicTypeEnum::from(v.inner))),
			IRType::Signed8(v) => Ok(OwnedTypeEnum::new(&v.owned, BasicTypeEnum::from(v.inner))),
			IRType::Signed16(v) => Ok(OwnedTypeEnum::new(&v.owned, BasicTypeEnum::from(v.inner))),
			IRType::Signed32(v) => Ok(OwnedTypeEnum::new(&v.owned, BasicTypeEnum::from(v.inner))),
			IRType::Signed64(v) => Ok(OwnedTypeEnum::new(&v.owned, BasicTypeEnum::from(v.inner))),
			IRType::Signed128(v) => Ok(OwnedTypeEnum::new(&v.owned, BasicTypeEnum::from(v.inner))),

			IRType::Pointer(v) => Ok(OwnedTypeEnum::new(&v.owned, BasicTypeEnum::from(v.inner))),

			IRType::Struct(a) => Ok(OwnedTypeEnum::new(&a.owned, BasicTypeEnum::from(a.inkwell_type))),
			IRType::Layout(a) => Ok(OwnedTypeEnum::new(&a.owned, BasicTypeEnum::from(a.inkwell_type))),
			
			_ => Err(PositionlessError::new("Given IR type doesn't have any Inkwell type!!!"))
		}
	}

	pub fn get_inkwell_instance_basetype(&self, ctx: &IRContext) -> PositionlessResult<OwnedTypeEnum> {
		match self {
			IRType::Struct(_) | IRType::Layout(_) => Ok(OwnedTypeEnum::new(&ctx.inkwell_ctx, ctx.ptr_type.into())),
			_ => self.get_inkwell_basetype()
		}
	}

	pub fn get_inkwell_base_metadatatype(&self) -> PositionlessResult<OwnedMetadataTypeEnum> {
		match self {
			IRType::Unsigned8(v) => Ok(OwnedMetadataTypeEnum::new(&v.owned, BasicMetadataTypeEnum::from(v.inner))),
			IRType::Unsigned16(v) => Ok(OwnedMetadataTypeEnum::new(&v.owned, BasicMetadataTypeEnum::from(v.inner))),
			IRType::Unsigned32(v) => Ok(OwnedMetadataTypeEnum::new(&v.owned, BasicMetadataTypeEnum::from(v.inner))),
			IRType::Unsigned64(v) => Ok(OwnedMetadataTypeEnum::new(&v.owned, BasicMetadataTypeEnum::from(v.inner))),
			IRType::Unsigned128(v) => Ok(OwnedMetadataTypeEnum::new(&v.owned, BasicMetadataTypeEnum::from(v.inner))),
			IRType::Signed8(v) => Ok(OwnedMetadataTypeEnum::new(&v.owned, BasicMetadataTypeEnum::from(v.inner))),
			IRType::Signed16(v) => Ok(OwnedMetadataTypeEnum::new(&v.owned, BasicMetadataTypeEnum::from(v.inner))),
			IRType::Signed32(v) => Ok(OwnedMetadataTypeEnum::new(&v.owned, BasicMetadataTypeEnum::from(v.inner))),
			IRType::Signed64(v) => Ok(OwnedMetadataTypeEnum::new(&v.owned, BasicMetadataTypeEnum::from(v.inner))),
			IRType::Signed128(v) => Ok(OwnedMetadataTypeEnum::new(&v.owned, BasicMetadataTypeEnum::from(v.inner))),

			IRType::Pointer(v) => Ok(OwnedMetadataTypeEnum::new(&v.owned, BasicMetadataTypeEnum::from(v.inner))),

			IRType::Struct(a) => Ok(OwnedMetadataTypeEnum::new(&a.owned, BasicMetadataTypeEnum::from(a.inkwell_type))),
			IRType::Layout(a) => Ok(OwnedMetadataTypeEnum::new(&a.owned, BasicMetadataTypeEnum::from(a.inkwell_type))),

			_ => Err(PositionlessError::new("Given IR type doesn't have any Inkwell type!!!"))
		}
	}

	pub fn get_inkwell_inttype(&self) -> PositionlessResult<&OwnedIntType> {
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

	pub fn get_inkwell_pointertype(ctx: &IRContext) -> OwnedPointerType {
		return OwnedPointerType::new(&ctx.inkwell_ctx, ctx.ptr_type);
	}

	/// Checks if the given type instance is the same type as the given one without having to use `PartialEq`
	pub fn is_same(&self, t: &Rc<IRType>) -> bool {
		return match(self, t.as_ref()) {
			(IRType::Signed8(_), IRType::Signed8(_)) => true,
			(IRType::Signed16(_), IRType::Signed16(_)) => true,
			(IRType::Signed32(_), IRType::Signed32(_)) => true,
			(IRType::Signed64(_), IRType::Signed64(_)) => true,
			(IRType::Signed128(_), IRType::Signed128(_)) => true,

			(IRType::Unsigned8(_), IRType::Unsigned8(_)) => true,
			(IRType::Unsigned16(_), IRType::Unsigned16(_)) => true,
			(IRType::Unsigned32(_), IRType::Unsigned32(_)) => true,
			(IRType::Unsigned64(_), IRType::Unsigned64(_)) => true,
			(IRType::Unsigned128(_), IRType::Unsigned128(_)) => true,

			(IRType::Bool(_), IRType::Bool(_)) => true,

			(IRType::Struct(a), IRType::Struct(b)) => a.name == b.name && a.is_layout == b.is_layout,
			(IRType::Layout(a), IRType::Layout(b)) => a.name == b.name && a.is_layout == b.is_layout,

			_ => false
		}
	}

	pub fn get_structured_type_descriptor(&self) -> PositionlessResult<Rc<IRStructuredType>> {
		return match self {
			IRType::Struct(e) => Ok(e.clone()),
			IRType::Layout(e) => Ok(e.clone()),
			_ => Err(PositionlessError::new("Given IRType doesn't have a structured type descriptor!"))
		}
	}

	pub fn has_structured_type_descriptor(&self) -> bool {
		return match self {
			IRType::Struct(_) | IRType::Layout(_) => true,
			_ => false
		}
	}

}