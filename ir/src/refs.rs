//! IR value reference definitions

use std::rc::Rc;

use errors::{IR_VALUE_REF_TEMP_TYPE, errs::{BaseResult, base::BaseError}};

use crate::{ctx::IRContext, irstruct::{ptr::IRPointer, staticvars::IRStaticVariable}, types::typing::{IRType, OwnedPointerValue}, values::IRValue};

pub enum IRValueRefKind {
	Ptr(Rc<IRType>, IRPointer),
	Val(IRValue),
	Global(Rc<IRType>, Rc<IRStaticVariable>),
	TempString(String)
}

/// The IR value reference. Basically represents any value whatsoever, can handle every shape of values and is used for uniform handling. 
pub struct IRValueRef {
	// TODO: maybe change IRValueRef to host the fields itself rather than having to use Options
	kind: IRValueRefKind,
}

impl IRValueRef {
	pub fn from_tempstr(str: String) -> Self {
		return IRValueRef { kind: IRValueRefKind::TempString(str) }
	}
	
	pub fn from_val(val: IRValue) -> Self {
		return IRValueRef { kind: IRValueRefKind::Val(val) }
	}

	pub fn from_static(val: Rc<IRStaticVariable>) -> Self {
		return IRValueRef { kind: IRValueRefKind::Global(val.t.clone(), val) }
	}

	pub fn from_pointer(ptr: IRPointer) -> Self {
		return IRValueRef { kind: IRValueRefKind::Ptr(ptr.t.clone(), ptr) }
	}

	/// Determines if aqcuiring the values require a load instruction or any instruction at all to obtain the value from.
	pub fn requires_load(&self) -> bool {
		return matches!(self.kind, IRValueRefKind::Ptr(_, _))
	}

	pub fn obtain(&self, ctx: &IRContext) -> BaseResult<IRValue> {
		match &self.kind {
			IRValueRefKind::Ptr(t, ptr) => {
				ptr.load(ctx, t.clone())
			},

			IRValueRefKind::Val(v) => Ok(IRValue::clone(v)),

			IRValueRefKind::Global(t, global) => {
				Ok(IRValue::new(global.as_val()?, t.clone()))
			}

			_ => return Err(BaseError::critical(IR_VALUE_REF_TEMP_TYPE!().to_string()))
		}
	}

	pub fn get_type(&self) -> Rc<IRType> {
		return match &self.kind {
			IRValueRefKind::Val(v) => v.t.clone(),
			IRValueRefKind::Ptr(t, _) => return t.clone(),
			IRValueRefKind::Global(t, _) => return t.clone(),
			_ => panic!("Used get_type on temp string type!!")
		}
	}
	
	pub fn as_pointer(&self) -> BaseResult<IRPointer> {
		match &self.kind {
			IRValueRefKind::Ptr(_, ptr) => return Ok(ptr.clone()),
			_ => return Err(BaseError::critical(IR_VALUE_REF_TEMP_TYPE!().to_string()))
		};
	}

	pub fn obtain_pointer(&self, ctx: &IRContext) -> BaseResult<OwnedPointerValue> {
		match &self.kind {
			IRValueRefKind::Ptr(_, ptr) => return Ok(OwnedPointerValue::new(&ctx.inkwell_ctx, ptr.inkwell_ptr)),

			IRValueRefKind::Val(v) => {
				let ptr = IRPointer::create(&ctx, String::from("_val_toptr"), v.t.clone(), Some(IRValueRef::from_val(IRValue::clone(v))))?;

				return Ok(OwnedPointerValue::new(&ctx.inkwell_ctx, ptr.inkwell_ptr));
			}

			IRValueRefKind::Global(_, g) => {
				if g.is_compiletime_replaceable() {
					return Ok(OwnedPointerValue::new(&ctx.inkwell_ctx, g.as_val()?.into_pointer_value()))
				}

				return Ok(OwnedPointerValue::new(&ctx.inkwell_ctx, g.as_string_ref()?.as_pointer_value()));
			},

			_ => return Err(BaseError::critical(IR_VALUE_REF_TEMP_TYPE!().to_string()))
		}
	}

	pub fn obtain_tempstr(&self) -> BaseResult<String> {
		match &self.kind {
			IRValueRefKind::TempString(e) => Ok(e.clone()),
			_ => return Err(BaseError::critical(IR_VALUE_REF_TEMP_TYPE!().to_string()))
		}
	}

}