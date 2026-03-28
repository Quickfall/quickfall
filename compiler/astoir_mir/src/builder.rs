//! Utility functions to build instructions and more

use compiler_errors::{IR_FUNCTION_INVALID_ARGUMENTS, errs::{BaseResult, base::BaseError}};
use compiler_typing::{raw::RawType, tree::Type};

use crate::{blocks::{hints::MIRValueHint, refer::MIRBlockReference}, ctx::MIRContext, insts::MIRInstruction, vals::{arrays::MIRArrayValue, base::BaseMIRValue, float::MIRFloatValue, int::MIRIntValue, ptr::MIRPointerValue, structs::MIRStructValue}};

pub fn build_stack_alloc(ctx: &mut MIRContext, size: usize, t: Type) -> BaseResult<MIRPointerValue> {
	let res = ctx.append_inst(MIRInstruction::StackAlloc { alloc_size: size, t: t.clone() }).get()?;

	let hint = ctx.ssa_hints.append_hint(MIRValueHint::Pointer(t));

	if res.get_ssa_index() != hint {
		return Err(BaseError::err("Couldn't hint SSA value for pointer! Indexes are different".to_string()))
	}

	return res.as_ptr()
}

pub fn build_load(ctx: &mut MIRContext, ptr: MIRPointerValue) -> BaseResult<BaseMIRValue> {
	let res = ctx.append_inst(MIRInstruction::Load { value: ptr }).get()?;

	return Ok(res);
}

pub fn build_store(ctx: &mut MIRContext, ptr: MIRPointerValue, val: BaseMIRValue) -> BaseResult<bool> {
	let base: BaseMIRValue = ptr.clone().into();

	let hint = ctx.ssa_hints.get_hint(base.get_ssa_index())?.as_pointer()?;

	if hint != val.vtype {
		return Err(BaseError::err(format!("Cannot put this value onto this pointer as it is not the same type! {:#?} - {:#?}", hint, val.vtype)))
	}

	ctx.append_inst(MIRInstruction::Store { variable: ptr, value: val });

	return Ok(true) 
}

pub fn build_downcast_int(ctx: &mut MIRContext, val: MIRIntValue, size: usize) -> BaseResult<MIRIntValue> {
	if val.size <= size {
		return Err(BaseError::critical("Tried using downintcast on a smaller sized int!".to_string()));
	}

	let res = ctx.append_inst(MIRInstruction::DowncastInteger { val, size }).get()?;

	return res.as_int();
}

pub fn build_upcast_int(ctx: &mut MIRContext, val: MIRIntValue, size: usize) -> BaseResult<MIRIntValue> {
	if val.size >= size {
		return Err(BaseError::critical("Tried using upintcast on a higher sized int!".to_string()));
	}

	let res = ctx.append_inst(MIRInstruction::UpcastInteger { val, size }).get()?;

	return res.as_int();
}

pub fn build_downcast_float(ctx: &mut MIRContext, val: MIRFloatValue, size: usize) -> BaseResult<MIRFloatValue> {
	if val.size <= size {
		return Err(BaseError::critical("Tried using downfloatcast on a smaller sized int!".to_string()));
	}

	let res = ctx.append_inst(MIRInstruction::DowncastFloat { val, size }).get()?;

	return res.as_float();
}

pub fn build_upcast_float(ctx: &mut MIRContext, val: MIRFloatValue, size: usize) -> BaseResult<MIRFloatValue> {
	if val.size >= size {
		return Err(BaseError::critical("Tried using upfloatcast on a higher sized int!".to_string()));
	}

	let res = ctx.append_inst(MIRInstruction::UpcastFloat { val, size }).get()?;

	return res.as_float();
}

pub fn build_int_add(ctx: &mut MIRContext, left: MIRIntValue, right: MIRIntValue, signed: bool) -> BaseResult<MIRIntValue> {
	if left.size != right.size {
		return Err(BaseError::critical("Tried using iadd on different sized integers".to_string()));
	}

	let res = ctx.append_inst(MIRInstruction::IntegerAdd { signed, left, right }).get()?;

	return res.as_int();
}

