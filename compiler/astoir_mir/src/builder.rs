//! Utility functions to build instructions and more

use compiler_typing::{SizedType, raw::RawType, storage::TypeStorage, tree::Type};
use diagnostics::{DiagnosticResult, MaybeDiagnostic, unsure_panic};

use crate::{blocks::{hints::MIRValueHint, refer::MIRBlockReference}, ctx::MIRContext, insts::MIRInstruction, vals::{arrays::MIRArrayValue, base::BaseMIRValue, float::MIRFloatValue, int::MIRIntValue, ptr::MIRPointerValue, structs::MIRStructValue}};

pub fn build_stack_alloc(ctx: &mut MIRContext, size: usize, t: Type) -> DiagnosticResult<MIRPointerValue> {
	let res = ctx.append_inst(MIRInstruction::StackAlloc { alloc_size: size, t: t.clone() }).get()?;

	let hint = ctx.ssa_hints.append_hint(MIRValueHint::Pointer(t));

	if res.get_ssa_index() != hint {
		unsure_panic!("coudln't hint SSA value for pointers! indexes are different");
	}

	return res.as_ptr()
}

pub fn build_load(ctx: &mut MIRContext, ptr: MIRPointerValue) -> DiagnosticResult<BaseMIRValue> {
	let res = ctx.append_inst(MIRInstruction::Load { value: ptr }).get()?;

	return Ok(res);
}

pub fn build_store(ctx: &mut MIRContext, storage: &TypeStorage, ptr: MIRPointerValue, val: BaseMIRValue) -> DiagnosticResult<()> {
	let base: BaseMIRValue = ptr.clone().into();

	let hint = ctx.ssa_hints.get_hint(base.get_ssa_index()).get_type();

	if !hint.get_maybe_containing_type().is_truly_eq(&val.vtype) && !hint.is_ptr() {
		if hint.get_maybe_containing_type().get_generic(storage).is_enum_parent() && val.vtype.get_generic(storage).is_enum_child() {
			return build_store_fallback(ctx, ptr, val.clone(), storage)
		}

		unsure_panic!("cannot put this value onto this pointer since it's not the type");
	}

	ctx.append_inst(MIRInstruction::Store { variable: ptr, value: val });

	return Ok(()) 
}

pub fn build_downcast_int(ctx: &mut MIRContext, val: MIRIntValue, size: usize) -> DiagnosticResult<MIRIntValue> {
	if val.size <= size {
		unsure_panic!("tried downcasting a smaller int");
	}

	let res = ctx.append_inst(MIRInstruction::DowncastInteger { val, size }).get()?;

	return res.as_int();
}

pub fn build_upcast_int(ctx: &mut MIRContext, val: MIRIntValue, size: usize) -> DiagnosticResult<MIRIntValue> {
	if val.size >= size {
		unsure_panic!("tried upcasting on a higher sized int");
	}

	let res = ctx.append_inst(MIRInstruction::UpcastInteger { val, size }).get()?;

	return res.as_int();
}

pub fn build_downcast_float(ctx: &mut MIRContext, val: MIRFloatValue, size: usize) -> DiagnosticResult<MIRFloatValue> {
	if val.size <= size {
		unsure_panic!("tried downcasting on a smaller sized float");
	}

	let res = ctx.append_inst(MIRInstruction::DowncastFloat { val, size }).get()?;

	return res.as_float();
}

pub fn build_upcast_float(ctx: &mut MIRContext, val: MIRFloatValue, size: usize) -> DiagnosticResult<MIRFloatValue> {
	if val.size >= size {
		unsure_panic!("tried upcasting on a higher sized float");
	}

	let res = ctx.append_inst(MIRInstruction::UpcastFloat { val, size }).get()?;

	return res.as_float();
}

pub fn build_int_add(ctx: &mut MIRContext, left: MIRIntValue, right: MIRIntValue, signed: bool) -> DiagnosticResult<MIRIntValue> {
	if left.size != right.size {
		unsure_panic!("Tried using iadd on different sized integers");
	}

	let res = ctx.append_inst(MIRInstruction::IntegerAdd { signed, left, right }).get()?;

	return res.as_int();
}

