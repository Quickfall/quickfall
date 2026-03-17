//! The definitions for instructions within the MIR. 

use std::fmt::Display;

use astoir_typing::{base::BaseType, compacted::CompactedType};

use crate::{blocks::{refer::MIRBlockReference}, ctx::MIRContext, vals::{base::BaseMIRValue, float::MIRFloatValue, int::MIRIntValue, ptr::MIRPointerValue}};

pub mod val;

/// An instruction inside of the MIR.
#[derive(Clone)]
pub enum MIRInstruction {
	StackAlloc { alloc_size: usize, t: CompactedType },
	Load { value: MIRPointerValue },
	Store { variable: MIRPointerValue, value: BaseMIRValue }, 

	// Number casting
	DowncastInteger { val: MIRIntValue, size: usize }, // make size smaller
	UpcastInteger { val: MIRIntValue, size: usize },  // make size bigger

	DowncastFloat { val: MIRFloatValue, exponent: usize, fraction: usize }, 
	UpcastFloat { val: MIRFloatValue, exponent: usize, fraction: usize }, 

	// Arithmetrics
	IntegerAdd { signed: bool, left: MIRIntValue, right: MIRIntValue }, 
	IntegerSub { signed: bool, left: MIRIntValue, right: MIRIntValue }, 
	IntegerMul { signed: bool, left: MIRIntValue, right: MIRIntValue }, 
	IntegerDiv { signed: bool, left: MIRIntValue, right: MIRIntValue },
	IntegerMod { signed: bool, left: MIRIntValue, right: MIRIntValue }, 
	IntegerNeg { val: MIRIntValue }, 
	
	FloatAdd { signed: bool, left: MIRFloatValue, right: MIRFloatValue }, 
	FloatSub { signed: bool, left: MIRFloatValue, right: MIRFloatValue }, 
	FloatMul { signed: bool, left: MIRFloatValue, right: MIRFloatValue }, 
	FloatDiv { signed: bool, left: MIRFloatValue, right: MIRFloatValue },
	FloatNeg { val: MIRFloatValue }, 

	// Bitwise (int typed)
	BitwiseAnd { a: MIRIntValue, b: MIRIntValue }, 
	BitwiseOr { a: MIRIntValue, b: MIRIntValue }, 
	BitwiseXor { a: MIRIntValue, b: MIRIntValue }, 
	BitwiseNot { val: MIRIntValue }, 
	
	ShiftLeft { a: MIRIntValue, shift: MIRIntValue }, 
	ShiftRight { a: MIRIntValue, shift: MIRIntValue }, 

	// Comparaison / Logical
	CompEq { a: MIRIntValue, b: MIRIntValue }, 
	CompNeg { a: MIRIntValue, b: MIRIntValue }, 
	CompLt { a: MIRIntValue, b: MIRIntValue}, // <
	CompLe { a: MIRIntValue, b: MIRIntValue}, // <=
	CompGt { a: MIRIntValue, b: MIRIntValue }, // >
	CompGe { a: MIRIntValue, b: MIRIntValue}, // >=

	// Constants
	IntegerSignedConstant { raw: i128, bitsize: usize },
	IntegerUnsignedConstant { raw: u128, bitsize: usize }, 
	FloatSignedConstant { raw: f64, exponent: usize, fraction: usize }, 
	FloatUnsignedConstant { raw: f64, exponent: usize, fraction: usize }, 
	FixedSignedConstant { raw: f64, number: usize, fraction: usize }, 
	FixedUnsignedConstant { raw: f64, number: usize, fraction: usize }, 
	StaticStringConstant { raw: String },

	// Control
	Return { val: Option<BaseMIRValue> }, 
	UnconditionalBranch { branch: MIRBlockReference },
	ConditionalBranch { cond: MIRIntValue, if_branch: MIRBlockReference, else_branch: MIRBlockReference }, 
	Phi { choices: Vec<(MIRBlockReference, BaseMIRValue)> },
	Select { cond: MIRIntValue, if_val: BaseMIRValue, else_val: BaseMIRValue },

	Call { function: usize, arguments: Vec<BaseMIRValue> },

	// Pointer utils

	FieldPointer { val: MIRPointerValue, field: usize },
	IndexPointer { val: MIRPointerValue, index: MIRIntValue }, 
	PointerAdd { pointer: MIRPointerValue, right: MIRIntValue }, 
	PointerSub { pointer: MIRPointerValue, right: MIRIntValue }, 

