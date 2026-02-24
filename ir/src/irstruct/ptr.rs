use std::rc::Rc;

use errors::{INKWELL_FUNC_FAILED, IR_DIFF_TYPE, errs::{BaseResult, base::BaseError}};
use inkwell::{builder::Builder, context::Context, types::BasicTypeEnum, values::{BasicValue, BasicValueEnum, IntValue, PointerValue}};

use crate::{ctx::IRContext, refs::IRValueRef, types::typing::{IRType, OwnedTypeEnum, OwnedValueEnum}, values::IRValue};

#[derive(Clone)]
pub struct IRPointer {
	owned: Rc<Context>,
	pub inkwell_ptr: PointerValue<'static>, // Only use this directly within structs
	pub t: Rc<IRType>,
	pub name: String
}

impl IRPointer {
	pub fn new(ptr: PointerValue<'static>, ctx: &IRContext, t: Rc<IRType>, name: String) -> Self {
		return IRPointer { inkwell_ptr: ptr, owned: ctx.inkwell_ctx.clone(), name, t }
	}

	pub fn create(ctx: &IRContext, name: String, t: Rc<IRType>, initial: Option<IRValueRef>) -> BaseResult<Self> {
		let ptr = match ctx.builder.build_alloca(t.get_inkwell_basetype()?.inner, &name) {
			Ok(v) => v,
			Err(e) => return Err(BaseError::critical(format!(INKWELL_FUNC_FAILED!(), "build_alloca", e)))
		};

		if initial.is_some() {
			match ctx.builder.build_store(ptr, *initial.unwrap().obtain(ctx)?.obtain()) {
				Err(e) => return Err(BaseError::critical(format!(INKWELL_FUNC_FAILED!(), "build_store", e))),
				Ok(_) => {} 
			};
		}

		return Ok(IRPointer { owned: ctx.inkwell_ctx.clone(), inkwell_ptr: ptr, t, name: name.clone() });
	}

	pub fn load(&self, ctx: &IRContext, t: Rc<IRType>) -> BaseResult<IRValue> {
		if !self.t.is_same(&t) {
			return Err(BaseError::err(IR_DIFF_TYPE!().to_string()));
		}

		match ctx.builder.build_load(*self.t.get_inkwell_instance_basetype(ctx)?, self.inkwell_ptr, &self.name) {
			Ok(v) => return Ok(IRValue::new(OwnedValueEnum::new(&ctx.inkwell_ctx, v), t)),
			Err(e) => return Err(BaseError::critical(format!(INKWELL_FUNC_FAILED!(), "build_load", e)))
		}
	} 
	
	pub fn store(&self, ctx: &IRContext, val: BasicValueEnum) -> bool {
		return match ctx.builder.build_store(self.inkwell_ptr, val) {
			Ok(_) => true,
			Err(_) => false
		}
	}

}