pub fn build_int_sub(ctx: &mut MIRContext, left: MIRIntValue, right: MIRIntValue, signed: bool) -> BaseResult<MIRIntValue> {
	if left.size != right.size {
		return Err(BaseError::critical("Tried using isub on different sized integers".to_string()));
	}

	let res = ctx.append_inst(MIRInstruction::IntegerSub { signed, left, right }).get()?;

	return res.as_int();
}

pub fn build_int_mul(ctx: &mut MIRContext, left: MIRIntValue, right: MIRIntValue, signed: bool) -> BaseResult<MIRIntValue> {
	if left.size != right.size {
		return Err(BaseError::critical("Tried using imul on different sized integers".to_string()));
	}

	let res = ctx.append_inst(MIRInstruction::IntegerMul { signed, left, right }).get()?;

	return res.as_int();
}

pub fn build_int_div(ctx: &mut MIRContext, left: MIRIntValue, right: MIRIntValue, signed: bool) -> BaseResult<MIRIntValue> {
	if left.size != right.size {
		return Err(BaseError::critical("Tried using idiv on different sized integers".to_string()));
	}

	let res = ctx.append_inst(MIRInstruction::IntegerDiv { signed, left, right }).get()?;

	return res.as_int();
}

pub fn build_int_neg(ctx: &mut MIRContext, val: MIRIntValue) -> BaseResult<MIRIntValue> {
	let res = ctx.append_inst(MIRInstruction::IntegerNeg { val }).get()?;

	return res.as_int();
}

pub fn build_float_add(ctx: &mut MIRContext, left: MIRFloatValue, right: MIRFloatValue, signed: bool) -> BaseResult<MIRFloatValue> {
	if left.size != right.size {
		return Err(BaseError::critical("Tried using fadd on different sized integers".to_string()));
	}

	let res = ctx.append_inst(MIRInstruction::FloatAdd { signed, left, right }).get()?;

	return res.as_float();
}

pub fn build_float_sub(ctx: &mut MIRContext, left: MIRFloatValue, right: MIRFloatValue, signed: bool) -> BaseResult<MIRFloatValue> {
	if left.size != right.size {
		return Err(BaseError::critical("Tried using fsub on different sized integers".to_string()));
	}

	let res = ctx.append_inst(MIRInstruction::FloatSub { signed, left, right }).get()?;

	return res.as_float();
}


pub fn build_float_mul(ctx: &mut MIRContext, left: MIRFloatValue, right: MIRFloatValue, signed: bool) -> BaseResult<MIRFloatValue> {
	if left.size != right.size {
		return Err(BaseError::critical("Tried using fmul on different sized integers".to_string()));
	}

	let res = ctx.append_inst(MIRInstruction::FloatMul { signed, left, right }).get()?;

	return res.as_float();
}


pub fn build_float_div(ctx: &mut MIRContext, left: MIRFloatValue, right: MIRFloatValue, signed: bool) -> BaseResult<MIRFloatValue> {
	if left.size != right.size {
		return Err(BaseError::critical("Tried using fdiv on different sized integers".to_string()));
	}

	let res = ctx.append_inst(MIRInstruction::FloatDiv { signed, left, right }).get()?;

	return res.as_float();
}


pub fn build_float_neg(ctx: &mut MIRContext, val: MIRFloatValue) -> BaseResult<MIRFloatValue> {
	let res = ctx.append_inst(MIRInstruction::FloatNeg { val }).get()?;

	return res.as_float();
}

pub fn build_bitwise_and(ctx: &mut MIRContext, a: MIRIntValue, b: MIRIntValue) -> BaseResult<MIRIntValue> {
	if a.size != 1 || b.size != 1 {
		return Err(BaseError::critical("Tried using and on a non 1 bit value".to_string()));
	}

	let res = ctx.append_inst(MIRInstruction::BitwiseAnd { a, b }).get()?;

	return res.as_int();
}

