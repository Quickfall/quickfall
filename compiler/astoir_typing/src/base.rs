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
	pub fn can_transmute_into(&self, into: &BaseType) -> bool {
		return false;
	}

	pub fn can_cast_into(&self, info: &BaseType) -> bool {
		return false;
	}

}