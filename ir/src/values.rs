//! IR value representation definitons

use commons::{err::{PositionlessError, PositionlessResult}, utils::num::{can_num_fit_inbits_signed, can_num_fit_inbits_unsigned}};

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