	FuncArgumentGrab { ind: usize, argtype: CompactedType },

	/// Indicates to the IR processor that this given value's era is finished and thus we drop the value
	MarkerEraDrop { value: BaseMIRValue },
}

impl MIRInstruction {
	pub fn has_return(&self, ctx: &MIRContext) -> bool {
		match self {
			Self::MarkerEraDrop { .. } | Self::UnconditionalBranch { .. } | Self::ConditionalBranch { .. } | Self::Return { .. } => {
				return false;
			},

			Self::Call { function, arguments: _ } => {
				let func = &ctx.functions[*function];

				return func.return_type.is_some();
			}

			_ => true
		}
	}

	pub fn should_hint(&self) -> bool {
		return match self {
			Self::StackAlloc { .. } => false,
			Self::FieldPointer { .. } => false,
			Self::IndexPointer { .. } => false,

			_ => true
		}
	}

	pub fn get_return_type(&self, ctx: &MIRContext) -> CompactedType {
		match self {
			Self::StackAlloc { .. } => return CompactedType::from(BaseType::Pointer),
			Self::Load { value} => {
				let base: BaseMIRValue = value.clone().into();

				let hint = ctx.ssa_hints.get_hint(base.get_ssa_index()).unwrap();

				return hint.as_pointer().unwrap();
			},

			Self::DowncastInteger { val, size } => return CompactedType::from(BaseType::NumericIntegerType(*size as u64, val.signed)),
			Self::UpcastInteger { val, size } => return CompactedType::from(BaseType::NumericIntegerType(*size as u64, val.signed)),

			Self::DowncastFloat { val, exponent, fraction } => return CompactedType::from(BaseType::FloatingNumberType(*exponent as u64, *fraction as u64, val.signed)),
			Self::UpcastFloat { val, exponent, fraction } => return CompactedType::from(BaseType::FloatingNumberType(*exponent as u64, *fraction as u64, val.signed)),

			Self::IntegerAdd { signed, left, right: _ } => return CompactedType::from(BaseType::NumericIntegerType(left.size as u64, *signed)), 
			Self::IntegerSub { signed, left, right: _ } => return CompactedType::from(BaseType::NumericIntegerType(left.size as u64, *signed)), 
			Self::IntegerMul { signed, left, right: _ } => return CompactedType::from(BaseType::NumericIntegerType(left.size as u64, *signed)), 
			Self::IntegerDiv { signed, left, right: _ } => return CompactedType::from(BaseType::NumericIntegerType(left.size as u64, *signed)), 
			Self::IntegerMod { signed, left, right: _ } => return CompactedType::from(BaseType::NumericIntegerType(left.size as u64, *signed)), 
			Self::IntegerNeg { val } => return CompactedType::from(BaseType::NumericIntegerType(val.size as u64, true)),

			Self::FloatAdd { signed: _, left, right: _ } => return CompactedType::from(BaseType::FloatingNumberType(left.exponent as u64, left.fraction as u64, left.signed)),
			Self::FloatSub { signed: _, left, right: _ } => return CompactedType::from(BaseType::FloatingNumberType(left.exponent as u64, left.fraction as u64, left.signed)),
			Self::FloatMul { signed: _, left, right: _ } => return CompactedType::from(BaseType::FloatingNumberType(left.exponent as u64, left.fraction as u64, left.signed)),
			Self::FloatDiv { signed: _, left, right: _ } => return CompactedType::from(BaseType::FloatingNumberType(left.exponent as u64, left.fraction as u64, left.signed)),
			Self::FloatNeg { val } => return CompactedType::from(BaseType::FloatingNumberType(val.exponent as u64, val.fraction as u64, true)),

			Self::BitwiseAnd { a, b: _ } => return CompactedType::from(BaseType::NumericIntegerType(a.size as u64, a.signed)),
			Self::BitwiseOr { a, b: _ } => return CompactedType::from(BaseType::NumericIntegerType(a.size as u64, a.signed)),
			Self::BitwiseXor { a, b: _ } => return CompactedType::from(BaseType::NumericIntegerType(a.size as u64, a.signed)),
			Self::BitwiseNot { val } => return CompactedType::from(BaseType::NumericIntegerType(val.size as u64, val.signed)),

			Self::ShiftLeft { a, shift: _ } => return CompactedType::from(BaseType::NumericIntegerType(a.size as u64, a.signed)),
			Self::ShiftRight { a, shift: _ } => return CompactedType::from(BaseType::NumericIntegerType(a.size as u64, a.signed)),

			Self::CompEq { .. } => return CompactedType::from(BaseType::Boolean),
			Self::CompNeg { .. } => return CompactedType::from(BaseType::Boolean),
			Self::CompLt { .. } => return CompactedType::from(BaseType::Boolean),
			Self::CompLe { .. } => return CompactedType::from(BaseType::Boolean),
			Self::CompGt { .. } => return CompactedType::from(BaseType::Boolean),
			Self::CompGe { .. } => return CompactedType::from(BaseType::Boolean),

			Self::IntegerSignedConstant { raw: _, bitsize } => return CompactedType::from(BaseType::NumericIntegerType(*bitsize as u64, true)),
			Self::IntegerUnsignedConstant { raw: _, bitsize } => return CompactedType::from(BaseType::NumericIntegerType(*bitsize as u64, true)),
			Self::FloatUnsignedConstant { raw: _, exponent, fraction } => return CompactedType::from(BaseType::FloatingNumberType(*exponent as u64, *fraction as u64, false)),
			Self::FloatSignedConstant { raw: _, exponent, fraction } => return CompactedType::from(BaseType::FloatingNumberType(*exponent as u64, *fraction as u64, true)),
			Self::FixedSignedConstant { raw: _, number, fraction } => return CompactedType::from(BaseType::NumericIntegerType(*number as u64 + *fraction as u64, true)),
			Self::FixedUnsignedConstant { raw: _, number, fraction } => return CompactedType::from(BaseType::NumericIntegerType(*number as u64 + *fraction as u64, false)),
			Self::StaticStringConstant { raw: _ } => return CompactedType::from(BaseType::Pointer),

			Self::Phi { choices } => {
				return choices[0].1.vtype.clone();
			},

			Self::Select { cond: _, if_val, else_val: _ } => return if_val.vtype.clone(),

			Self::Call { function, arguments: _ } => {
				let func = &ctx.functions[*function];

				return func.return_type.clone().unwrap();
			}
			
			Self::FieldPointer { .. } => return CompactedType::from(BaseType::Pointer),
			Self::IndexPointer { .. } => return CompactedType::from(BaseType::Pointer),

			Self::PointerAdd { .. } => return CompactedType::from(BaseType::Pointer),
			Self::PointerSub { .. } => return CompactedType::from(BaseType::Pointer), 

			Self::FuncArgumentGrab { ind: _, argtype } => argtype.clone(),

			_ => panic!("Tried using get_return_type on non returning type!")
		}
	}
}

