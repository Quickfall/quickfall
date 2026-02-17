use commons::err::{PositionlessError, PositionlessResult};
use inkwell::{builder::Builder, types::BasicTypeEnum, values::{BasicValue, BasicValueEnum, IntValue, PointerValue}};

use crate::{refs::IRValueRef, types::typing::IRType, values::IRValue};

pub struct IRPointer<'a> {
	inkwell_ptr: PointerValue<'a>,
	t: &'a IRType<'a>,
	pub name: String
}

impl<'a> IRPointer<'a> {
	pub fn new(ptr: PointerValue<'a>, t: &'a IRType<'a>, name: String) -> Self {
		return IRPointer { inkwell_ptr: ptr, name, t }
	}

	pub fn create(builder: &'a Builder<'a>, name: String, t: &'a IRType<'a>, initial: IRValueRef<'a>) -> PositionlessResult<Self> {
		let ptr = t.make_numeric_stackvar(builder, name.clone(), initial.obtain(builder)?)?;

		return Ok(IRPointer { inkwell_ptr: ptr, t, name: name.clone() });
	}

	pub fn load(&self, builder: &Builder<'a>, t: &'a IRType<'a>) -> PositionlessResult<IRValue<'a>> {
		if self.t != t {
			return Err(PositionlessError::new("Provided IRType isn't the same!"));
		}

		match builder.build_load(self.t.get_inkwell_basetype()?, self.inkwell_ptr, &self.name) {
			Ok(v) => return Ok(IRValue::new(v, t)),
			Err(_) => return Err(PositionlessError::new("build_load failed!"))
		}
	} 

	pub fn load_from_inkwell_type(&self, builder: &'a Builder<'a>, t: BasicTypeEnum<'a>) -> PositionlessResult<IRValue<'a>> {
		if self.t.get_inkwell_basetype()? != t {
			return Err(PositionlessError::new("Provided type isn't the same!"));
		}

		match builder.build_load(t, self.inkwell_ptr, &self.name) {
			Ok(v) => return Ok(IRValue::new(v, self.t)),
			Err(_) => return Err(PositionlessError::new("build_load failed!"))
		}
	}

	pub fn store<V: BasicValue<'a>>(&self, builder: &Builder<'a>, val: V) {
		builder.build_store(self.inkwell_ptr, val);
	}

}