use std::rc::Rc;

use commons::err::{PositionlessError, PositionlessResult};
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

	pub fn create(ctx: &IRContext, name: String, t: Rc<IRType>, initial: Option<IRValueRef>) -> PositionlessResult<Self> {
		let ptr = match ctx.builder.build_alloca(t.get_inkwell_basetype()?.inner, &name) {
			Ok(v) => v,
			Err(e) => return Err(PositionlessError::new(&format!("build_alloca failed! {}", e)))
		};

		if initial.is_some() {
			match ctx.builder.build_store(ptr, *initial.unwrap().obtain(ctx)?.obtain()) {
				Err(_) => return Err(PositionlessError::new("build_store failed!")),
				Ok(_) => {} 
			};
		}

		return Ok(IRPointer { owned: ctx.inkwell_ctx.clone(), inkwell_ptr: ptr, t, name: name.clone() });
	}

	pub fn load(&self, ctx: &IRContext, t: Rc<IRType>) -> PositionlessResult<IRValue> {
		if !self.t.is_same(&t) {
			return Err(PositionlessError::new("Provided IRType isn't the same!"));
		}

		match ctx.builder.build_load(*self.t.get_inkwell_instance_basetype(ctx)?, self.inkwell_ptr, &self.name) {
			Ok(v) => return Ok(IRValue::new(OwnedValueEnum::new(&ctx.inkwell_ctx, v), t)),
			Err(_) => return Err(PositionlessError::new("build_load failed!"))
		}
	} 
	
	pub fn store(&self, ctx: &IRContext, val: BasicValueEnum) -> bool {
		return match ctx.builder.build_store(self.inkwell_ptr, val) {
			Ok(_) => true,
			Err(_) => false
		}
	}

}