//! Utility functions to build instructions and more

use astoir_typing::compacted::CompactedType;
use compiler_errors::{IR_FUNCTION_INVALID_ARGUMENTS, errs::{BaseResult, base::BaseError}};

use crate::{blocks::{MIRBlock, hints::MIRValueHint, refer::MIRBlockReference}, ctx::MIRContext, funcs::MIRFunction, insts::MIRInstruction, vals::{base::BaseMIRValue, float::MIRFloatValue, int::MIRIntValue, ptr::MIRPointerValue}};

pub fn build_stack_alloc(ctx: &mut MIRContext, block: &mut MIRBlock, size: usize, t: CompactedType) -> BaseResult<MIRPointerValue> {
	let res = block.append(ctx, MIRInstruction::StackAlloc { alloc_size: size, t: t.clone() }).get()?;

	let hint = ctx.ssa_hints.append_hint(MIRValueHint::Pointer(t));

	if res.get_ssa_index() != hint {
		return Err(BaseError::err("Couldn't hint SSA value for pointer! Indexes are different".to_string()))
	}

	return res.as_ptr()
}

pub fn build_load(ctx: &mut MIRContext, block: &mut MIRBlock, ptr: MIRPointerValue) -> BaseResult<BaseMIRValue> {
	let res = block.append(ctx, MIRInstruction::Load { value: ptr }).get()?;

	return Ok(res);
}

pub fn build_store(ctx: &mut MIRContext, block: &mut MIRBlock, ptr: MIRPointerValue, val: BaseMIRValue) -> BaseResult<bool> {
	let base: BaseMIRValue = ptr.clone().into();

	let hint = ctx.ssa_hints.get_hint(base.get_ssa_index())?.as_pointer()?;

	if !hint.base.is_equal(&val.vtype.base) {
		return Err(BaseError::err("Cannot put this value onto this pointer as it is not the same type!".to_string()))
	}

	block.append(ctx, MIRInstruction::Store { variable: ptr, value: val });

	return Ok(true) 
}

pub fn build_downcast_int(ctx: &mut MIRContext, block: &mut MIRBlock, val: MIRIntValue, size: usize) -> BaseResult<MIRIntValue> {
	if val.size <= size {
		return Err(BaseError::critical("Tried using downintcast on a smaller sized int!".to_string()));
	}

	let res = block.append(ctx, MIRInstruction::DowncastInteger { val, size }).get()?;

	return res.as_int();
}

pub fn build_upcast_int(ctx: &mut MIRContext, block: &mut MIRBlock, val: MIRIntValue, size: usize) -> BaseResult<MIRIntValue> {
	if val.size >= size {
		return Err(BaseError::critical("Tried using upintcast on a higher sized int!".to_string()));
	}

	let res = block.append(ctx, MIRInstruction::UpcastInteger { val, size }).get()?;

	return res.as_int();
}

pub fn build_downcast_float(ctx: &mut MIRContext, block: &mut MIRBlock, val: MIRFloatValue, exponent: usize, fraction: usize) -> BaseResult<MIRFloatValue> {
	if val.exponent + val.fraction <= exponent + fraction {
		return Err(BaseError::critical("Tried using downfloatcast on a smaller sized int!".to_string()));
	}

	let res = block.append(ctx, MIRInstruction::DowncastFloat { val, exponent, fraction }).get()?;

	return res.as_float();
}

pub fn build_upcast_float(ctx: &mut MIRContext, block: &mut MIRBlock, val: MIRFloatValue, exponent: usize, fraction: usize) -> BaseResult<MIRFloatValue> {
	if val.exponent + val.fraction >= exponent + fraction {
		return Err(BaseError::critical("Tried using upfloatcast on a higher sized int!".to_string()));
	}

	let res = block.append(ctx, MIRInstruction::UpcastFloat { val, exponent, fraction }).get()?;

	return res.as_float();
}

