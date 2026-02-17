//! IR context related code

use std::ops::Add;

use commons::utils::map::HashedMap;
use inkwell::{AddressSpace, builder::Builder, context::Context, types::PointerType};

use crate::irstruct::{funcs::IRFunction, ptr::IRPointer};

/// The global IR context. 
/// Basically holds anything related to the current IR compilation (eg: functions, types, global vars)
pub struct IRContext<'a> {
	pub inkwell_ctx: &'a Context,
	pub builder: Builder<'a>,
	pub ptr_type: PointerType<'a>,

	pub functions: HashedMap<IRFunction<'a>>,
	pub static_vars: HashedMap<IRPointer<'a>>
}

impl<'a> IRContext<'a> {
	pub fn new(builder: Builder<'a>, ctx: &'a Context) -> Self {
		return IRContext { inkwell_ctx: ctx, builder, ptr_type: ctx.ptr_type(AddressSpace::from(0)), functions: HashedMap::new(0), static_vars: HashedMap::new(0) }
	}
}