impl Display for MIRInstruction {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::StackAlloc { alloc_size, t } => writeln!(f, "stkalloc {}", *alloc_size)?,
			Self::Load { value } => writeln!(f, "load {}", value)?,
			Self::Store { variable, value } => writeln!(f, "store d{} s{}", variable, value)?,
			
			Self::DowncastInteger { val, size } => writeln!(f, "dintcast {} {}", val, size)?,
			Self::DowncastFloat { val, exponent, fraction } => writeln!(f, "dfcast {} {} {}", val, exponent, fraction)?,
			Self::UpcastInteger { val, size } => writeln!(f, "uintcast {} {}", val, size)?,
			Self::UpcastFloat { val, exponent, fraction } => writeln!(f, "ufcast {} {} {}", val, exponent, fraction)?,

			Self::IntegerAdd { signed, left, right } => writeln!(f, "iadd s{} {} {}", signed, left, right)?,
			Self::IntegerSub { signed, left, right } => writeln!(f, "isub s{} {} {}", signed, left, right)?,
			Self::IntegerMul { signed, left, right } => writeln!(f, "imul s{} {} {}", signed, left, right)?,
			Self::IntegerDiv { signed, left, right } => writeln!(f, "idiv s{} {} {}", signed, left, right)?,
			Self::IntegerMod { signed, left, right } => writeln!(f, "imod s{} {} {}", signed, left, right)?,
			Self::IntegerNeg { val } => writeln!(f, "ineg {}", val)?,

