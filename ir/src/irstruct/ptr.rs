use commons::err::{PositionlessError, PositionlessResult};
use inkwell::{builder::Builder, context::Context, types::BasicTypeEnum, values::{BasicValue, BasicValueEnum, IntValue, PointerValue}};

use crate::{ctx::IRContext, refs::IRValueRef, types::typing::IRType, values::IRValue};

#[derive(Clone)]
pub struct IRPointer<'a> {
	pub inkwell_ptr: PointerValue<'a>, // Only use this directly within structs
	pub t: &'a IRType<'a>,
	pub name: String
}

impl<'a> IRPointer<'a> {
	pub fn new(ptr: PointerValue<'a>, t: &'a IRType<'a>, name: String) -> Self {
		return IRPointer { inkwell_ptr: ptr, name, t }
	}

	pub fn create(ctx: &'a IRContext<'a>, name: String, t: &'a IRType<'a>, initial: Option<IRValueRef<'a>>) -> PositionlessResult<Self> {
		let ptr = match ctx.builder.build_alloca(t.get_inkwell_basetype()?, &name) {
			Ok(v) => v,
			Err(_) => return Err(PositionlessError::new("build_alloca failed!"))
		};

		if initial.is_some() {
			match ctx.builder.build_store(ptr, initial.unwrap().obtain(ctx)?.obtain()) {
				Err(_) => return Err(PositionlessError::new("build_store failed!")),
				Ok(_) => {} 
			};
		}

		return Ok(IRPointer { inkwell_ptr: ptr, t, name: name.clone() });
	}

	pub fn load(&self, ctx: &'a IRContext<'a>, t: &'a IRType<'a>) -> PositionlessResult<IRValue<'a>> {
		if !self.t.is_same(t) {
			return Err(PositionlessError::new("Provided IRType isn't the same!"));
		}

		match ctx.builder.build_load(self.t.get_inkwell_instance_basetype(ctx)?, self.inkwell_ptr, &self.name) {
			Ok(v) => return Ok(IRValue::new(v, t)),
			Err(_) => return Err(PositionlessError::new("build_load failed!"))
		}
	} 

	pub fn load_from_inkwell_type(&self, ctx: &'a IRContext<'a>, t: BasicTypeEnum<'a>) -> PositionlessResult<IRValue<'a>> {
		if self.t.get_inkwell_instance_basetype(ctx)? != t {
			return Err(PositionlessError::new("Given types aren't the same!"))
		}

		match ctx.builder.build_load(t, self.inkwell_ptr, &self.name) {
			Ok(v) => return Ok(IRValue::new(v, self.t)),
			Err(_) => return Err(PositionlessError::new("build_load failed!"))
		}
	}

	pub fn store<V: BasicValue<'a>>(&self, builder: &Builder<'a>, val: V) -> bool {
		return match builder.build_store(self.inkwell_ptr, val) {
			Ok(_) => true,
			Err(_) => false
		}
	}

}