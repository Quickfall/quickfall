//! Utility functions to build instructions and more

use astoir_typing::compacted::CompactedType;
use compiler_errors::errs::{BaseResult, base::BaseError};

use crate::{blocks::{MIRBlock, hints::MIRValueHint, refer::MIRBlockReference}, funcs::MIRFunction, insts::MIRInstruction, vals::{base::{BaseMIRValue}, float::MIRFloatValue, int::MIRIntValue, ptr::MIRPointerValue}};

pub fn build_stack_alloc(block: &mut MIRBlock, size: usize, t: CompactedType) -> BaseResult<MIRPointerValue> {
	let res = block.append(MIRInstruction::StackAlloc { alloc_size: size, t: t.clone() }).get()?;

	block.hints.append_hint(res.get_instruction(), MIRValueHint::Pointer(t));

	return res.as_ptr()
}

pub fn build_load(block: &mut MIRBlock, ptr: MIRPointerValue) -> BaseResult<BaseMIRValue> {
	let res = block.append(MIRInstruction::Load { value: ptr }).get()?;

	return Ok(res);
}

pub fn build_store(block: &mut MIRBlock, ptr: MIRPointerValue, val: BaseMIRValue) -> BaseResult<bool> {
	let base: BaseMIRValue = ptr.clone().into();

	let hint = block.hints.get_hint(base.get_instruction())?.as_pointer()?;

	if !hint.eq(&val.vtype) {
		return Err(BaseError::err("Cannot put this value onto this pointer as it is not the same type!".to_string()))
	}

	block.append(MIRInstruction::Store { variable: ptr, value: val });

	return Ok(true) 
}

pub fn build_downcast_int(block: &mut MIRBlock, val: MIRIntValue, size: usize) -> BaseResult<MIRIntValue> {
	if val.size <= size {
		return Err(BaseError::critical("Tried using downintcast on a smaller sized int!".to_string()));
	}

	let res = block.append(MIRInstruction::DowncastInteger { val, size }).get()?;

	return res.as_int();
}

pub fn build_upcast_int(block: &mut MIRBlock, val: MIRIntValue, size: usize) -> BaseResult<MIRIntValue> {
	if val.size >= size {
		return Err(BaseError::critical("Tried using upintcast on a higher sized int!".to_string()));
	}

	let res = block.append(MIRInstruction::UpcastInteger { val, size }).get()?;

	return res.as_int();
}

pub fn build_downcast_float(block: &mut MIRBlock, val: MIRFloatValue, size: usize) -> BaseResult<MIRFloatValue> {
	if val.size <= size {
		return Err(BaseError::critical("Tried using downfloatcast on a smaller sized int!".to_string()));
	}

	let res = block.append(MIRInstruction::DowncastFloat { val, size }).get()?;

	return res.as_float();
}

pub fn build_upcast_float(block: &mut MIRBlock, val: MIRFloatValue, size: usize) -> BaseResult<MIRFloatValue> {
	if val.size >= size {
		return Err(BaseError::critical("Tried using upfloatcast on a higher sized int!".to_string()));
	}

	let res = block.append(MIRInstruction::UpcastFloat { val, size }).get()?;

	return res.as_float();
}

pub fn build_int_add(block: &mut MIRBlock, left: MIRIntValue, right: MIRIntValue, signed: bool) -> BaseResult<MIRIntValue> {
	if left.size != right.size {
		return Err(BaseError::critical("Tried using iadd on different sized integers".to_string()));
	}

	let res = block.append(MIRInstruction::IntegerAdd { signed, left, right }).get()?;

	return res.as_int();
}

pub fn build_int_sub(block: &mut MIRBlock, left: MIRIntValue, right: MIRIntValue, signed: bool) -> BaseResult<MIRIntValue> {
	if left.size != right.size {
		return Err(BaseError::critical("Tried using isub on different sized integers".to_string()));
	}

	let res = block.append(MIRInstruction::IntegerSub { signed, left, right }).get()?;

	return res.as_int();
}

pub fn build_int_mul(block: &mut MIRBlock, left: MIRIntValue, right: MIRIntValue, signed: bool) -> BaseResult<MIRIntValue> {
	if left.size != right.size {
		return Err(BaseError::critical("Tried using imul on different sized integers".to_string()));
	}

	let res = block.append(MIRInstruction::IntegerMul { signed, left, right }).get()?;

	return res.as_int();
}

