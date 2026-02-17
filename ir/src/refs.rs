//! IR value reference definitions

use commons::err::{PositionlessError, PositionlessResult};
use inkwell::{builder::Builder, types::{AnyTypeEnum, BasicTypeEnum}, values::PointerValue};

use crate::{ctx::IRContext, irstruct::{ptr::IRPointer, staticvars::IRStaticVariable}, types::typing::IRType, values::IRValue};

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

	pub fn from_static(val: IRStaticVariable<'a>) -> Self {
		return IRValueRef { kind: IRValueRefKind::Global(val.t, val) }
	}

	pub fn from_pointer(ptr: IRPointer<'a>) -> Self {
		return IRValueRef { kind: IRValueRefKind::Ptr(ptr.t, ptr) }
	}

	/// Determines if aqcuiring the values require a load instruction or any instruction at all to obtain the value from.
	pub fn requires_load(&self) -> bool {
		return matches!(self.kind, IRValueRefKind::Ptr(_, _))
	}

	pub fn obtain(&self, ctx: &'a IRContext<'a>) -> PositionlessResult<IRValue<'a>> {
		match &self.kind {
			IRValueRefKind::Ptr(t, ptr) => {
				ptr.load(ctx, t)
			},

			IRValueRefKind::Val(v) => Ok(IRValue::clone(v)),

			IRValueRefKind::Global(t, global) => {
				Ok(IRValue::new(global.as_val()?, t))
			}
		}
	}

	pub fn obtain_pointer(&self, ctx: &'a IRContext<'a>) -> PositionlessResult<PointerValue<'a>> {
		match &self.kind {
			IRValueRefKind::Ptr(_, ptr) => return Ok(ptr.inkwell_ptr),

			IRValueRefKind::Val(v) => {
				let ptr = IRPointer::create(&ctx, String::from("_val_toptr"), v.t, Some(IRValueRef::from_val(IRValue::clone(v))))?;

				return Ok(ptr.inkwell_ptr);
			}

			IRValueRefKind::Global(_, g) => {
				return Ok(g.as_string_ref()?.as_pointer_value());
			}
		}
	}

}