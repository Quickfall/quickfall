//! The raw type declarations

use std::{fmt::Display, hash::Hash};

use crate::{SizedType, bounds::traits::Trait, enums::{RawEnumEntryContainer, RawEnumTypeContainer}, storage::TypeStorage, structs::{LoweredStructTypeContainer, RawStructTypeContainer}, tree::Type, utils::get_pointer_size};

/// The raw types. Are also named generics
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RawType {
	Integer(usize, bool),
	Floating(usize, bool),
	FixedPoint(usize, usize, bool),

	Boolean,

	Pointer,

	StaticString,

	Struct(bool, RawStructTypeContainer),
	
	Enum(RawEnumTypeContainer),
	EnumEntry(RawEnumEntryContainer),

	LoweredStruct(bool, LoweredStructTypeContainer),

	SizedInteger(bool),
	SizedFloating(bool),
	SizedFixedPoint(bool)
}

impl RawType {
	/// Make a hint type.
	pub fn make_hint(hint_amount: usize) -> RawType {
		let bits = (hint_amount as f64).log2().ceil() as usize;

		return RawType::Integer(bits, false)
	}

	pub fn get_type_params_count(&self, storage: &TypeStorage) -> usize {
		match self {
			RawType::Enum(container) => container.type_params.len(),
			RawType::EnumEntry(container) => storage.types.vals[container.parent].get_type_params_count(storage),
			RawType::Struct(_, container) => container.type_params.len(),

			_ => 0
		}
	}

	pub fn is_enum_parent(&self) -> bool {
		match self {
			Self::Enum(_) => true,
			Self::LoweredStruct(_, b) => b.is_lowered_enum_parent,
			_ => false
		}
	}
	
	pub fn is_enum_child(&self) -> bool {
		match self {
			Self::EnumEntry(_) => true,
			Self::LoweredStruct(_, b) => b.is_lowered_enum_child,
			_ => false
		}
	}

	pub fn is_field_based(&self) -> bool {
		match self {
			RawType::Struct(_, _) => true,
			RawType::EnumEntry(_) => true,

			_ => false
		}
	}

	pub fn is_signed(&self) -> bool {
		match self {
			Self::Integer(_, signed) => *signed,
			Self::Floating(_, signed) => *signed,
			Self::FixedPoint(_, _, signed) => *signed,

			_ => false
		}
	}

	pub fn is_sized(&self) -> bool {
		match self {
			Self::SizedInteger(_) => true,
			Self::SizedFloating(_) => true,
			Self::SizedFixedPoint(_) => true,

			_ => false
		}
	}
	
	pub fn is_integer(&self) -> bool {
		match self {
			Self::SizedInteger(_) => true,
			Self::Integer(_, _) => true,

			_ => false
		}
	}

	pub fn is_floating_point(&self) -> bool {
		match self {
			Self::SizedFloating(_) => true,
			Self::Floating(_, _) => true,

			_ => false
		}
	}

	pub fn is_fixed_point(&self) -> bool {
		match self {
			Self::SizedFixedPoint(_) => true,
			Self::FixedPoint(_, _, _) => true,
			_ => false
		}
	}

	pub fn is_static(&self) -> bool {
		match self {
			Self::StaticString => true,
			_ => false
		}
	}

	pub fn has_trait(&self, t: Trait, raw_type: &Type) -> bool {
		match t {
			Trait::Integer => self.is_integer(),
			Trait::Floating => self.is_floating_point(),
			Trait::Fixed => self.is_fixed_point(),
			Trait::Signed => self.is_signed(),
			Trait::String => self == &RawType::StaticString,
			Trait::Static => self.is_static(),
			Trait::NonInteger => self.is_floating_point() || self.is_fixed_point(),
			Trait::Numeric => self.is_integer() || self.is_floating_point() || self.is_fixed_point(),
			Trait::CpuSupported => {
				match self {
					Self::Floating(size, _) => {
						let log = size.ilog2();

						return (log >= 4 && log <= 7) || *size == 80;
					},

					Self::SizedFloating(_) => {
						let size = raw_type.get_generic_info().1[0];
						let log = size.ilog2();

						return (log >= 4 && log <= 7) || size == 80;
					}

					_ => return true
				}
			}
		}
	}