pub fn build_int_div(block: &mut MIRBlock, left: MIRIntValue, right: MIRIntValue, signed: bool) -> BaseResult<MIRIntValue> {
	if left.size != right.size {
		return Err(BaseError::critical("Tried using idiv on different sized integers".to_string()));
	}

	let res = block.append(MIRInstruction::IntegerDiv { signed, left, right }).get()?;

	return res.as_int();
}

pub fn build_int_neg(block: &mut MIRBlock, val: MIRIntValue) -> BaseResult<MIRIntValue> {
	let res = block.append(MIRInstruction::IntegerNeg { val }).get()?;

	return res.as_int();
}

pub fn build_float_add(block: &mut MIRBlock, left: MIRFloatValue, right: MIRFloatValue, signed: bool) -> BaseResult<MIRFloatValue> {
	if left.size != right.size {
		return Err(BaseError::critical("Tried using fadd on different sized integers".to_string()));
	}

	let res = block.append(MIRInstruction::FloatAdd { signed, left, right }).get()?;

	return res.as_float();
}

pub fn build_float_sub(block: &mut MIRBlock, left: MIRFloatValue, right: MIRFloatValue, signed: bool) -> BaseResult<MIRFloatValue> {
	if left.size != right.size {
		return Err(BaseError::critical("Tried using fsub on different sized integers".to_string()));
	}

	let res = block.append(MIRInstruction::FloatSub { signed, left, right }).get()?;

	return res.as_float();
}


pub fn build_float_mul(block: &mut MIRBlock, left: MIRFloatValue, right: MIRFloatValue, signed: bool) -> BaseResult<MIRFloatValue> {
	if left.size != right.size {
		return Err(BaseError::critical("Tried using fmul on different sized integers".to_string()));
	}

	let res = block.append(MIRInstruction::FloatMul { signed, left, right }).get()?;

	return res.as_float();
}


pub fn build_float_div(block: &mut MIRBlock, left: MIRFloatValue, right: MIRFloatValue, signed: bool) -> BaseResult<MIRFloatValue> {
	if left.size != right.size {
		return Err(BaseError::critical("Tried using fdiv on different sized integers".to_string()));
	}

	let res = block.append(MIRInstruction::FloatDiv { signed, left, right }).get()?;

	return res.as_float();
}


pub fn build_float_neg(block: &mut MIRBlock, val: MIRFloatValue) -> BaseResult<MIRFloatValue> {
	let res = block.append(MIRInstruction::FloatNeg { val }).get()?;

	return res.as_float();
}

pub fn build_bitwise_and(block: &mut MIRBlock, a: MIRIntValue, b: MIRIntValue) -> BaseResult<MIRIntValue> {
	if a.size != 1 || b.size != 1 {
		return Err(BaseError::critical("Tried using and on a non 1 bit value".to_string()));
	}

	let res = block.append(MIRInstruction::BitwiseAnd { a, b }).get()?;

	return res.as_int();
}

pub fn build_bitwise_or(block: &mut MIRBlock, a: MIRIntValue, b: MIRIntValue) -> BaseResult<MIRIntValue> {
	if a.size != 1 || b.size != 1 {
		return Err(BaseError::critical("Tried using or on a non 1 bit value".to_string()));
	}

	let res = block.append(MIRInstruction::BitwiseOr { a, b }).get()?;

	return res.as_int();
}

pub fn build_bitwise_xor(block: &mut MIRBlock, a: MIRIntValue, b: MIRIntValue) -> BaseResult<MIRIntValue> {
	if a.size != 1 || b.size != 1 {
		return Err(BaseError::critical("Tried using xor on a non 1 bit value".to_string()));
	}

	let res = block.append(MIRInstruction::BitwiseXor { a, b }).get()?;

	return res.as_int();
}

pub fn build_bitwise_not(block: &mut MIRBlock, a: MIRIntValue) -> BaseResult<MIRIntValue> {
	if a.size != 1 {
		return Err(BaseError::critical("Tried using not on a non 1 bit value".to_string()));
	}

	let res = block.append(MIRInstruction::BitwiseNot { val: a }).get()?;

	return res.as_int();
}

