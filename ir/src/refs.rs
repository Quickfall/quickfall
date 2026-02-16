//! IR value reference definitions

use commons::err::{PositionlessError, PositionlessResult};
use inkwell::{builder::Builder, types::{AnyTypeEnum, BasicTypeEnum}};

use crate::{irstruct::{ptr::IRPointer, staticvars::IRStaticVariable}, types::typing::IRType, values::{IRValue}};

pub enum IRValueRefKind<'a> {
	Ptr(IRPointer<'a>),
	Val(IRValue<'a>),
	Global(IRStaticVariable<'a>)
}

/// The IR value reference. Basically represents any value whatsoever, can handle every shape of values and is used for uniform handling. 
pub struct IRValueRef<'a> {
	// TODO: maybe change IRValueRef to host the fields itself rather than having to use Options
	kind: IRValueRefKind<'a>,

	t: &'a IRType<'a>
}

impl<'a> IRValueRef<'a> {
	/// Determines if aqcuiring the values require a load instruction or any instruction at all to obtain the value from.
	pub fn requires_load(&self) -> bool {
		return matches!(self.kind, IRValueRefKind::Ptr(_))
	}

	pub fn obtain(&self, builder: &'a Builder<'a>) -> PositionlessResult<IRValue<'a>> {
		match &self.kind {
			IRValueRefKind::Ptr(ptr) => {
				ptr.load(builder, self.t)
			},

			IRValueRefKind::Val(v) => Ok(IRValue::clone(v)),

			IRValueRefKind::Global(global) => {
				Ok(IRValue::new(global.as_val()?, self.t))
			}
		}
	}

}