//! IR context related code

use commons::utils::map::HashedMap;
use inkwell::context::Context;

use crate::irstruct::{funcs::IRFunction, ptr::IRPointer};

/// The global IR context. 
/// Basically holds anything related to the current IR compilation (eg: functions, types, global vars)
pub struct IRContext<'a> {
	pub inkwell_ctx: &'a Context,
	pub functions: HashedMap<IRFunction<'a>>,
	pub static_vars: HashedMap<IRPointer<'a>>

}