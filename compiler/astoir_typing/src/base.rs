//! Definitions for basic types in AstoIR. These are more types of types than concrete types

#[derive(Clone)]
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

	/// A structured type
	/// 0: is the struct a layout
	Struct(bool)
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

	/// Get the size in bits
	pub fn get_size(&self) -> usize {
		return match self {
			BaseType::Boolean => 1,
			BaseType::ArbitraryType(n) => *n as usize,
			BaseType::FixedPointNumberType(a, b, _) => (*a + b) as usize,
			BaseType::FloatingNumberType(a, b, _) => (*a + b) as usize,
			BaseType::NumericIntegerType(a, t) => *a as usize,

			BaseType::Struct(_) => 0
		}
	}

	pub fn is_floating(&self) -> bool {
		return match self {
			BaseType::FixedPointNumberType(_, _,_ ) => true,
			BaseType::FloatingNumberType(_, _, _) => true,

			_ => false
		}
	}

	pub fn is_integer(&self) -> bool {
		return match self {
			BaseType::NumericIntegerType(_, _) => true,

			_ => false
		}
	}

	pub fn can_transmute_into(&self, into: &BaseType) -> bool {
		if self.is_number() {
			if self.is_floating() != into.is_floating() && !into.is_floating() {
				// Disallow float -> int transmutations
				return false;
			}

			if self.get_size() > into.get_size() {
				return false; 
			}
		}

		return true;
	}

	pub fn can_cast_into(&self, info: &BaseType) -> bool {
		return false;
	}

}