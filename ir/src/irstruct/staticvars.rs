//! Static variable related code

use commons::err::{PositionlessError, PositionlessResult};
use inkwell::{builder::Builder, context::Context, types::StringRadix, values::{BasicValueEnum, GlobalValue, IntValue}};

use crate::{types::typing::IRType, values::IRValue};

pub struct IRStaticVariable<'a> {
	inkwell: Option<GlobalValue<'a>>,
	val: Option<BasicValueEnum<'a>>,
	t: &'a IRType<'a>,
	pub name: String
}

impl<'a> IRStaticVariable<'a> {
	pub fn from_str(ctx: &'a Builder<'a>, str: &str, name: String, t: &'a IRType<'a>) -> PositionlessResult<IRStaticVariable<'a>> {
		let inkwell = match ctx.build_global_string_ptr(str, &name) {
			Ok(v) => v,
			Err(_) => return Err(PositionlessError::new("build_global_string_ptr failed!!"))
		};

		return Ok(IRStaticVariable { inkwell: Some(inkwell), t, name, val: None });
	}

	pub fn from_val(name: String, t: &'a IRType<'a>, val: IRValue<'a>) -> PositionlessResult<IRStaticVariable<'a>> {
		return Ok(IRStaticVariable { val: Some(val.obtain()), inkwell: None, t, name })
	}

	pub fn is_compiletime_replaceable(&self) -> bool {
		return self.val.is_some();
	}

	pub fn as_val(&self) -> PositionlessResult<BasicValueEnum<'a>> {
		if !self.is_compiletime_replaceable() {
			return Err(PositionlessError::new("Tried using as_int_val on a non-compiletime determined global val"));
		}

		if !self.t.is_numeric_type() {
			return Err(PositionlessError::new("Tried using as_int_val on a non-integer global value type!"));
		}

		return Ok(self.val.unwrap());
	}

	pub fn as_string_ref(&self) -> PositionlessResult<GlobalValue<'a>> {
		if self.is_compiletime_replaceable() {
			return Err(PositionlessError::new("Tried using as_string_ref on a compiletime determined global var"));
		}

		return Ok(self.inkwell.unwrap())
	}

}