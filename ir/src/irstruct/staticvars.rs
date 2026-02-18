//! Static variable related code

use std::rc::Rc;

use commons::err::{PositionlessError, PositionlessResult};
use inkwell::{builder::Builder, values::{BasicValueEnum, GlobalValue}};

use crate::{ctx::IRContext, types::typing::{IRType, OwnedGlobalValue, OwnedValueEnum}, values::IRValue};

#[derive(Clone)]
pub struct IRStaticVariable {
	inkwell: Option<OwnedGlobalValue>,
	val: Option<OwnedValueEnum>,
	pub t: Rc<IRType>,
	pub name: String
}

impl IRStaticVariable {
	pub fn from_str(ctx: &IRContext, str: &str, name: String, t: Rc<IRType>) -> PositionlessResult<IRStaticVariable> {
		let inkwell = match ctx.builder.build_global_string_ptr(str, &name) {
			Ok(v) => v,
			Err(_) => return Err(PositionlessError::new("build_global_string_ptr failed!!"))
		};

		return Ok(IRStaticVariable { inkwell: Some(OwnedGlobalValue::new(&ctx.inkwell_ctx, inkwell)), t, name, val: None });
	}

	pub fn from_val(name: String, t: Rc<IRType>, val: IRValue) -> PositionlessResult<IRStaticVariable> {
		return Ok(IRStaticVariable { val: Some(val.obtain()), inkwell: None, t, name })
	}

	pub fn is_compiletime_replaceable(&self) -> bool {
		return self.val.is_some();
	}

	pub fn as_val(&self) -> PositionlessResult<OwnedValueEnum> {
		if self.val.is_some() {
			return Ok(self.val.as_ref().unwrap().clone());
		}

		return Ok(OwnedValueEnum::new(&self.inkwell.as_ref().unwrap().owned, self.as_string_ref()?.as_pointer_value().into()));
	}

	pub fn as_string_ref(&self) -> PositionlessResult<OwnedGlobalValue> {
		if self.is_compiletime_replaceable() {
			return Err(PositionlessError::new("Tried using as_string_ref on a compiletime determined global var"));
		}

		return Ok(self.inkwell.clone().unwrap())
	}

}