//! The MIR layer of the AstoIR.
//! The MIR layer represents a block based representation of the program. Uses low level instructions near Assembly
//!

use crate::ctx::MIRContext;
pub mod blocks;
pub mod builder;
pub mod ctx;
pub mod fmt;
pub mod funcs;
pub mod inst_writer;
pub mod insts;
pub mod vals;

pub trait DisplayAstoIR {
    fn format(&self, f: &mut std::fmt::Formatter<'_>, ctx: &MIRContext) -> std::fmt::Result;
}