			Self::FloatAdd { signed, left, right } => writeln!(f, "fadd s{} {} {}", signed, left, right)?,
			Self::FloatSub { signed, left, right } => writeln!(f, "fsub s{} {} {}", signed, left, right)?,
			Self::FloatMul { signed, left, right } => writeln!(f, "fmul s{} {} {}", signed, left, right)?,
			Self::FloatDiv { signed, left, right } => writeln!(f, "fdiv s{} {} {}", signed, left, right)?,
			Self::FloatNeg { val } => writeln!(f, "fneg {}", val)?,

			Self::BitwiseAnd { a, b } => writeln!(f, "and {} {}", a, b)?,
			Self::BitwiseOr { a, b } => writeln!(f, "or {} {}", a, b)?,
			Self::BitwiseXor { a, b } => writeln!(f, "xor {} {}", a, b)?,
			Self::BitwiseNot { val } => writeln!(f, "not {}", val)?,

			Self::ShiftLeft { a, shift } => writeln!(f, "shiftl {} {}", a, shift)?,
			Self::ShiftRight { a, shift } => writeln!(f, "shiftr {} {}", a, shift)?,

			Self::CompEq { a, b } => writeln!(f, "eq {} {}", a, b)?,
			Self::CompNeg { a, b } => writeln!(f, "ne {} {}", a, b)?,
			Self::CompLt { a, b } => writeln!(f, "lt {} {}", a, b)?,
			Self::CompLe { a, b } => writeln!(f, "le {} {}", a, b)?,
			Self::CompGt { a, b } => writeln!(f, "gt {} {}", a, b)?,
			Self::CompGe { a, b } => writeln!(f, "ge {} {}", a, b)?,

			Self::IntegerSignedConstant { raw, bitsize } => writeln!(f, "constints {} {}", raw, bitsize)?,
			Self::IntegerUnsignedConstant { raw, bitsize } => writeln!(f, "constintu {} {}", raw, bitsize)?,

			Self::FloatSignedConstant { raw, exponent, fraction } => writeln!(f, "constfs {} {} {}", raw, exponent, fraction)?,
			Self::FloatUnsignedConstant { raw, exponent, fraction } => writeln!(f, "constfu {} {} {}", raw, exponent, fraction)?,

			Self::FixedSignedConstant { raw, number, fraction } => writeln!(f, "constffs {} {} {}", raw, number, fraction)?,
			Self::FixedUnsignedConstant { raw, number, fraction } => writeln!(f, "constffu {} {} {}", raw, number, fraction)?,

			Self::StaticStringConstant { raw } => writeln!(f, "conststr {}", raw)?,

			Self::Return { val } => {
				if val.is_some() {
					writeln!(f, "ret {}", val.clone().unwrap())?;
				} else {
					writeln!(f, "ret")?;
				}
			},
			
			Self::UnconditionalBranch { branch } => writeln!(f, "ucondbr {}", branch)?,
			Self::ConditionalBranch { cond, if_branch, else_branch } => writeln!(f, "condbr {} {} {}", cond, if_branch, else_branch)?,

			Self::Phi { choices } => {
				write!(f, "phi")?;

				for choice in choices {
					write!(f, " [b{}, {}]", choice.0, choice.1)?;
				}

				write!(f, "\n")?;
			},

			Self::Select { cond, if_val, else_val } => writeln!(f, "select {} {} {}", cond, if_val, else_val)?,

			Self::Call { function, arguments } => {
				write!(f, "call {}", function)?;

				for arg in arguments {
					write!(f, " {}", arg)?;
				}

				write!(f, "\n")?;
			},

			Self::FieldPointer { val, field } => writeln!(f, "fieldptr {} {}", val, field)?,
			Self::IndexPointer { val, index } => writeln!(f, "indptr {} {}", val, index)?,

			Self::PointerAdd { pointer, right } => writeln!(f, "ptradd {} {}", pointer, right)?,
			Self::PointerSub { pointer, right } => writeln!(f, "ptrsub {} {}", pointer, right)?,

			Self::FuncArgumentGrab { ind, argtype: _ } => writeln!(f, "funcarg {}", ind)?,

			Self::MarkerEraDrop { value } => writeln!(f, ".marker_era_drop {}", value)?
		}

		Ok(())
	}
}
