//! IR value representation definitons

use commons::{err::{PositionlessError, PositionlessResult}, utils::num::{can_num_fit_inbits_signed, can_num_fit_inbits_unsigned}};
use inkwell::{types::StringRadix, values::{BasicValueEnum, IntValue}};

use crate::types::typing::IRType;

#[deprecated(note="IRValue is fairly unused and acts as a weird mix of IR and AST value handling. Please use IRNewValue", )]
#[derive(Debug)]
pub enum IRValue {
	Signed8(i8),
	Signed16(i16),
	Signed32(i32),
	Signed64(i64),
	Signed128(i128),

	Unsigned8(u8),
	Unsigned16(u16),
	Unsigned32(u32),
	Unsigned64(u64),
	Unsigned128(u128),

	Bool(bool),

	Struct(Vec<Box<IRValue>>, u64), // type hash
	Layout(Vec<Box<IRValue>>, u64) // type hash
}

#[deprecated(note="IRValue is fairly unused and acts as a weird mix of IR and AST value handling. Please use IRNewValue", )]
impl IRValue {
	pub fn make_signed(sz: usize, val: i128) -> PositionlessResult<IRValue> {
		if !can_num_fit_inbits_signed(sz, val) {
			return Err(PositionlessError::new(&format!("Cannot fit the given number value {} into {} bits!", val, sz)));
		}

		match sz {
			8 => return Ok(IRValue::Signed8(val as i8)),
			16 => return Ok(IRValue::Signed16(val as i16)),
			32 => return Ok(IRValue::Signed32(val as i32)),
			64 => return Ok(IRValue::Signed64(val as i64)),
			128 => return Ok(IRValue::Signed128(val as i128)),
			_ => return Err(PositionlessError::new(&format!("Invalid bit size! got {}", sz)))
		}
	}

	pub fn make_unsigned(sz: usize, val: i128) -> PositionlessResult<IRValue> {
		if !can_num_fit_inbits_unsigned(sz, val) {
			return Err(PositionlessError::new(&format!("Cannot fit the given number value {} into {} bits!", val, sz)));
		}

		match sz {
			8 => return Ok(IRValue::Unsigned8(val as u8)),
			16 => return Ok(IRValue::Unsigned16(val as u16)),
			32 => return Ok(IRValue::Unsigned32(val as u32)),
			64 => return Ok(IRValue::Unsigned64(val as u64)),
			128 => return Ok(IRValue::Unsigned128(val as u128)),
			_ => return Err(PositionlessError::new(&format!("Invalid bit size! got {}", sz)))
		}
	}

	pub fn make_bool(val: bool) -> IRValue {
		return IRValue::Bool(val);
	}

	pub fn expects_numeric_value(&self, sz: usize, signed: bool) -> PositionlessResult<i128> {
		let val: i128 = match self {
			IRValue::Unsigned8(v) => *v as i128,
			IRValue::Unsigned16(v) => *v as i128,
			IRValue::Unsigned32(v) => *v as i128,
			IRValue::Unsigned64(v) => *v as i128,
			IRValue::Unsigned128(v) => *v as i128,
			
			IRValue::Signed8(v) => *v as i128,
			IRValue::Signed16(v) => *v as i128,
			IRValue::Signed32(v) => *v as i128,
			IRValue::Signed64(v) => *v as i128,
			IRValue::Signed128(v) => *v as i128,

			_ => return Err(PositionlessError::new(&format!("Expected a numeric value! Got {:#?}", self)))
		};

		if signed {
			if !can_num_fit_inbits_signed(sz, val) {
				return Err(PositionlessError::new(&format!("Numerical value {} cannot fit in bits!", val)));
			}

			return Ok(val);
		}

		if !can_num_fit_inbits_unsigned(sz, val) {
			return Err(PositionlessError::new(&format!("Numerical value {} cannot fit in bits!", val)));
		}

		return Ok(val);
	}

	pub fn expects_bool_value(&self) -> PositionlessResult<bool> {
		match self {
			IRValue::Bool(v) => return Ok(*v),
			_ => return Err(PositionlessError::new(&format!("Expected a boolean but instead got {:#?}", self)))
		}
	} 
}

/// The new IR value system. Allows for a close interaction with inkwell rather than a more AST-side one.
/// # Safety
/// IRNewValue enforces a strict typing system for values. An instance of `IRType` is required for every gather and will fail if the provided type isn't the variable's.
pub struct IRNewValue<'a> {
	inkwell_val: BasicValueEnum<'a>,
	t: &'a IRType<'a>, 
}

impl<'a> IRNewValue<'a> {
	/// Creates a new untracked instance
	pub fn new(inkwell_val: BasicValueEnum<'a>, t: &'a IRType<'a>) -> Self {
		return IRNewValue { inkwell_val, t }
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

		return Ok(IRNewValue::new(val.into(), t))
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

		return Ok(IRNewValue::new(val.into(), t))
	}

	pub fn from_bool(val: bool, t: &'a IRType<'a>) -> PositionlessResult<Self> {
		let inkwell_type = match t {
			IRType::Bool(ty) => ty,
			_ => return Err(PositionlessError::new("from_bool got fed a non-boolean IRType instance! t != IRType::Bool!"))
		};

		return Ok(IRNewValue::new(inkwell_type.const_int(val as u64, false).into(), t))
	}

	/// Typeless obtain. Can be considered as an unsafe handle. Doesn't perform type checking
	pub fn obtain(&self) -> BasicValueEnum<'a> {
		return self.inkwell_val;
	}

	/// Obtains the value as an integer value. Returns None if the value is incompatible with integers
	pub fn obtain_as_int(&self) -> Option<IntValue<'a>> {
		if !self.t.is_numeric_type() {
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