use commons::err::{PositionlessError, PositionlessResult};
use inkwell::{builder::Builder, values::{BasicValue, IntValue, PointerValue}};

use crate::{types::typing::IRType, values::IRValue};

pub struct IRPointer<'a> {
	inkwell_ptr: PointerValue<'a>,
	t: &'a IRType<'a>,
	pub name: String
}

impl<'a> IRPointer<'a> {
	pub fn new(ptr: PointerValue<'a>, t: &'a IRType<'a>, name: String) -> Self {
		return IRPointer { inkwell_ptr: ptr, name, t }
	}

	pub fn create(builder: &Builder<'a>, name: String, t: &'a IRType<'a>, initial: IRValue) -> PositionlessResult<Self> {
		let ptr = t.make_numeric_stackvar(builder, name.clone(), initial)?;

		return Ok(IRPointer { inkwell_ptr: ptr, t, name: name });
	}

	pub fn load_val_int(&self, builder: &Builder<'a>) -> PositionlessResult<IntValue<'a>> {
		if !self.t.is_numeric_type() {
			return Err(PositionlessError::new("Requires a numeric type!"));
		}

		match builder.build_load(*self.t.get_inkwell_inttype()?, self.inkwell_ptr, &self.name) {
			Ok(v) => return Ok(v.into_int_value()),
			Err(_) => return Err(PositionlessError::new("build_load failed!"))
		}
	} 

	pub fn store<V: BasicValue<'a>>(&self, builder: &Builder<'a>, val: V) {
		builder.build_store(self.inkwell_ptr, val);
	}

}