pub fn build_bitwise_or(ctx: &mut MIRContext, a: MIRIntValue, b: MIRIntValue) -> BaseResult<MIRIntValue> {
	if a.size != 1 || b.size != 1 {
		return Err(BaseError::critical("Tried using or on a non 1 bit value".to_string()));
	}

	let res = ctx.append_inst(MIRInstruction::BitwiseOr { a, b }).get()?;

	return res.as_int();
}

pub fn build_bitwise_xor(ctx: &mut MIRContext, a: MIRIntValue, b: MIRIntValue) -> BaseResult<MIRIntValue> {
	if a.size != 1 || b.size != 1 {
		return Err(BaseError::critical("Tried using xor on a non 1 bit value".to_string()));
	}

	let res = ctx.append_inst(MIRInstruction::BitwiseXor { a, b }).get()?;

	return res.as_int();
}

pub fn build_bitwise_not(ctx: &mut MIRContext, a: MIRIntValue) -> BaseResult<MIRIntValue> {
	if a.size != 1 {
		return Err(BaseError::critical("Tried using not on a non 1 bit value".to_string()));
	}

	let res = ctx.append_inst(MIRInstruction::BitwiseNot { val: a }).get()?;

	return res.as_int();
}

pub fn build_shift_left(ctx: &mut MIRContext, val: MIRIntValue, shift: MIRIntValue) -> BaseResult<MIRIntValue> {
	let res = ctx.append_inst(MIRInstruction::ShiftLeft { a: val, shift }).get()?;

	return res.as_int();
}

pub fn build_shift_right(ctx: &mut MIRContext, val: MIRIntValue, shift: MIRIntValue) -> BaseResult<MIRIntValue> {
	let res = ctx.append_inst(MIRInstruction::ShiftRight { a: val, shift }).get()?;

	return res.as_int();
}

pub fn build_comp_eq(ctx: &mut MIRContext, a: MIRIntValue, b: MIRIntValue) -> BaseResult<MIRIntValue> {
	if a.size != b.size {
		return Err(BaseError::critical("Tried using cmpeq on different sized integers".to_string()));
	}

	let res = ctx.append_inst(MIRInstruction::CompEq { a, b }).get()?;

	return res.as_int();
}

pub fn build_comp_neg(ctx: &mut MIRContext, a: MIRIntValue, b: MIRIntValue) -> BaseResult<MIRIntValue> {
	if a.size != b.size {
		return Err(BaseError::critical("Tried using cmpneg on different sized integers".to_string()));
	}

	let res = ctx.append_inst(MIRInstruction::CompNeg { a, b }).get()?;

	return res.as_int();
}

pub fn build_comp_lt(ctx: &mut MIRContext, a: MIRIntValue, b: MIRIntValue) -> BaseResult<MIRIntValue> {
	if a.size != b.size {
		return Err(BaseError::critical("Tried using cmplt on different sized integers".to_string()));
	}

	let res = ctx.append_inst(MIRInstruction::CompLt { a, b }).get()?;

	return res.as_int();
}

pub fn build_comp_le(ctx: &mut MIRContext, a: MIRIntValue, b: MIRIntValue) -> BaseResult<MIRIntValue> {
	if a.size != b.size {
		return Err(BaseError::critical("Tried using cmple on different sized integers".to_string()));
	}

	let res = ctx.append_inst(MIRInstruction::CompLe { a, b }).get()?;

	return res.as_int();
}

pub fn build_comp_gt(ctx: &mut MIRContext, a: MIRIntValue, b: MIRIntValue) -> BaseResult<MIRIntValue> {
	if a.size != b.size {
		return Err(BaseError::critical("Tried using cmpgt on different sized integers".to_string()));
	}

	let res = ctx.append_inst(MIRInstruction::CompGt { a, b }).get()?;

	return res.as_int();
}

