//! Utility functions to build instructions and more

use compiler_errors::errs::{BaseResult, base::BaseError};

use crate::{blocks::MIRBlock, insts::MIRInstruction, vals::{base::{BaseMIRValue, BaseValueType}, float::MIRFloatValue, int::MIRIntValue, ptr::MIRPointerValue}};

pub fn build_stack_alloc(block: &mut MIRBlock, size: usize, t: BaseValueType) -> BaseResult<MIRPointerValue> {
	let res = block.append(MIRInstruction::StackAlloc { alloc_size: size, t }).get()?;

	return res.as_ptr()
}

pub fn build_load(block: &mut MIRBlock, ptr: MIRPointerValue) -> BaseResult<BaseMIRValue> {
	let res = block.append(MIRInstruction::Load { value: ptr }).get()?;

	return Ok(res);
}

pub fn build_store(block: &mut MIRBlock, ptr: MIRPointerValue, val: BaseMIRValue) -> BaseResult<bool> {
	// TODO: add block enforcing
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

pub fn build_field_pointer(block: &mut MIRBlock, val: MIRPointerValue, field: usize) -> BaseResult<bool> {
	block.append(MIRInstruction::FieldPointer { val, field });

	Ok(true)
}

pub fn build_index_pointer(block: &mut MIRBlock, val: MIRPointerValue, index: usize) -> BaseResult<bool> {
	block.append(MIRInstruction::IndexPointer { val, index });

	Ok(true)
}

pub fn build_marker_era_drop(block: &mut MIRBlock, val: BaseMIRValue) -> BaseResult<bool> {
	block.append(MIRInstruction::MarkerEraDrop { value: val });

	Ok(true)
}