pub fn build_int_add(ctx: &mut MIRContext, block: &mut MIRBlock, left: MIRIntValue, right: MIRIntValue, signed: bool) -> BaseResult<MIRIntValue> {
	if left.size != right.size {
		return Err(BaseError::critical("Tried using iadd on different sized integers".to_string()));
	}

	let res = block.append(ctx, MIRInstruction::IntegerAdd { signed, left, right }).get()?;

	return res.as_int();
}

pub fn build_int_sub(ctx: &mut MIRContext, block: &mut MIRBlock, left: MIRIntValue, right: MIRIntValue, signed: bool) -> BaseResult<MIRIntValue> {
	if left.size != right.size {
		return Err(BaseError::critical("Tried using isub on different sized integers".to_string()));
	}

	let res = block.append(ctx, MIRInstruction::IntegerSub { signed, left, right }).get()?;

	return res.as_int();
}

pub fn build_int_mul(ctx: &mut MIRContext, block: &mut MIRBlock, left: MIRIntValue, right: MIRIntValue, signed: bool) -> BaseResult<MIRIntValue> {
	if left.size != right.size {
		return Err(BaseError::critical("Tried using imul on different sized integers".to_string()));
	}

	let res = block.append(ctx, MIRInstruction::IntegerMul { signed, left, right }).get()?;

	return res.as_int();
}

pub fn build_int_div(ctx: &mut MIRContext, block: &mut MIRBlock, left: MIRIntValue, right: MIRIntValue, signed: bool) -> BaseResult<MIRIntValue> {
	if left.size != right.size {
		return Err(BaseError::critical("Tried using idiv on different sized integers".to_string()));
	}

	let res = block.append(ctx, MIRInstruction::IntegerDiv { signed, left, right }).get()?;

	return res.as_int();
}

pub fn build_int_neg(ctx: &mut MIRContext, block: &mut MIRBlock, val: MIRIntValue) -> BaseResult<MIRIntValue> {
	let res = block.append(ctx, MIRInstruction::IntegerNeg { val }).get()?;

	return res.as_int();
}

pub fn build_float_add(ctx: &mut MIRContext, block: &mut MIRBlock, left: MIRFloatValue, right: MIRFloatValue, signed: bool) -> BaseResult<MIRFloatValue> {
	if left.exponent != right.exponent || left.fraction != right.fraction {
		return Err(BaseError::critical("Tried using fadd on different sized integers".to_string()));
	}

	let res = block.append(ctx, MIRInstruction::FloatAdd { signed, left, right }).get()?;

	return res.as_float();
}

pub fn build_float_sub(ctx: &mut MIRContext, block: &mut MIRBlock, left: MIRFloatValue, right: MIRFloatValue, signed: bool) -> BaseResult<MIRFloatValue> {
	if left.exponent != right.exponent || left.fraction != right.fraction {
		return Err(BaseError::critical("Tried using fsub on different sized integers".to_string()));
	}

	let res = block.append(ctx, MIRInstruction::FloatSub { signed, left, right }).get()?;

	return res.as_float();
}


pub fn build_float_mul(ctx: &mut MIRContext, block: &mut MIRBlock, left: MIRFloatValue, right: MIRFloatValue, signed: bool) -> BaseResult<MIRFloatValue> {
	if left.exponent != right.exponent || left.fraction != right.fraction {
		return Err(BaseError::critical("Tried using fmul on different sized integers".to_string()));
	}

	let res = block.append(ctx, MIRInstruction::FloatMul { signed, left, right }).get()?;

	return res.as_float();
}


pub fn build_float_div(ctx: &mut MIRContext, block: &mut MIRBlock, left: MIRFloatValue, right: MIRFloatValue, signed: bool) -> BaseResult<MIRFloatValue> {
	if left.exponent != right.exponent || left.fraction != right.fraction {
		return Err(BaseError::critical("Tried using fdiv on different sized integers".to_string()));
	}

	let res = block.append(ctx, MIRInstruction::FloatDiv { signed, left, right }).get()?;

	return res.as_float();
}


pub fn build_float_neg(ctx: &mut MIRContext, block: &mut MIRBlock, val: MIRFloatValue) -> BaseResult<MIRFloatValue> {
	let res = block.append(ctx, MIRInstruction::FloatNeg { val }).get()?;

	return res.as_float();
}

