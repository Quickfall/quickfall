//! The typing system used all across the compiler in all levels of AstoIR and AST.

use crate::{references::TypeReference};

pub mod tree;
pub mod raw;
pub mod structs;
pub mod enums;
pub mod references;

/// A function contained within a type.
pub type TypedFunction = (Vec<TypeReference>, Option<TypeReference>);

pub type RawTypeReference = usize;