pub fn build_shift_left(block: &mut MIRBlock, val: MIRIntValue, shift: MIRIntValue) -> BaseResult<MIRIntValue> {
	let res = block.append(MIRInstruction::ShiftLeft { a: val, shift }).get()?;

	return res.as_int();
}

pub fn build_shift_right(block: &mut MIRBlock, val: MIRIntValue, shift: MIRIntValue) -> BaseResult<MIRIntValue> {
	let res = block.append(MIRInstruction::ShiftRight { a: val, shift }).get()?;

	return res.as_int();
}

pub fn build_comp_eq(block: &mut MIRBlock, a: MIRIntValue, b: MIRIntValue) -> BaseResult<MIRIntValue> {
	if a.size != b.size {
		return Err(BaseError::critical("Tried using cmpeq on different sized integers".to_string()));
	}

	let res = block.append(MIRInstruction::CompEq { a, b }).get()?;

	return res.as_int();
}

pub fn build_comp_neg(block: &mut MIRBlock, a: MIRIntValue, b: MIRIntValue) -> BaseResult<MIRIntValue> {
	if a.size != b.size {
		return Err(BaseError::critical("Tried using cmpneg on different sized integers".to_string()));
	}

	let res = block.append(MIRInstruction::CompNeg { a, b }).get()?;

	return res.as_int();
}

pub fn build_comp_lt(block: &mut MIRBlock, a: MIRIntValue, b: MIRIntValue) -> BaseResult<MIRIntValue> {
	if a.size != b.size {
		return Err(BaseError::critical("Tried using cmplt on different sized integers".to_string()));
	}

	let res = block.append(MIRInstruction::CompLt { a, b }).get()?;

	return res.as_int();
}

pub fn build_comp_le(block: &mut MIRBlock, a: MIRIntValue, b: MIRIntValue) -> BaseResult<MIRIntValue> {
	if a.size != b.size {
		return Err(BaseError::critical("Tried using cmple on different sized integers".to_string()));
	}

	let res = block.append(MIRInstruction::CompLe { a, b }).get()?;

	return res.as_int();
}

pub fn build_comp_gt(block: &mut MIRBlock, a: MIRIntValue, b: MIRIntValue) -> BaseResult<MIRIntValue> {
	if a.size != b.size {
		return Err(BaseError::critical("Tried using cmpgt on different sized integers".to_string()));
	}

	let res = block.append(MIRInstruction::CompGt { a, b }).get()?;

	return res.as_int();
}

pub fn build_comp_ge(block: &mut MIRBlock, a: MIRIntValue, b: MIRIntValue) -> BaseResult<MIRIntValue> {
	if a.size != b.size {
		return Err(BaseError::critical("Tried using cmpge on different sized integers".to_string()));
	}

	let res = block.append(MIRInstruction::CompGe { a, b }).get()?;

	return res.as_int();
}

pub fn build_return(block: &mut MIRBlock, val: BaseMIRValue) -> BaseResult<bool> {
	block.append(MIRInstruction::Return { val });

	Ok(true)
}

pub fn build_unconditional_branch(block: &mut MIRBlock, func: &MIRFunction, branch: MIRBlockReference) -> BaseResult<bool> {
	if branch >= func.blocks.len() {	
		return Err(BaseError::err("Provided invalid block reference! to build_unconditional_branch".to_string()))
	}

	block.append(MIRInstruction::UnconditionalBranch { branch });
	Ok(true)
}

pub fn build_conditional_branch(block: &mut MIRBlock, func: &MIRFunction, condition: MIRIntValue, if_branch: MIRBlockReference, else_branch: MIRBlockReference) -> BaseResult<bool> {
	if condition.size != 1 {
		return Err(BaseError::err("Provided cond to build_conditional_branch is not a boolean".to_string()));
	}

	if if_branch >= func.blocks.len() || else_branch >= func.blocks.len() {
		return Err(BaseError::err("Provided invalid block reference! to build_conditional_branch".to_string()))
	}

	block.append(MIRInstruction::ConditionalBranch { cond: condition, if_branch, else_branch });
	Ok(true)
}