pub fn build_int_sub(ctx: &mut MIRContext, left: MIRIntValue, right: MIRIntValue, signed: bool) -> DiagnosticResult<MIRIntValue> {
	if left.size != right.size {
		unsure_panic!("Tried using isub on different sized integers");
	}

	let res = ctx.append_inst(MIRInstruction::IntegerSub { signed, left, right }).get()?;

	return res.as_int();
}

pub fn build_int_mul(ctx: &mut MIRContext, left: MIRIntValue, right: MIRIntValue, signed: bool) -> DiagnosticResult<MIRIntValue> {
	if left.size != right.size {
		unsure_panic!("Tried using imul on different sized integers");
	}

	let res = ctx.append_inst(MIRInstruction::IntegerMul { signed, left, right }).get()?;

	return res.as_int();
}

pub fn build_int_div(ctx: &mut MIRContext, left: MIRIntValue, right: MIRIntValue, signed: bool) -> DiagnosticResult<MIRIntValue> {
	if left.size != right.size {
		unsure_panic!("Tried using idiv on different sized integers");
	}

	let res = ctx.append_inst(MIRInstruction::IntegerDiv { signed, left, right }).get()?;

	return res.as_int();
}

pub fn build_int_neg(ctx: &mut MIRContext, val: MIRIntValue) -> DiagnosticResult<MIRIntValue> {
	let res = ctx.append_inst(MIRInstruction::IntegerNeg { val }).get()?;

	return res.as_int();
}

pub fn build_float_add(ctx: &mut MIRContext, left: MIRFloatValue, right: MIRFloatValue, signed: bool) -> DiagnosticResult<MIRFloatValue> {
	if left.size != right.size {
		unsure_panic!("Tried using fadd on different sized integers");
	}

	let res = ctx.append_inst(MIRInstruction::FloatAdd { signed, left, right }).get()?;

	return res.as_float();
}

pub fn build_float_sub(ctx: &mut MIRContext, left: MIRFloatValue, right: MIRFloatValue, signed: bool) -> DiagnosticResult<MIRFloatValue> {
	if left.size != right.size {
		unsure_panic!("Tried using fsub on different sized integers");
	}

	let res = ctx.append_inst(MIRInstruction::FloatSub { signed, left, right }).get()?;

	return res.as_float();
}


pub fn build_float_mul(ctx: &mut MIRContext, left: MIRFloatValue, right: MIRFloatValue, signed: bool) -> DiagnosticResult<MIRFloatValue> {
	if left.size != right.size {
		unsure_panic!("Tried using fmul on different sized integers");
	}

	let res = ctx.append_inst(MIRInstruction::FloatMul { signed, left, right }).get()?;

	return res.as_float();
}


pub fn build_float_div(ctx: &mut MIRContext, left: MIRFloatValue, right: MIRFloatValue, signed: bool) -> DiagnosticResult<MIRFloatValue> {
	if left.size != right.size {
		unsure_panic!("Tried using fdiv on different sized integers");
	}

	let res = ctx.append_inst(MIRInstruction::FloatDiv { signed, left, right }).get()?;

	return res.as_float();
}


pub fn build_float_neg(ctx: &mut MIRContext, val: MIRFloatValue) -> DiagnosticResult<MIRFloatValue> {
	let res = ctx.append_inst(MIRInstruction::FloatNeg { val }).get()?;

	return res.as_float();
}

pub fn build_bitwise_and(ctx: &mut MIRContext, a: MIRIntValue, b: MIRIntValue) -> DiagnosticResult<MIRIntValue> {
	if a.size != 1 || b.size != 1 {
		unsure_panic!("Tried using and on a non 1 bit value");
	}

	let res = ctx.append_inst(MIRInstruction::BitwiseAnd { a, b }).get()?;

	return res.as_int();
}

pub fn build_bitwise_or(ctx: &mut MIRContext, a: MIRIntValue, b: MIRIntValue) -> DiagnosticResult<MIRIntValue> {
	if a.size != 1 || b.size != 1 {
		unsure_panic!("Tried using or on a non 1 bit value");
	}

	let res = ctx.append_inst(MIRInstruction::BitwiseOr { a, b }).get()?;

	return res.as_int();
}

