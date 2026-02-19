//! Everything related to boolean handling and generation

use commons::err::{PositionlessError, PositionlessResult};
use inkwell::IntPredicate;
use lexer::toks::comp::ComparingOperator;

use crate::{ctx::IRContext, refs::IRValueRef, types::{BOOL_TYPE_HASH, typing::{OwnedIntType, OwnedIntValue, OwnedValueEnum}}, values::IRValue};

pub fn make_bool_xor(ctx: &IRContext, b: IRValueRef) -> PositionlessResult<IRValue> {
	let val = b.obtain(ctx)?;
	let inkwell = match val.obtain_as_bool() {
		Some(v) => v,
		None => return Err(PositionlessError::new("Cannot obtain boolean from the provided boolean value reference!"))
	};

	let bool_t = match ctx.type_storage.get(BOOL_TYPE_HASH) {
		Some(v) => v,
		None => return Err(PositionlessError::new("Cannot find boolean type in type storage!"))
	};

	let one = bool_t.get_inkwell_inttype()?.const_int(1, false);
	
	let xor_v = match ctx.builder.build_xor(inkwell.inner, one, "xor_") {
		Ok(v) => v,
		Err(_) => return Err(PositionlessError::new("build_xor failed!"))
	};

	return Ok(IRValue::new(OwnedValueEnum::new(&ctx.inkwell_ctx, xor_v.into()), bool_t));
}

pub fn make_bool_cmp_int(ctx: &IRContext, a: IRValueRef, b: IRValueRef, comp: ComparingOperator) -> PositionlessResult<IRValue> {
	let a_val = a.obtain(ctx)?;
	let b_val = b.obtain(ctx)?;

	let t = a_val.t.clone();

	let a_int = match a_val.obtain_as_int(ctx, t.clone()) {
		Some(v) => v,
		None => return Err(PositionlessError::new("Value a wasn't an int when trying to use comp!"))
	};

	let b_int = match b_val.obtain_as_int(ctx, t.clone()) {
		Some(v) => v,
		None => return Err(PositionlessError::new("Value b wasn't an int when trying to use comp!"))
	};

	let predicate: IntPredicate;

	if t.is_signed() {
		predicate = match comp {
			ComparingOperator::Equal => IntPredicate::EQ,
			ComparingOperator::NotEqual => IntPredicate::NE,
			ComparingOperator::Higher => IntPredicate::SGT,
			ComparingOperator::HigherEqual => IntPredicate::SGE,
			ComparingOperator::Lower => IntPredicate::SLT,
			ComparingOperator::LowerEqual => IntPredicate::SLE
		};
	} else {
		predicate = match comp {
			ComparingOperator::Equal => IntPredicate::EQ,
			ComparingOperator::NotEqual => IntPredicate::NE,
			ComparingOperator::Higher => IntPredicate::ULT,
			ComparingOperator::HigherEqual => IntPredicate::UGT,
			ComparingOperator::Lower => IntPredicate::ULE,
			ComparingOperator::LowerEqual => IntPredicate::UGE
		}
	}

	let cmp = match ctx.builder.build_int_compare(predicate, *a_int, *b_int, "_cmp") {
		Ok(v) => v,
		Err(_) => return Err(PositionlessError::new("build_int_compare failed!"))
	};

	let bool_t = match ctx.type_storage.get(BOOL_TYPE_HASH) {
		Some(v) => v,
		None => return Err(PositionlessError::new("boolean type wasn't found!"))
	};

	return Ok(IRValue::new(OwnedValueEnum::new(&ctx.inkwell_ctx,cmp.into()), bool_t));
}