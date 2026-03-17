use crate::base::BaseType;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConcreteType {
	pub base: BaseType,
	
	pub pointer: bool,
	pub pointer_array: bool,

	pub type_params: Vec<usize>,
	pub size_params: Vec<usize>
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ComplexType {
	Array(Box<ComplexType>),
	Concrete(ConcreteType)
}

impl ComplexType {
	pub fn is_array(&self) -> bool {
		return match self {
			Self::Array(_) => true,
			_ => false
		}
	}

	pub fn get_concrete(&self) -> &ConcreteType {
		return match self {
			Self::Array(e) => e.get_concrete(),
			Self::Concrete(e) => e
		};
	}

	/// Can the given type be automatically casted (aka trasmuted) into the given type.
	/// Transmutation is basically automatic casting 
	pub fn can_transmute_into(&self, info: &ComplexType) -> bool {
		return match self {
			Self::Array(_) => false,
			
			Self::Concrete(t) => {
				let other = info.get_concrete();

				t.base.can_transmute_into(&other.base)
			}
		}
	}

	pub fn can_cast_into(&self, into: &ComplexType) -> bool {
		if self.can_transmute_into(into) {
			return true;
		}

		match self {
			Self::Array(t) => {
				return t.can_transmute_into(into) && into.is_array();
			},

			Self::Concrete(t) => {
				let other = into.get_concrete();

				if t.type_params != other.type_params {
					return false;
				}

				if t.pointer && other.pointer {
					return true;
				}

				return t.base.can_cast_into(&other.base);
			}
		}
	}

}