pub fn build_bitwise_xor(ctx: &mut MIRContext, a: MIRIntValue, b: MIRIntValue) -> DiagnosticResult<MIRIntValue> {
	if a.size != 1 || b.size != 1 {
		unsure_panic!("Tried using xor on a non 1 bit value");
	}

	let res = ctx.append_inst(MIRInstruction::BitwiseXor { a, b }).get()?;

	return res.as_int();
}

pub fn build_bitwise_not(ctx: &mut MIRContext, a: MIRIntValue) -> DiagnosticResult<MIRIntValue> {
	if a.size != 1 {
		unsure_panic!("Tried using not on a non 1 bit value");
	}

	let res = ctx.append_inst(MIRInstruction::BitwiseNot { val: a }).get()?;

	return res.as_int();
}

pub fn build_shift_left(ctx: &mut MIRContext, val: MIRIntValue, shift: MIRIntValue) -> DiagnosticResult<MIRIntValue> {
	let res = ctx.append_inst(MIRInstruction::ShiftLeft { a: val, shift }).get()?;

	return res.as_int();
}

pub fn build_shift_right(ctx: &mut MIRContext, val: MIRIntValue, shift: MIRIntValue) -> DiagnosticResult<MIRIntValue> {
	let res = ctx.append_inst(MIRInstruction::ShiftRight { a: val, shift }).get()?;

	return res.as_int();
}

pub fn build_comp_eq(ctx: &mut MIRContext, a: MIRIntValue, b: MIRIntValue) -> DiagnosticResult<MIRIntValue> {
	if a.size != b.size {
		unsure_panic!("Tried using cmpeq on different sized integers");
	}

	let res = ctx.append_inst(MIRInstruction::CompEq { a, b }).get()?;

	return res.as_int();
}

pub fn build_comp_neg(ctx: &mut MIRContext, a: MIRIntValue, b: MIRIntValue) -> DiagnosticResult<MIRIntValue> {
	if a.size != b.size {
		unsure_panic!("Tried using cmpneg on different sized integers");
	}

	let res = ctx.append_inst(MIRInstruction::CompNeg { a, b }).get()?;

	return res.as_int();
}

pub fn build_comp_lt(ctx: &mut MIRContext, a: MIRIntValue, b: MIRIntValue) -> DiagnosticResult<MIRIntValue> {
	if a.size != b.size {
		unsure_panic!("Tried using cmplt on different sized integers");
	}

	let res = ctx.append_inst(MIRInstruction::CompLt { a, b }).get()?;

	return res.as_int();
}

pub fn build_comp_le(ctx: &mut MIRContext, a: MIRIntValue, b: MIRIntValue) -> DiagnosticResult<MIRIntValue> {
	if a.size != b.size {
		unsure_panic!("Tried using cmple on different sized integers");
	}

	let res = ctx.append_inst(MIRInstruction::CompLe { a, b }).get()?;

	return res.as_int();
}

pub fn build_comp_gt(ctx: &mut MIRContext, a: MIRIntValue, b: MIRIntValue) -> DiagnosticResult<MIRIntValue> {
	if a.size != b.size {
		unsure_panic!("Tried using cmpgt on different sized integers");
	}

	let res = ctx.append_inst(MIRInstruction::CompGt { a, b }).get()?;

	return res.as_int();
}

pub fn build_comp_ge(ctx: &mut MIRContext, a: MIRIntValue, b: MIRIntValue) -> DiagnosticResult<MIRIntValue> {
	if a.size != b.size {
		unsure_panic!("Tried using cmpge on different sized integers");
	}

	let res = ctx.append_inst(MIRInstruction::CompGe { a, b }).get()?;

	return res.as_int();
}

pub fn build_return(ctx: &mut MIRContext, val: Option<BaseMIRValue>) -> DiagnosticResult<bool> {
	ctx.append_inst(MIRInstruction::Return { val });

	Ok(true)
}

pub fn build_unconditional_branch(ctx: &mut MIRContext, branch: MIRBlockReference) -> DiagnosticResult<bool> {
	ctx.append_inst(MIRInstruction::UnconditionalBranch { branch });
	Ok(true)
}

pub fn build_conditional_branch(ctx: &mut MIRContext, condition: MIRIntValue, if_branch: MIRBlockReference, else_branch: MIRBlockReference) -> DiagnosticResult<bool> {
	if condition.size != 1 {
		unsure_panic!("provided cond to build_conditional_branch is not a boolean");
	}

	ctx.append_inst(MIRInstruction::ConditionalBranch { cond: condition, if_branch, else_branch });
	Ok(true)
}