pub fn build_phi(block: &mut MIRBlock, func: &MIRFunction, choices: Vec<(MIRBlockReference, BaseMIRValue)>) -> BaseResult<BaseMIRValue> {
	let t = &choices[0].1.vtype;

	for choice in &choices {
		if choice.0 >= func.blocks.len() {
			return Err(BaseError::err("Provided invalid block reference to build_phi!".to_string()))
		}

		if !choice.1.vtype.eq(t) {
			return Err(BaseError::err("Provided value to phi was not of the same type".to_string()));
		}
 	}

	return block.append(MIRInstruction::Phi { choices }).get()
}

pub fn build_select(block: &mut MIRBlock, condition: MIRIntValue, if_val: BaseMIRValue, else_val: BaseMIRValue) -> BaseResult<BaseMIRValue> {
	if condition.size != 1 {
		return Err(BaseError::err("Provided cond to build_select is not a boolean".to_string()));
	}

	if !if_val.vtype.eq(&else_val.vtype) {
		return Err(BaseError::err("Both values do not have the same type in build_select!".to_string()))
	}

	return block.append(MIRInstruction::Select { cond: condition, if_val, else_val }).get()
}

pub fn build_field_pointer(block: &mut MIRBlock, ptr: MIRPointerValue, field: usize) -> BaseResult<MIRPointerValue> {
	let val = block.append(MIRInstruction::FieldPointer { val: ptr.clone(), field }).get()?;
	let base: &BaseMIRValue = &ptr.into();

	let ptr_t = block.hints.get_hint(base.get_instruction())?.as_pointer()?.as_struct()?;
	let field_type = lower_astoir_typing_type(ptr_t.fields.vals[field].get_concrete().clone())?;

	block.hints.append_hint(val.get_instruction(), MIRValueHint::Pointer(field_type));

	return val.as_ptr();
}

pub fn build_index_pointer(block: &mut MIRBlock, val: MIRPointerValue, index: usize) -> BaseResult<bool> {
	block.append(MIRInstruction::IndexPointer { val, index });

	Ok(true)
}

pub fn build_marker_era_drop(block: &mut MIRBlock, val: BaseMIRValue) -> BaseResult<bool> {
	block.append(MIRInstruction::MarkerEraDrop { value: val });

	Ok(true)
}

pub fn build_signed_int_const(block: &mut MIRBlock, raw: i128, bitsize: usize) -> BaseResult<MIRIntValue> {
	let res = block.append(MIRInstruction::IntegerSignedConstant { raw, bitsize }).get()?;

	return res.as_int();
}

pub fn build_unsigned_int_const(block: &mut MIRBlock, raw: u128, bitsize: usize) -> BaseResult<MIRIntValue> {
	let res = block.append(MIRInstruction::IntegerUnsignedConstant { raw, bitsize }).get()?;

	return res.as_int();
}

pub fn build_signed_float_const(block: &mut MIRBlock, raw: f64, bitsize: usize) -> BaseResult<MIRFloatValue> {
	let res = block.append(MIRInstruction::FloatSignedConstant { raw, bitsize }).get()?;

	return res.as_float();
}

pub fn build_unsigned_float_const(block: &mut MIRBlock, raw: f64, bitsize: usize) -> BaseResult<MIRFloatValue> {
	let res = block.append(MIRInstruction::FloatUnsignedConstant { raw, bitsize }).get()?;

	return res.as_float();
}

pub fn build_signed_fixed_const(block: &mut MIRBlock, raw: f64, bitsize: usize) -> BaseResult<MIRIntValue> {
	let res = block.append(MIRInstruction::FixedSignedConstant { raw, bitsize }).get()?;

	return res.as_int();
}

pub fn build_unsigned_fixed_const(block: &mut MIRBlock, raw: f64, bitsize: usize) -> BaseResult<MIRIntValue> {
	let res = block.append(MIRInstruction::FixedUnsignedConstant { raw, bitsize }).get()?;

	return res.as_int();
}

pub fn build_static_string_const(block: &mut MIRBlock, raw: String) -> BaseResult<MIRPointerValue> {
	let res = block.append(MIRInstruction::StaticStringConstant { raw }).get()?;

	return res.as_ptr();
}