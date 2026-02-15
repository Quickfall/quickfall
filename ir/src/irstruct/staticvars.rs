//! Static variable related code

use commons::err::{PositionlessError, PositionlessResult};
use inkwell::{builder::Builder, context::Context, values::GlobalValue};

use crate::types::typing::IRType;

pub struct IRStaticVariable<'a> {
	inkwell: Option<GlobalValue<'a>>,
	t: &'a IRType<'a>,
	pub name: String
}

impl<'a> IRStaticVariable<'a> {
	pub fn from_str(ctx: &'a Builder<'a>, str: &str, name: String, t: &'a IRType<'a>) -> PositionlessResult<IRStaticVariable<'a>> {
		let inkwell = match ctx.build_global_string_ptr(str, &name) {
			Ok(v) => v,
			Err(_) => return Err(PositionlessError::new("build_global_string_ptr failed!!"))
		};

		return Ok(IRStaticVariable { inkwell: Some(inkwell), t, name });
	}

}