pub fn build_bitwise_and(ctx: &mut MIRContext, block: &mut MIRBlock, a: MIRIntValue, b: MIRIntValue) -> BaseResult<MIRIntValue> {
	if a.size != 1 || b.size != 1 {
		return Err(BaseError::critical("Tried using and on a non 1 bit value".to_string()));
	}

	let res = block.append(ctx, MIRInstruction::BitwiseAnd { a, b }).get()?;

	return res.as_int();
}

pub fn build_bitwise_or(ctx: &mut MIRContext, block: &mut MIRBlock, a: MIRIntValue, b: MIRIntValue) -> BaseResult<MIRIntValue> {
	if a.size != 1 || b.size != 1 {
		return Err(BaseError::critical("Tried using or on a non 1 bit value".to_string()));
	}

	let res = block.append(ctx, MIRInstruction::BitwiseOr { a, b }).get()?;

	return res.as_int();
}

pub fn build_bitwise_xor(ctx: &mut MIRContext, block: &mut MIRBlock, a: MIRIntValue, b: MIRIntValue) -> BaseResult<MIRIntValue> {
	if a.size != 1 || b.size != 1 {
		return Err(BaseError::critical("Tried using xor on a non 1 bit value".to_string()));
	}

	let res = block.append(ctx, MIRInstruction::BitwiseXor { a, b }).get()?;

	return res.as_int();
}

pub fn build_bitwise_not(ctx: &mut MIRContext, block: &mut MIRBlock, a: MIRIntValue) -> BaseResult<MIRIntValue> {
	if a.size != 1 {
		return Err(BaseError::critical("Tried using not on a non 1 bit value".to_string()));
	}

	let res = block.append(ctx, MIRInstruction::BitwiseNot { val: a }).get()?;

	return res.as_int();
}

pub fn build_shift_left(ctx: &mut MIRContext, block: &mut MIRBlock, val: MIRIntValue, shift: MIRIntValue) -> BaseResult<MIRIntValue> {
	let res = block.append(ctx, MIRInstruction::ShiftLeft { a: val, shift }).get()?;

	return res.as_int();
}

pub fn build_shift_right(ctx: &mut MIRContext, block: &mut MIRBlock, val: MIRIntValue, shift: MIRIntValue) -> BaseResult<MIRIntValue> {
	let res = block.append(ctx, MIRInstruction::ShiftRight { a: val, shift }).get()?;

	return res.as_int();
}

pub fn build_comp_eq(ctx: &mut MIRContext, block: &mut MIRBlock, a: MIRIntValue, b: MIRIntValue) -> BaseResult<MIRIntValue> {
	if a.size != b.size {
		return Err(BaseError::critical("Tried using cmpeq on different sized integers".to_string()));
	}

	let res = block.append(ctx, MIRInstruction::CompEq { a, b }).get()?;

	return res.as_int();
}

pub fn build_comp_neg(ctx: &mut MIRContext, block: &mut MIRBlock, a: MIRIntValue, b: MIRIntValue) -> BaseResult<MIRIntValue> {
	if a.size != b.size {
		return Err(BaseError::critical("Tried using cmpneg on different sized integers".to_string()));
	}

	let res = block.append(ctx, MIRInstruction::CompNeg { a, b }).get()?;

	return res.as_int();
}

pub fn build_comp_lt(ctx: &mut MIRContext, block: &mut MIRBlock, a: MIRIntValue, b: MIRIntValue) -> BaseResult<MIRIntValue> {
	if a.size != b.size {
		return Err(BaseError::critical("Tried using cmplt on different sized integers".to_string()));
	}

	let res = block.append(ctx, MIRInstruction::CompLt { a, b }).get()?;

	return res.as_int();
}

