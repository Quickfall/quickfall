//! IR value representation definitons

use std::collections::HashMap;

use commons::{err::{PositionedResult, PositionlessError, PositionlessResult}, utils::num::{can_num_fit_inbits_signed, can_num_fit_inbits_unsigned}};

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

}