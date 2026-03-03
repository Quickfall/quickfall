//! The HIR layer of the AstoIR. 
//! The HIR layer represents a near AST where elements are resolved instead of raw name hashes, enforcing that the code works in theory.

pub mod ctx;
pub mod nodes;
pub mod structs;