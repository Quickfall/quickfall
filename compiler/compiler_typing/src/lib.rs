//! The typing system used all across the compiler in all levels of AstoIR and AST.

use crate::tree::Type;

pub mod tree;
pub mod raw;
pub mod structs;

/// A function contained within a type.
pub type TypedFunction = (Vec<Type>, Option<Type>);

pub type RawTypeReference = usize;