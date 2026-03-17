//! Declarations for type transmutation in Quickfall.

use astoir_typing::base::BaseType;
use compiler_errors::{IR_TRANSMUTATION, errs::{BaseResult, base::BaseError}};

use crate::{builder::{build_downcast_int, build_upcast_int}, ctx::MIRContext, vals::base::BaseMIRValue};

/// Performs transmutation on the given value to try to get the targeted type.
pub fn transmute_value(val: BaseMIRValue, target: BaseType, ctx: &mut MIRContext) -> BaseResult<BaseMIRValue> {
	if val.vtype.base.is_integer() != target.is_integer() || val.vtype.base.is_floating() != target.is_floating() || val.vtype.base.is_signed() != target.is_signed() {
		return Err(BaseError::err(IR_TRANSMUTATION!().to_string()));
	}

	if target.is_integer() {
		let sz = val.vtype.base.get_size()?;
		let newsz = target.get_size()?;

		if sz == newsz {
			return Ok(val);
		}

		if sz > newsz {
			let res = build_downcast_int(ctx, val.as_int()?, newsz)?;

			return Ok(res.into());
		} else {
			let res = build_upcast_int(ctx, val.as_int()?, newsz)?;

			return Ok(res.into());
		}		
	}

	if target.is_floating() {
		let sz = val.vtype.base.get_floating_size()?;
		let newsz = target.get_floating_size()?;

		// TODO: change this since floats cannot have two sizes for now
			
		return Ok(val);
	}

	return Err(BaseError::err(IR_TRANSMUTATION!().to_string()));
}