pub fn build_select(ctx: &mut MIRContext, condition: MIRIntValue, if_val: BaseMIRValue, else_val: BaseMIRValue) -> DiagnosticResult<BaseMIRValue> {
	if condition.size != 1 {
		unsure_panic!("provided cond to build_select is not a boolean");
	}

	if if_val.vtype != else_val.vtype {
		unsure_panic!("both values do not have the same type");
	}

	return ctx.append_inst(MIRInstruction::Select { cond: condition, if_val, else_val }).get()
}

pub fn build_field_pointer(ctx: &mut MIRContext, ptr: MIRPointerValue, field: usize) -> DiagnosticResult<MIRPointerValue> {
	let val = ctx.append_inst(MIRInstruction::FieldPointer { val: ptr.clone(), field }).get()?;
	let base: &BaseMIRValue = &ptr.into();

	let pointer_hint = ctx.ssa_hints.get_hint(base.get_ssa_index()).as_pointer().as_generic_lowered();
	let t;

	if let RawType::LoweredStruct(_, container) = pointer_hint {
		t = container.fields.vals[field].clone();
	} else {
		unsure_panic!("field pointer hint was not a lowered struct");
	}

	let ind = ctx.ssa_hints.append_hint(MIRValueHint::Pointer(t));

	if ind != val.get_ssa_index() {
		unsure_panic!("couldn't hint SSA value for pointer! indexes are different");
	}

	return val.as_ptr();
}

pub fn build_index_pointer(ctx: &mut MIRContext, val: MIRPointerValue, index: MIRIntValue) -> DiagnosticResult<MIRPointerValue> {
	let res = ctx.append_inst(MIRInstruction::IndexPointer { val: val.clone(), index }).get()?;
	let base: BaseMIRValue = MIRPointerValue::into(val);

	let t = base.vtype.get_inner_type();

	let ind = ctx.ssa_hints.append_hint(MIRValueHint::Pointer(*t));

	if ind != res.get_ssa_index() {
		unsure_panic!("couldn't hint SSA value for pointer! indexes are different");
	}

	res.as_ptr()
}

pub fn build_marker_era_drop(ctx: &mut MIRContext, val: BaseMIRValue) -> DiagnosticResult<bool> {
	ctx.append_inst(MIRInstruction::MarkerEraDrop { value: val });

	Ok(true)
}

pub fn build_signed_int_const(ctx: &mut MIRContext, raw: i128, bitsize: usize) -> DiagnosticResult<MIRIntValue> {
	let res = ctx.append_inst(MIRInstruction::IntegerSignedConstant { raw, bitsize }).get()?;

	return res.as_int();
}

pub fn build_unsigned_int_const(ctx: &mut MIRContext, raw: u128, bitsize: usize) -> DiagnosticResult<MIRIntValue> {
	let res = ctx.append_inst(MIRInstruction::IntegerUnsignedConstant { raw, bitsize }).get()?;

	return res.as_int();
}

pub fn build_signed_float_const(ctx: &mut MIRContext, raw: f64, size: usize) -> DiagnosticResult<MIRFloatValue> {
	let res = ctx.append_inst(MIRInstruction::FloatSignedConstant { raw, size }).get()?;

	return res.as_float();
}

pub fn build_unsigned_float_const(ctx: &mut MIRContext, raw: f64, size: usize) -> DiagnosticResult<MIRFloatValue> {
	let res = ctx.append_inst(MIRInstruction::FloatUnsignedConstant { raw, size }).get()?;

	return res.as_float();
}

pub fn build_signed_fixed_const(ctx: &mut MIRContext, raw: f64, number: usize, fraction: usize) -> DiagnosticResult<MIRIntValue> {
	let res = ctx.append_inst(MIRInstruction::FixedSignedConstant { raw, number, fraction }).get()?;

	return res.as_int();
}

pub fn build_unsigned_fixed_const(ctx: &mut MIRContext, raw: f64, number: usize, fraction: usize) -> DiagnosticResult<MIRIntValue> {
	let res = ctx.append_inst(MIRInstruction::FixedUnsignedConstant { raw, number, fraction }).get()?;

	return res.as_int();
}