pub fn build_comp_le(ctx: &mut MIRContext, block: &mut MIRBlock, a: MIRIntValue, b: MIRIntValue) -> BaseResult<MIRIntValue> {
	if a.size != b.size {
		return Err(BaseError::critical("Tried using cmple on different sized integers".to_string()));
	}

	let res = block.append(ctx, MIRInstruction::CompLe { a, b }).get()?;

	return res.as_int();
}

pub fn build_comp_gt(ctx: &mut MIRContext, block: &mut MIRBlock, a: MIRIntValue, b: MIRIntValue) -> BaseResult<MIRIntValue> {
	if a.size != b.size {
		return Err(BaseError::critical("Tried using cmpgt on different sized integers".to_string()));
	}

	let res = block.append(ctx, MIRInstruction::CompGt { a, b }).get()?;

	return res.as_int();
}

pub fn build_comp_ge(ctx: &mut MIRContext, block: &mut MIRBlock, a: MIRIntValue, b: MIRIntValue) -> BaseResult<MIRIntValue> {
	if a.size != b.size {
		return Err(BaseError::critical("Tried using cmpge on different sized integers".to_string()));
	}

	let res = block.append(ctx, MIRInstruction::CompGe { a, b }).get()?;

	return res.as_int();
}

pub fn build_return(ctx: &mut MIRContext, block: &mut MIRBlock, val: BaseMIRValue) -> BaseResult<bool> {
	block.append(ctx, MIRInstruction::Return { val });

	Ok(true)
}

pub fn build_unconditional_branch(ctx: &mut MIRContext, block: &mut MIRBlock, func: &MIRFunction, branch: MIRBlockReference) -> BaseResult<bool> {
	if branch >= func.blocks.len() {	
		return Err(BaseError::err("Provided invalid block reference! to build_unconditional_branch".to_string()))
	}

	block.append(ctx, MIRInstruction::UnconditionalBranch { branch });
	Ok(true)
}

pub fn build_conditional_branch(ctx: &mut MIRContext, block: &mut MIRBlock, func: &MIRFunction, condition: MIRIntValue, if_branch: MIRBlockReference, else_branch: MIRBlockReference) -> BaseResult<bool> {
	if condition.size != 1 {
		return Err(BaseError::err("Provided cond to build_conditional_branch is not a boolean".to_string()));
	}

	if if_branch >= func.blocks.len() || else_branch >= func.blocks.len() {
		return Err(BaseError::err("Provided invalid block reference! to build_conditional_branch".to_string()))
	}

	block.append(ctx, MIRInstruction::ConditionalBranch { cond: condition, if_branch, else_branch });
	Ok(true)
}

pub fn build_phi(ctx: &mut MIRContext, block: &mut MIRBlock, func: &MIRFunction, choices: Vec<(MIRBlockReference, BaseMIRValue)>) -> BaseResult<BaseMIRValue> {
	let t = &choices[0].1.vtype;

	for choice in &choices {
		if choice.0 >= func.blocks.len() {
			return Err(BaseError::err("Provided invalid block reference to build_phi!".to_string()))
		}

		if !choice.1.vtype.base.is_equal(&t.base) {
			return Err(BaseError::err("Provided value to phi was not of the same type".to_string()));
		}
 	}

	return block.append(ctx, MIRInstruction::Phi { choices }).get()
}

pub fn build_select(ctx: &mut MIRContext, block: &mut MIRBlock, condition: MIRIntValue, if_val: BaseMIRValue, else_val: BaseMIRValue) -> BaseResult<BaseMIRValue> {
	if condition.size != 1 {
		return Err(BaseError::err("Provided cond to build_select is not a boolean".to_string()));
	}

	if !if_val.vtype.base.is_equal(&else_val.vtype.base) {
		return Err(BaseError::err("Both values do not have the same type in build_select!".to_string()))
	}

	return block.append(ctx, MIRInstruction::Select { cond: condition, if_val, else_val }).get()
}