pub fn build_comp_ge(ctx: &mut MIRContext, a: MIRIntValue, b: MIRIntValue) -> BaseResult<MIRIntValue> {
	if a.size != b.size {
		return Err(BaseError::critical("Tried using cmpge on different sized integers".to_string()));
	}

	let res = ctx.append_inst(MIRInstruction::CompGe { a, b }).get()?;

	return res.as_int();
}

pub fn build_return(ctx: &mut MIRContext, val: Option<BaseMIRValue>) -> BaseResult<bool> {
	ctx.append_inst(MIRInstruction::Return { val });

	Ok(true)
}

pub fn build_unconditional_branch(ctx: &mut MIRContext, branch: MIRBlockReference) -> BaseResult<bool> {
	ctx.append_inst(MIRInstruction::UnconditionalBranch { branch });
	Ok(true)
}

pub fn build_conditional_branch(ctx: &mut MIRContext, condition: MIRIntValue, if_branch: MIRBlockReference, else_branch: MIRBlockReference) -> BaseResult<bool> {
	if condition.size != 1 {
		return Err(BaseError::err("Provided cond to build_conditional_branch is not a boolean".to_string()));
	}

	ctx.append_inst(MIRInstruction::ConditionalBranch { cond: condition, if_branch, else_branch });
	Ok(true)
}

pub fn build_select(ctx: &mut MIRContext, condition: MIRIntValue, if_val: BaseMIRValue, else_val: BaseMIRValue) -> BaseResult<BaseMIRValue> {
	if condition.size != 1 {
		return Err(BaseError::err("Provided cond to build_select is not a boolean".to_string()));
	}

	if if_val.vtype != else_val.vtype {
		return Err(BaseError::err("Both values do not have the same type in build_select!".to_string()))
	}

	return ctx.append_inst(MIRInstruction::Select { cond: condition, if_val, else_val }).get()
}

pub fn build_field_pointer(ctx: &mut MIRContext, ptr: MIRPointerValue, field: usize) -> BaseResult<MIRPointerValue> {
	let val = ctx.append_inst(MIRInstruction::FieldPointer { val: ptr.clone(), field }).get()?;
	let base: &BaseMIRValue = &ptr.into();

	let pointer_hint = ctx.ssa_hints.get_hint(base.get_ssa_index())?.as_pointer()?.as_generic_lowered()?;
	let t;

	if let RawType::LoweredStruct(_, container) = pointer_hint {
		t = container.fields.vals[field].clone();
	} else {
		return Err(BaseError::err("Field pointer hint was not a lowered struct!".to_string()))
	}

	let ind = ctx.ssa_hints.append_hint(MIRValueHint::Pointer(t));

	if ind != val.get_ssa_index() {
		return Err(BaseError::err("Couldn't hint SSA value for pointer! Indexes are different".to_string()))
	}

	return val.as_ptr();
}

pub fn build_index_pointer(ctx: &mut MIRContext, val: MIRPointerValue, index: MIRIntValue) -> BaseResult<MIRPointerValue> {
	let res = ctx.append_inst(MIRInstruction::IndexPointer { val, index }).get()?;

	Ok(res.as_ptr()?)
}

pub fn build_marker_era_drop(ctx: &mut MIRContext, val: BaseMIRValue) -> BaseResult<bool> {
	ctx.append_inst(MIRInstruction::MarkerEraDrop { value: val });

	Ok(true)
}

pub fn build_signed_int_const(ctx: &mut MIRContext, raw: i128, bitsize: usize) -> BaseResult<MIRIntValue> {
	let res = ctx.append_inst(MIRInstruction::IntegerSignedConstant { raw, bitsize }).get()?;

	return res.as_int();
}

