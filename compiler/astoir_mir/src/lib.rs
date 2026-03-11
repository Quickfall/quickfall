//! The MIR layer of the AstoIR. 
//! The MIR layer represents a block based representation of the program. Uses low level instructions near Assembly

use astoir_typing::{base::BaseType, complete::{ConcreteType}};
use compiler_errors::errs::{BaseResult, base::BaseError};

use crate::vals::base::{BaseValueType};

pub mod insts;
pub mod vals;
pub mod blocks;
pub mod builder;
pub mod funcs;

pub fn lower_astoir_typing_type(concrete: ConcreteType) -> BaseResult<BaseValueType> {
	match &concrete.base {
		BaseType::NumericIntegerType(a, _) => return Ok(BaseValueType::IntValue(*a as usize)),
		BaseType::FixedPointNumberType(a, b, _) => return Ok(BaseValueType::IntValue(*a as usize + *b as usize)),
		BaseType::FloatingNumberType(a, b, _) => return Ok(BaseValueType::FloatValue(*a as usize + *b as usize)), // TODO: check for float compatibility
		BaseType::Boolean => return Ok(BaseValueType::IntValue(1)),
		BaseType::ArbitraryType(a) => return Ok(BaseValueType::IntValue(*a as usize)),
		BaseType::Pointer => return Ok(BaseValueType::IntValue(concrete.base.get_size()?)),
		BaseType::Struct(_, b) => return Ok(BaseValueType::StructTypeValue(b.clone(), b.ind)),

		_ => return Err(BaseError::err("Cannot lower astoir_typing type to MIR".to_string()))
	}
}