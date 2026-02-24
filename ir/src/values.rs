//! IR value representation definitons

use std::{rc::Rc};

use errors::{INKWELL_FUNC_FAILED, IR_TYPE_BOOL, IR_TYPE_SIGNED, IR_TYPE_UNSIGNED, errs::{BaseResult, base::BaseError}};
use inkwell::{types::StringRadix};

use crate::{ctx::IRContext, types::typing::{IRType, OwnedIntValue, OwnedValueEnum}};

/// The new IR value system. Allows for a close interaction with inkwell rather than a more AST-side one.
/// # Safety
/// IRValue enforces a strict typing system for values. An instance of `IRType` is required for every gather and will fail if the provided type isn't the variable's.
#[derive(Clone)]
pub struct IRValue {
	inkwell_val: OwnedValueEnum,
	pub t: Rc<IRType>,
}

impl IRValue {
	/// Creates a new untracked instance
	pub fn new(inkwell_val: OwnedValueEnum, t: Rc<IRType>) -> Self {
		return IRValue { inkwell_val, t: t.clone() }
	}

	pub fn from_unsigned(ctx: &IRContext, t: Rc<IRType>, v: u128) -> BaseResult<Self> {
		if !t.is_numeric_type() || t.is_signed() {
			return Err(BaseError::err(IR_TYPE_UNSIGNED!().to_string()));
		}

		let int_type = t.get_inkwell_inttype()?;
		let val = match int_type.const_int_from_string(&v.to_string(), StringRadix::Decimal) {
			Some(v) => v,
			None => return Err(BaseError::critical(format!(INKWELL_FUNC_FAILED!(), "const_int_from_string", "e")))
		};

		return Ok(IRValue::new(OwnedValueEnum::new(&ctx.inkwell_ctx, val.into()), t));
	}

	pub fn from_signed(ctx: &IRContext, t: Rc<IRType>, v: i128) -> BaseResult<Self> {
		if !t.is_numeric_type() || !t.is_signed() {
			return Err(BaseError::err(IR_TYPE_SIGNED!().to_string()));
		}

		let int_type = t.get_inkwell_inttype()?;
		let val = match int_type.const_int_from_string(&v.to_string(), StringRadix::Decimal) {
			Some(v) => v,
			None => return Err(BaseError::critical(format!(INKWELL_FUNC_FAILED!(), "const_int_from_string", "e")))
		};

		return Ok(IRValue::new(OwnedValueEnum::new(&ctx.inkwell_ctx, val.into()), t));
	}

	pub fn from_bool(val: bool, ctx: &IRContext, t: Rc<IRType>) -> BaseResult<Self> {
		let inkwell_type = match t.as_ref() {
			IRType::Bool(ty) => ty,
			_ => return Err(BaseError::err(IR_TYPE_BOOL!().to_string()))
		};

		return Ok(IRValue::new(OwnedValueEnum::new(&ctx.inkwell_ctx, inkwell_type.const_int(val as u64, false).into()),t));
	}

	/// Typeless obtain. Can be considered as an unsafe handle. Doesn't perform type checking
	pub fn obtain(&self) -> OwnedValueEnum {
		return self.inkwell_val.clone();
	}

	/// Obtains the value as an integer value. Returns None if the value is incompatible with integers
	pub fn obtain_as_int(&self, ctx: &IRContext, t: Rc<IRType>) -> Option<OwnedIntValue> {
		println!("is same as: {}", self.t.is_same(&t));

		if !self.t.is_numeric_type() || !self.t.is_same(&t) {
			return None;
		}

		return Some(OwnedIntValue::new(&ctx.inkwell_ctx, self.inkwell_val.into_int_value()));
	}

	/// Obtains the value as an bool value. Returns None if the value is incompatible with booleans
	pub fn obtain_as_bool(&self) -> Option<OwnedIntValue> {
		return match self.t.as_ref() {
			IRType::Bool(v) => Some(OwnedIntValue::new(&v.owned, self.inkwell_val.into_int_value())),
			_ => None
		}
	}

}