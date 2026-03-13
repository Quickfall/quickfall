//! Definitions for basic types in AstoIR. These are more types of types than concrete types

use compiler_errors::{IR_INCOMPLETE_TYPE, IR_INVALID_NODE_TYPE, errs::{BaseResult, base::BaseError}};

use crate::structs::StructTypeContainer;

#[derive(Clone, Debug)]
pub enum BaseType {
	/// An integer type.
	/// 0: the size in bits
	/// 1: signed or unsigned
	NumericIntegerType(u64, bool),

	/// A floating number type.
	/// 0: the size of the exponent in bits
	/// 1: the size of the fraction in bits
	/// This should produce a warning if no CPU instructions can be used to directly handle this
	FloatingNumberType(u64, u64, bool),

	/// A fixed point number type.
	/// 0: the size of the number in bits
	/// 1: the size of the fraction
	/// This should produce a warning if no CPU instructions can be used to directly handle this
	FixedPointNumberType(u64, u64, bool), 

	/// A boolean type.
	Boolean,

	/// An arbitrary type. Mostly made for internal use only.
	/// 0: size in bits 
	ArbitraryType(u64),

	/// A pointer address type
	Pointer,

	StaticStr,

	/// A structured type
	/// 0: is the struct a layout
	Struct(bool, StructTypeContainer),

	IncompleteNumericType(bool),
	IncompleteFloatingType(bool),
	IncompleteFixedPointType(bool),
	IncompleteArbitraryType
}

impl BaseType {
	pub fn is_number(&self) -> bool {
		return match self {
			BaseType::NumericIntegerType(_, _) => true,
			BaseType::FixedPointNumberType(_, _, _) => true,
			BaseType::FloatingNumberType(_, _, _) => true,

			_ => false
		}
	}

	pub fn get_struct_container(&self) -> BaseResult<&StructTypeContainer> {
		return match self {
			BaseType::Struct(_, e) => Ok(e),
			_ => Err(BaseError::err(IR_INVALID_NODE_TYPE!().to_string()))
		}
	}

	/// Get the size in bits
	pub fn get_size(&self) -> BaseResult<usize> {
		return match self {
			BaseType::Boolean => Ok(1),
			BaseType::ArbitraryType(n) => Ok(*n as usize),
			BaseType::FixedPointNumberType(a, b, _) => Ok((*a + b) as usize),
			BaseType::FloatingNumberType(a, b, _) => Ok((*a + b) as usize),
			BaseType::NumericIntegerType(a, _) => Ok(*a as usize),
			BaseType::Pointer | BaseType::StaticStr => Ok(size_of::<usize>()),
			BaseType::Struct(_, k) => {
				let mut sz = 0;

				for t in &k.fields.vals {
					sz += t.get_concrete().base.get_size()?; // TODO: add support for type arg size changing
				}

				return Ok(sz);
			},

			_ => return Err(BaseError::err(IR_INCOMPLETE_TYPE!().to_string()))
		}
	}

	pub fn get_floating_size(&self) -> BaseResult<(usize, usize)> {
		return match self {
			BaseType::FloatingNumberType(a, b, _) => Ok((*a as usize, *b as usize)),

			_ => Err(BaseError::err("Invalid get_floating_size!".to_string()))
		}
	}

	pub fn is_floating(&self) -> bool {
		return match self {
			BaseType::FixedPointNumberType(_, _,_ ) => true,
			BaseType::FloatingNumberType(_, _, _) => true,
			BaseType::IncompleteFloatingType(_) => true,

			_ => false
		}
	}

	pub fn is_integer(&self) -> bool {
		return match self {
			BaseType::NumericIntegerType(_, _) => true,
			BaseType::IncompleteNumericType(_) => true,

			_ => false
		}
	}

	pub fn is_incomplete(&self) -> bool {
		return match self {
			BaseType::IncompleteArbitraryType => true,
			BaseType::IncompleteNumericType(_) => true,
			BaseType::IncompleteFloatingType(_) => true,
			BaseType::IncompleteFixedPointType(_) => true,

			_ => false
		}
	}

	pub fn is_signed(&self) -> bool {
		return match self {
			BaseType::NumericIntegerType(_, signed) => *signed,
			BaseType::FloatingNumberType(_, _, signed) => *signed,
			BaseType::FixedPointNumberType(_, _, signed) => *signed,
			BaseType::IncompleteNumericType(signed) => *signed,
			BaseType::IncompleteFloatingType(signed) => *signed,
			BaseType::IncompleteFixedPointType(signed) => *signed,

			_ => false
  		}
	}

	pub fn can_transmute_into(&self, into: &BaseType) -> bool {
		if self.is_number() {
			if self.is_floating() != into.is_floating() && !into.is_floating() {
				// Disallow float -> int transmutations
				return false;
			}

			// TODO: add safe risky transmutation
			//if self.get_size() > into.get_size() {
			//	return false; 
			//}
		}

		return true;
	}

	pub fn can_cast_into(&self, _info: &BaseType) -> bool {
		return false;
	}

	pub fn is_equal(&self, t: &BaseType) -> bool {
		return match (self, t) {
			(BaseType::NumericIntegerType(size, signed), BaseType::NumericIntegerType(a, b)) => *size == *a && *signed == *b,
			(BaseType::FloatingNumberType(exponent, fraction, signed), BaseType::FloatingNumberType(a, b, c)) => *exponent == *a && *fraction == *b && *signed == *c,
			(BaseType::FixedPointNumberType(number, fraction, signed), BaseType::FixedPointNumberType(a, b, c)) => *number == *a && *fraction == *b && *signed == *c,
			(BaseType::Boolean, BaseType::Boolean) => true,
			(BaseType::ArbitraryType(size), BaseType::ArbitraryType(a)) => *size == *a,
			(BaseType::Pointer, BaseType::Pointer) => true,
			(BaseType::StaticStr, BaseType::StaticStr) => true,
			(BaseType::Struct(layout, container), BaseType::Struct(a, b)) => *layout == *a && container.ind == b.ind,
			
			_ => false
		}
	}

}