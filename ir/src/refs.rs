//! IR value reference definitions

use crate::{irstruct::{ptr::IRPointer, staticvars::IRStaticVariable}, types::typing::IRType, values::IRValue};

/// The IR value reference. Basically represents any value whatsoever, can handle every shape of values and is used for uniform handling.
pub struct IRValueRef<'a> {
	// TODO: maybe change IRValueRef to host the fields itself rather than having to use Options
	ptr: Option<IRPointer<'a>>,
	val: Option<IRValue>,
	global: Option<IRStaticVariable<'a>>,

	t: &'a IRType<'a>
}

impl<'a> IRValueRef<'a> {
	/// Determines if aqcuiring the values require a load instruction or any instruction at all to obtain the value from.
	pub fn requires_load(&self) -> bool {
		return self.ptr.is_some();
	}

}