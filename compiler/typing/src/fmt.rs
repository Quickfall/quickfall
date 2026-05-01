use std::fmt::Display;

use crate::{
    container::Type,
    raw::{InformationRawType, RawType},
};

impl RawType {
    pub fn fmt(&self, f: &mut std::fmt::Formatter<'_>, t: &InformationRawType) -> std::fmt::Result {
        match self {
            Self::Integer(signed, size) => {
                if *signed {
                    write!(f, "s{}", size)?;
                } else {
                    write!(f, "u{}", size)?;
                }
            }

            Self::Floating(signed, size) => {
                if *signed {
                    write!(f, "f{}", size)?;
                } else {
                    write!(f, "uf{}", size)?;
                }
            }

            Self::FixedPoint(signed, a, b) => {
                if *signed {
                    write!(f, "x{}.{}", a, b)?;
                } else {
                    write!(f, "ux{}.{}", a, b)?;
                }
            }

            Self::StaticString => {
                write!(f, "staticstr")?;
            }

            Self::AnyPointer => {
                write!(f, "ptr")?;
            }

            Self::Boolean => {
                write!(f, "bool")?;
            }

            Self::Struct(container) => {
                write!(f, "{}", container.name)?;
            }

            Self::Enum(container) => {
                write!(f, "{}", container.name.val)?;
            }

            Self::EnumChild(container) => {
                write!(
                    f,
                    "{}::{}",
                    container.parent.name.val, container.child_index
                )?;
            }

            Self::UnsizedInteger(signed) => {
                if *signed {
                    write!(f, "s{}", t.sizes[0])?;
                } else {
                    write!(f, "u{}", t.sizes[0])?;
                }
            }

            Self::UnsizedFloating(signed) => {
                if *signed {
                    write!(f, "f{}", t.sizes[0])?;
                } else {
                    write!(f, "uf{}", t.sizes[0])?;
                }
            }

            Self::UnsizedFixedPoint(signed) => {
                if *signed {
                    write!(f, "x{}.{}", t.sizes[0], t.sizes[1])?;
                } else {
                    write!(f, "ux{}.{}", t.sizes[0], t.sizes[1])?;
                }
            }
        };

        return Ok(());
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Raw { raw } => raw.t.fmt(f, raw),
            Self::Array { size, inner } => {
                inner.fmt(f)?;

                write!(f, "[{}]", size)?;
                Ok(())
            }

            Self::Pointer { is_array, inner } => {
                inner.fmt(f)?;

                write!(f, "*")?;

                if *is_array {
                    write!(f, "[]")?;
                }

                Ok(())
            }

            Self::GenericTypeParam {
                constraints: _,
                name,
            } => {
                writeln!(f, "{}", name)
            }
        }
    }
}