pub fn build_static_string_const(ctx: &mut MIRContext, raw: String) -> DiagnosticResult<MIRPointerValue> {
	let res = ctx.append_inst(MIRInstruction::StaticStringConstant { raw }).get()?;

	return res.as_ptr();
}

pub fn build_static_struct_const(ctx: &mut MIRContext, struct_type: RawType, values: Vec<BaseMIRValue>) -> DiagnosticResult<MIRStructValue> {
	let res = ctx.append_inst(MIRInstruction::StructInitializerConstant { struct_type, values }).get()?;

	return res.as_struct();
}

pub fn build_static_array_const(ctx: &mut MIRContext, values: Vec<BaseMIRValue>) -> DiagnosticResult<MIRArrayValue> {
	let res = ctx.append_inst(MIRInstruction::ArrayInitializerConstant { values }).get()?;

	return res.as_array();
}

pub fn build_static_array_one_const(ctx: &mut MIRContext, val: BaseMIRValue, size: usize) -> DiagnosticResult<MIRArrayValue> {
	let res = ctx.append_inst(MIRInstruction::ArrayInitializerConstantSame { size, val }).get()?;

	return res.as_array();
}

pub fn build_argument_grab(ctx: &mut MIRContext, index: usize, t: Type) -> DiagnosticResult<BaseMIRValue> {
	let res = ctx.append_inst(MIRInstruction::FuncArgumentGrab { ind: index, argtype: t }).get()?;

	return Ok(res);
}

pub fn build_call(ctx: &mut MIRContext, func: usize, ind: usize, args: Vec<BaseMIRValue>) -> DiagnosticResult<Option<BaseMIRValue>> {
	let func = &ctx.functions[func];

	for(arg, t) in args.iter().zip(func.arguments.iter()) {
		if !arg.vtype.is_truly_eq(t) {
			println!("{:#?} -> {:#?}", arg.vtype, t);
			unsure_panic!("invalid function argument types!");
		}
	}

	let res = ctx.append_inst(MIRInstruction::Call { function: ind, arguments: args }).val;

	return Ok(res);
}

pub fn build_phi(ctx: &mut MIRContext, choices: Vec<(MIRBlockReference, BaseMIRValue)>) -> DiagnosticResult<BaseMIRValue> {
	let t = &choices[0].1.vtype;

	for choice in &choices {
		if &choice.1.vtype != t {
			unsure_panic!("phi node must have same type values");
		}
	}

	let res = ctx.append_inst(MIRInstruction::Phi { choices }).get()?;

	return Ok(res);
}

pub fn build_ir_cast(ctx: &mut MIRContext, val: BaseMIRValue, to: Type) -> DiagnosticResult<BaseMIRValue> {
	let res = ctx.append_inst(MIRInstruction::IRCast { val, to }).get()?;

	return Ok(res);
}

pub fn build_memory_copy_unsafe(ctx: &mut MIRContext, src: MIRPointerValue, dest: MIRPointerValue, size: usize) {
	ctx.append_inst(MIRInstruction::MemoryCopy { src, dest, sz: size });
}

/// Fallback to whenever store fails since the type isn't valid. Allows to use unsafe memory copy for enums
/// # Behvior
/// If `src` is actually a pointer, simply uses the `unsmemcopy` instruction to copy the memory from `src` to `dest`. 
/// If not, it first creates a pointer for `src`, storing the value inside and then using the `unsmemcopy` instruction.
pub fn build_store_fallback(ctx: &mut MIRContext, dest: MIRPointerValue, src: BaseMIRValue, storage: &TypeStorage) -> MaybeDiagnostic {
	let sz = src.vtype.get_size(&src.vtype, false, storage);

	if src.can_be_pointer() {
		let src = src.as_ptr()?;
	
		ctx.append_inst(MIRInstruction::MemoryCopy { src, dest, sz: sz});
		return Ok(())
	}	

	let ptr = build_stack_alloc(ctx, sz, src.vtype.clone())?;

	build_store(ctx, storage, ptr.clone(), src)?;

	ctx.append_inst(MIRInstruction::MemoryCopy { src: ptr, dest, sz });

	Ok(())
}	