	pub fn can_transmute(&self, _self_size: Vec<usize>, b: &RawType, _b_sizes: Vec<usize>) -> bool {
		match (self, b) {
			(Self::Integer(_, _), Self::Integer(_, _)) => true,
			(Self::SizedInteger(_), Self::Integer(_, _)) => true,
			(Self::Integer(_, _), Self::SizedInteger(_)) => true,

			(Self::Floating(_, _), Self::Integer(_, _)) => true,
			(Self::Integer(_, _), Self::Floating(_, _)) => true,

			(Self::Floating(_, _), Self::SizedInteger(_)) => true,
			(Self::SizedInteger(_), Self::Floating(_, _)) => true,

			(Self::StaticString, Self::Pointer) => true,

			(Self::EnumEntry(container), Self::Enum(c2)) => {
				return container.parent == c2.self_ref;
			}

			_ => false
		}
	}
}

impl SizedType for RawType {
	fn get_size(&self, t: &Type, compacted_size: bool, storage: &TypeStorage) -> usize {
		match self {
			RawType::Integer(size, _) => *size,
			RawType::Floating(size, _) => *size,
			RawType::FixedPoint(size_a, size_b, _) => *size_a + size_b,
			RawType::Boolean => {
				if compacted_size {
					return 1;
				}

				return 8;
			},

			RawType::Pointer => return get_pointer_size(),

			RawType::StaticString => return 0, // TODO: make sure  we don't need this

			RawType::Struct(_, container) => return container.get_size(t, compacted_size, storage),
			RawType::Enum(container) => return container.get_size(t, compacted_size, storage),
			RawType::EnumEntry(container) => return container.get_size(t, compacted_size, storage),
			RawType::LoweredStruct(_, container) => return container.get_size(t, compacted_size, storage),

			_ => return 0
		}
	}
}

impl Display for RawType {
	// TODO: add display names for structs and enums
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let s = match self {
			Self::Integer(size, signed) => {
				if *signed {
					format!("s{}", size)
				} else {
					format!("u{}", size)
				}
			}, 

			Self::Floating(size, signed) => {
				if *signed {
					format!("f{}", size)
				} else {
					format!("uf{}", size)
				}
			},

			Self::FixedPoint(a, b, signed) => {
				if *signed {
					format!("x{}", a + b)
				} else {
					format!("ux{}", a + b)
				}
			},

			Self::Boolean => "bool".to_string(),
			Self::Pointer => "ptr".to_string(),
			Self::StaticString => "staticstr".to_string(),

			Self::Struct(_, _) => "__struct__".to_string(),
			Self::Enum(_) => "__enum__".to_string(),
			Self::EnumEntry(_) => "__enum__child".to_string(),
			Self::LoweredStruct(_, _) => "__low__struct__".to_string(),

			Self::SizedInteger(signed) => {
				if *signed {
					"s?".to_string()
				} else {
					"u?".to_string()
				}
			},

			Self::SizedFloating(signed) => {
				if *signed {
					"f?".to_string()
				} else {
					"uf?".to_string()
				}
			},

			Self::SizedFixedPoint(signed) => {
				if *signed {
					"x?".to_string()
				} else {
					"ux?".to_string()
				}
			}
 		};

		write!(f, "{}", s)
	}
}

impl Hash for RawType {
	fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
		match self {
			Self::Integer(a, b) => {
				hasher.write_usize(0);
				hasher.write_usize(*a);
				hasher.write_u8(*b as u8);
			},

			Self::Floating(a, b) => {
				hasher.write_usize(1);
				hasher.write_usize(*a);
				hasher.write_u8(*b as u8);
			},

			Self::FixedPoint(a, b, c) => {
				hasher.write_usize(2);
				hasher.write_usize(*a);
				hasher.write_usize(*b);
				hasher.write_u8(*c as u8);
			},

			Self::Boolean => hasher.write_usize(3),
			Self::Pointer => hasher.write_usize(4),
			Self::StaticString => hasher.write_usize(5),

			Self::LoweredStruct(a, b) => {
				hasher.write_usize(6);
				hasher.write_u8(*a as u8);

				for field in &b.fields.vals {
					field.hash(hasher);
				}

				for function in &b.functions.vals {
					hasher.write_usize(*function);
				}
			},

			RawType::Enum(container) => {
				hasher.write_usize(7);
				hasher.write_usize(container.self_ref);
			}
			
			_ => panic!("Unhashable type {:#?}", self)
		}
	}
}