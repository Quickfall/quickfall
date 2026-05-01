use crate::{TypeTransmutation, container::Type, raw::RawType};

impl TypeTransmutation for Type {
    fn can_transmute(&self, type_destination: Type) -> bool {
        match (self, &type_destination) {
            (
                Type::Pointer { is_array, inner: _ },
                Type::Pointer {
                    is_array: is_array2,
                    inner: _,
                },
            ) => *is_array == *is_array2,

            (
                Type::Array { size, inner },
                Type::Array {
                    size: size2,
                    inner: inner2,
                },
            ) => size == size2 && inner.can_transmute(*inner2.clone()),

            (Type::GenericTypeParam { .. }, Type::GenericTypeParam { .. }) => false, // Transmutation disabled for type parameters

            (Type::Pointer { is_array, inner: _ }, Type::Raw { raw }) => {
                if !is_array && raw.t == RawType::AnyPointer {
                    return true;
                }

                return false;
            }

            (Type::Raw { raw }, Type::Pointer { is_array, inner: _ }) => {
                if !is_array && raw.t == RawType::AnyPointer {
                    return true;
                }

                return false;
            }

            _ => false,
        }
    }
}

impl TypeTransmutation for RawType {
    fn can_transmute(&self, type_destination: Self) -> bool {
        if self.is_numeric() && type_destination.is_numeric() {
            return true;
        }

        match (self, type_destination) {
            (RawType::StaticString, RawType::AnyPointer) => return true,
            (RawType::EnumChild(container), RawType::Enum(parent_container)) => {
                return container.parent.self_id == parent_container.self_id;
            }

            _ => return false,
        }
    }
}