pub fn build_unsigned_int_const(ctx: &mut MIRContext, raw: u128, bitsize: usize) -> BaseResult<MIRIntValue> {
	let res = ctx.append_inst(MIRInstruction::IntegerUnsignedConstant { raw, bitsize }).get()?;

	return res.as_int();
}

pub fn build_signed_float_const(ctx: &mut MIRContext, raw: f64, size: usize) -> BaseResult<MIRFloatValue> {
	let res = ctx.append_inst(MIRInstruction::FloatSignedConstant { raw, size }).get()?;

	return res.as_float();
}

pub fn build_unsigned_float_const(ctx: &mut MIRContext, raw: f64, size: usize) -> BaseResult<MIRFloatValue> {
	let res = ctx.append_inst(MIRInstruction::FloatUnsignedConstant { raw, size }).get()?;

	return res.as_float();
}

pub fn build_signed_fixed_const(ctx: &mut MIRContext, raw: f64, number: usize, fraction: usize) -> BaseResult<MIRIntValue> {
	let res = ctx.append_inst(MIRInstruction::FixedSignedConstant { raw, number, fraction }).get()?;

	return res.as_int();
}

pub fn build_unsigned_fixed_const(ctx: &mut MIRContext, raw: f64, number: usize, fraction: usize) -> BaseResult<MIRIntValue> {
	let res = ctx.append_inst(MIRInstruction::FixedUnsignedConstant { raw, number, fraction }).get()?;

	return res.as_int();
}

pub fn build_static_string_const(ctx: &mut MIRContext, raw: String) -> BaseResult<MIRPointerValue> {
	let res = ctx.append_inst(MIRInstruction::StaticStringConstant { raw }).get()?;

	return res.as_ptr();
}

pub fn build_static_struct_const(ctx: &mut MIRContext, struct_type: RawType, values: Vec<BaseMIRValue>) -> BaseResult<MIRStructValue> {
	let res = ctx.append_inst(MIRInstruction::StructInitializerConstant { struct_type, values }).get()?;

	return res.as_struct();
}

pub fn build_static_array_const(ctx: &mut MIRContext, values: Vec<BaseMIRValue>) -> BaseResult<MIRArrayValue> {
	let res = ctx.append_inst(MIRInstruction::ArrayInitializerConstant { values }).get()?;

	return res.as_array();
}

pub fn build_static_array_one_const(ctx: &mut MIRContext, val: BaseMIRValue, size: usize) -> BaseResult<MIRArrayValue> {
	let res = ctx.append_inst(MIRInstruction::ArrayInitializerConstantSame { size, val }).get()?;

	return res.as_array();
}

pub fn build_argument_grab(ctx: &mut MIRContext, index: usize, t: Type) -> BaseResult<BaseMIRValue> {
	let res = ctx.append_inst(MIRInstruction::FuncArgumentGrab { ind: index, argtype: t }).get()?;

	return Ok(res);
}

pub fn build_call(ctx: &mut MIRContext, func: usize, ind: usize, args: Vec<BaseMIRValue>) -> BaseResult<Option<BaseMIRValue>> {
	let func = &ctx.functions[func];

	for(arg, t) in args.iter().zip(func.arguments.iter()) {
		if !arg.vtype.is_truly_eq(t) {
			return Err(BaseError::err(format!(IR_FUNCTION_INVALID_ARGUMENTS!(), arg.vtype, t)));
		}
	}

	let res = ctx.append_inst(MIRInstruction::Call { function: ind, arguments: args }).val;

	return Ok(res);
}

pub fn build_phi(ctx: &mut MIRContext, choices: Vec<(MIRBlockReference, BaseMIRValue)>) -> BaseResult<BaseMIRValue> {
	let t = &choices[0].1.vtype;

	for choice in &choices {
		if &choice.1.vtype != t {
			return Err(BaseError::err("phi node must have same type values".to_string()));
		}
	}

	let res = ctx.append_inst(MIRInstruction::Phi { choices }).get()?;

	return Ok(res);
}