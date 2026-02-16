//! IR value reference definitions

use commons::err::{PositionlessError, PositionlessResult};
use inkwell::{builder::Builder, types::{AnyTypeEnum, BasicTypeEnum}};

use crate::{irstruct::{ptr::IRPointer, staticvars::IRStaticVariable}, types::typing::IRType, values::{IRValue}};

pub enum IRValueRefKind<'a> {
	Ptr(&'a IRType<'a>, IRPointer<'a>),
	Val(IRValue<'a>),
	Global(&'a IRType<'a>, IRStaticVariable<'a>)
}

/// The IR value reference. Basically represents any value whatsoever, can handle every shape of values and is used for uniform handling. 
pub struct IRValueRef<'a> {
	// TODO: maybe change IRValueRef to host the fields itself rather than having to use Options
	kind: IRValueRefKind<'a>,
}

impl<'a> IRValueRef<'a> {
	pub fn from_val(val: IRValue<'a>) -> Self {
		return IRValueRef { kind: IRValueRefKind::Val(val) }
	}

	/// Determines if aqcuiring the values require a load instruction or any instruction at all to obtain the value from.
	pub fn requires_load(&self) -> bool {
		return matches!(self.kind, IRValueRefKind::Ptr(_, _))
	}

	pub fn obtain(&self, builder: &'a Builder<'a>) -> PositionlessResult<IRValue<'a>> {
		match &self.kind {
			IRValueRefKind::Ptr(t, ptr) => {
				ptr.load(builder, t)
			},

			IRValueRefKind::Val(v) => Ok(IRValue::clone(v)),

			IRValueRefKind::Global(t, global) => {
				Ok(IRValue::new(global.as_val()?, t))
			}
		}
	}

}