pub fn build_field_pointer(ctx: &mut MIRContext, block: &mut MIRBlock, ptr: MIRPointerValue, field: usize) -> BaseResult<MIRPointerValue> {
	let val = block.append(ctx, MIRInstruction::FieldPointer { val: ptr.clone(), field }).get()?;
	let base: &BaseMIRValue = &ptr.into();

	let pointer_hint = ctx.ssa_hints.get_hint(base.get_ssa_index())?.as_pointer()?;
	let ptr_t = pointer_hint.base.get_struct_container()?;

	let t = CompactedType::from(ptr_t.fields.vals[field].clone());

	let ind = ctx.ssa_hints.append_hint(MIRValueHint::Pointer(t));

	if ind != val.get_ssa_index() {
		return Err(BaseError::err("Couldn't hint SSA value for pointer! Indexes are different".to_string()))
	}

	return val.as_ptr();
}

pub fn build_index_pointer(ctx: &mut MIRContext, block: &mut MIRBlock, val: MIRPointerValue, index: usize) -> BaseResult<bool> {
	block.append(ctx, MIRInstruction::IndexPointer { val, index });

	Ok(true)
}

pub fn build_marker_era_drop(ctx: &mut MIRContext, block: &mut MIRBlock, val: BaseMIRValue) -> BaseResult<bool> {
	block.append(ctx, MIRInstruction::MarkerEraDrop { value: val });

	Ok(true)
}

pub fn build_signed_int_const(ctx: &mut MIRContext, block: &mut MIRBlock, raw: i128, bitsize: usize) -> BaseResult<MIRIntValue> {
	let res = block.append(ctx, MIRInstruction::IntegerSignedConstant { raw, bitsize }).get()?;

	return res.as_int();
}

pub fn build_unsigned_int_const(ctx: &mut MIRContext, block: &mut MIRBlock, raw: u128, bitsize: usize) -> BaseResult<MIRIntValue> {
	let res = block.append(ctx, MIRInstruction::IntegerUnsignedConstant { raw, bitsize }).get()?;

	return res.as_int();
}

pub fn build_signed_float_const(ctx: &mut MIRContext, block: &mut MIRBlock, raw: f64, exponent: usize, fraction: usize) -> BaseResult<MIRFloatValue> {
	let res = block.append(ctx, MIRInstruction::FloatSignedConstant { raw, exponent, fraction }).get()?;

	return res.as_float();
}

pub fn build_unsigned_float_const(ctx: &mut MIRContext, block: &mut MIRBlock, raw: f64, exponent: usize, fraction: usize) -> BaseResult<MIRFloatValue> {
	let res = block.append(ctx, MIRInstruction::FloatUnsignedConstant { raw, exponent, fraction }).get()?;

	return res.as_float();
}

pub fn build_signed_fixed_const(ctx: &mut MIRContext, block: &mut MIRBlock, raw: f64, number: usize, fraction: usize) -> BaseResult<MIRIntValue> {
	let res = block.append(ctx, MIRInstruction::FixedSignedConstant { raw, number, fraction }).get()?;

	return res.as_int();
}

pub fn build_unsigned_fixed_const(ctx: &mut MIRContext, block: &mut MIRBlock, raw: f64, number: usize, fraction: usize) -> BaseResult<MIRIntValue> {
	let res = block.append(ctx, MIRInstruction::FixedUnsignedConstant { raw, number, fraction }).get()?;

	return res.as_int();
}

pub fn build_static_string_const(ctx: &mut MIRContext, block: &mut MIRBlock, raw: String) -> BaseResult<MIRPointerValue> {
	let res = block.append(ctx, MIRInstruction::StaticStringConstant { raw }).get()?;

	return res.as_ptr();
}

pub fn build_call(ctx: &mut MIRContext, block: &mut MIRBlock, func: usize, ind: usize, args: Vec<BaseMIRValue>) -> BaseResult<BaseMIRValue> {
	let func = &ctx.functions[func];

	for(arg, t) in args.iter().zip(func.arguments.iter()) {
		if !arg.vtype.eq(t) {
			return Err(BaseError::err(IR_FUNCTION_INVALID_ARGUMENTS!().to_string()));
		}
	}

	let res = block.append(ctx, MIRInstruction::Call { function: ind, arguments: args }).get()?;

	return Ok(res);
}