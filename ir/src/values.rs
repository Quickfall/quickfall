//! IR value representation definitons

use commons::{err::{PositionlessError, PositionlessResult}};
use inkwell::{types::StringRadix, values::{BasicValueEnum, IntValue}};

use crate::{irstruct::ptr::IRPointer, types::typing::IRType};

/// The new IR value system. Allows for a close interaction with inkwell rather than a more AST-side one.
/// # Safety
/// IRValue enforces a strict typing system for values. An instance of `IRType` is required for every gather and will fail if the provided type isn't the variable's.
#[derive(Clone)]
pub struct IRValue<'a> {
	inkwell_val: BasicValueEnum<'a>,
	pub t: &'a IRType<'a>, 
}

impl<'a> IRValue<'a> {
	/// Creates a new untracked instance
	pub fn new(inkwell_val: BasicValueEnum<'a>, t: &'a IRType<'a>) -> Self {
		return IRValue { inkwell_val, t }
	}

	pub fn from_unsigned(t: &'a IRType<'a>, v: u128) -> PositionlessResult<Self> {
		if !t.is_numeric_type() || t.is_signed() {
			return Err(PositionlessError::new("The given type cannot be applied to make an unsigned!"));
		}

		let int_type = t.get_inkwell_inttype()?;
		let val = match int_type.const_int_from_string(&v.to_string(), StringRadix::Decimal) {
			Some(v) => v,
			None => return Err(PositionlessError::new("const_int_from_string failed!"))
		};

		return Ok(IRValue::new(val.into(), t))
	}

	pub fn from_signed(t: &'a IRType<'a>, v: i128) -> PositionlessResult<Self> {
		if !t.is_numeric_type() || !t.is_signed() {
			return Err(PositionlessError::new("The given type cannot be applied to make a signed!"));
		}

		let int_type = t.get_inkwell_inttype()?;
		let val = match int_type.const_int_from_string(&v.to_string(), StringRadix::Decimal) {
			Some(v) => v,
			None => return Err(PositionlessError::new("const_int_from_string failed!"))
		};

		return Ok(IRValue::new(val.into(), t))
	}

	pub fn from_bool(val: bool, t: &'a IRType<'a>) -> PositionlessResult<Self> {
		let inkwell_type = match t {
			IRType::Bool(ty) => ty,
			_ => return Err(PositionlessError::new("from_bool got fed a non-boolean IRType instance! t != IRType::Bool!"))
		};

		return Ok(IRValue::new(inkwell_type.const_int(val as u64, false).into(), t))
	}

	/// Typeless obtain. Can be considered as an unsafe handle. Doesn't perform type checking
	pub fn obtain(&self) -> BasicValueEnum<'a> {
		return self.inkwell_val;
	}

	/// Obtains the value as an integer value. Returns None if the value is incompatible with integers
	pub fn obtain_as_int(&self, t: &'a IRType<'a>) -> Option<IntValue<'a>> {
		if !self.t.is_numeric_type() || !self.t.is_same(t) {
			return None;
		}

		return Some(self.inkwell_val.into_int_value());
	}

	/// Obtains the value as an bool value. Returns None if the value is incompatible with booleans
	pub fn obtain_as_bool(&self) -> Option<IntValue<'a>> {
		return match self.t {
			IRType::Bool(_) => Some(self.inkwell_val.into_int_value()),
			_ => None
		}
	}

}