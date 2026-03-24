//! The definitions for instructions within the MIR. 

use std::fmt::Display;

use compiler_typing::{raw::RawType, tree::Type};

use crate::{blocks::{refer::MIRBlockReference}, ctx::MIRContext, vals::{base::BaseMIRValue, float::MIRFloatValue, int::MIRIntValue, ptr::MIRPointerValue}};

pub mod val;

/// An instruction inside of the MIR.
#[derive(Clone)]
pub enum MIRInstruction {
	StackAlloc { alloc_size: usize, t: RawType },
	Load { value: MIRPointerValue },
	Store { variable: MIRPointerValue, value: BaseMIRValue }, 

	// Number casting
	DowncastInteger { val: MIRIntValue, size: usize }, // make size smaller
	UpcastInteger { val: MIRIntValue, size: usize },  // make size bigger

	DowncastFloat { val: MIRFloatValue, size: usize }, 
	UpcastFloat { val: MIRFloatValue, size: usize }, 

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
	FloatSignedConstant { raw: f64, size: usize }, 
	FloatUnsignedConstant { raw: f64, size: usize }, 
	FixedSignedConstant { raw: f64, number: usize, fraction: usize }, 
	FixedUnsignedConstant { raw: f64, number: usize, fraction: usize }, 
	StaticStringConstant { raw: String },

	StructInitializerConstant { struct_type: RawType, values: Vec<BaseMIRValue> },

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

	FuncArgumentGrab { ind: usize, argtype: Type },

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

	pub fn get_return_type(&self, ctx: &MIRContext) -> Type {
		match self {
			Self::StackAlloc { .. } => return Type::GenericLowered(RawType::Pointer),
			Self::Load { value} => {
				let base: BaseMIRValue = value.clone().into();

				let hint = ctx.ssa_hints.get_hint(base.get_ssa_index()).unwrap();

				return hint.as_pointer().unwrap();
			},

			Self::DowncastInteger { val, size } => return Type::GenericLowered(RawType::Integer(*size, val.signed)),
			Self::UpcastInteger { val, size } => return Type::GenericLowered(RawType::Integer(*size, val.signed)),

			Self::DowncastFloat { val, size } => return Type::GenericLowered(RawType::Floating(*size, val.signed)),
			Self::UpcastFloat { val, size } => return Type::GenericLowered(RawType::Floating(*size, val.signed)),

			Self::IntegerAdd { signed, left, right: _ } => return Type::GenericLowered(RawType::Integer(left.size, *signed)), 
			Self::IntegerSub { signed, left, right: _ } => return Type::GenericLowered(RawType::Integer(left.size, *signed)), 
			Self::IntegerMul { signed, left, right: _ } => return Type::GenericLowered(RawType::Integer(left.size, *signed)), 
			Self::IntegerDiv { signed, left, right: _ } => return Type::GenericLowered(RawType::Integer(left.size, *signed)), 
			Self::IntegerMod { signed, left, right: _ } => return Type::GenericLowered(RawType::Integer(left.size, *signed)), 
			Self::IntegerNeg { val } => return Type::GenericLowered(RawType::Integer(val.size, true)),

			Self::FloatAdd { signed, left, right: _ } => return Type::GenericLowered(RawType::Floating(left.size, *signed)),
			Self::FloatSub { signed, left, right: _ } => return Type::GenericLowered(RawType::Floating(left.size, *signed)),
			Self::FloatMul { signed, left, right: _ } => return Type::GenericLowered(RawType::Floating(left.size, *signed)),
			Self::FloatDiv { signed, left, right: _ } => return Type::GenericLowered(RawType::Floating(left.size, *signed)),
			Self::FloatNeg { val } => return Type::GenericLowered(RawType::Floating(val.size, true)),

			Self::BitwiseAnd { a, b: _ } => return Type::GenericLowered(RawType::Integer(a.size, a.signed)),
			Self::BitwiseOr { a, b: _ } => return Type::GenericLowered(RawType::Integer(a.size, a.signed)),
			Self::BitwiseXor { a, b: _ } => return Type::GenericLowered(RawType::Integer(a.size, a.signed)),
			Self::BitwiseNot { val } => return Type::GenericLowered(RawType::Integer(val.size, val.signed)),

			Self::ShiftLeft { a, shift: _ } => return Type::GenericLowered(RawType::Integer(a.size, a.signed)),
			Self::ShiftRight { a, shift: _ } => return Type::GenericLowered(RawType::Integer(a.size, a.signed)),

			Self::CompEq { .. } => return Type::GenericLowered(RawType::Boolean),
			Self::CompNeg { .. } => return Type::GenericLowered(RawType::Boolean),
			Self::CompLt { .. } => return Type::GenericLowered(RawType::Boolean),
			Self::CompLe { .. } => return Type::GenericLowered(RawType::Boolean),
			Self::CompGt { .. } => return Type::GenericLowered(RawType::Boolean),
			Self::CompGe { .. } => return Type::GenericLowered(RawType::Boolean),

			Self::IntegerSignedConstant { raw: _, bitsize } => return Type::GenericLowered(RawType::Integer(*bitsize, true)),
			Self::IntegerUnsignedConstant { raw: _, bitsize } => return Type::GenericLowered(RawType::Integer(*bitsize, false)),
			Self::FloatUnsignedConstant { raw: _, size } => return Type::GenericLowered(RawType::Floating(*size, false)),
			Self::FloatSignedConstant { raw: _, size } => return Type::GenericLowered(RawType::Floating(*size, true)),
			Self::FixedSignedConstant { raw: _, number, fraction } => return Type::GenericLowered(RawType::FixedPoint(*number, *fraction, true)),
			Self::FixedUnsignedConstant { raw: _, number, fraction } => return Type::GenericLowered(RawType::FixedPoint(*number, *fraction, false)),
			Self::StaticStringConstant { raw: _ } => return Type::GenericLowered(RawType::Pointer),
			Self::StructInitializerConstant { struct_type, values: _ } => return Type::GenericLowered(struct_type.clone()),

			Self::Phi { choices } => {
				return choices[0].1.vtype.clone();
			},

			Self::Select { cond: _, if_val, else_val: _ } => return if_val.vtype.clone(),

			Self::Call { function, arguments: _ } => {
				let func = &ctx.functions[*function];

				return func.return_type.clone().unwrap();
			}
			
			Self::FieldPointer { .. } => return Type::GenericLowered(RawType::Pointer),
			Self::IndexPointer { .. } => return Type::GenericLowered(RawType::Pointer),

			Self::PointerAdd { .. } => return Type::GenericLowered(RawType::Pointer),
			Self::PointerSub { .. } => return Type::GenericLowered(RawType::Pointer), 

			Self::FuncArgumentGrab { ind: _, argtype } => argtype.clone(),

			_ => panic!("Tried using get_return_type on non returning type!")
		}
	}
}

impl Display for MIRInstruction {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::StackAlloc { alloc_size, t: _ } => writeln!(f, "stkalloc {}", *alloc_size)?,
			Self::Load { value } => writeln!(f, "load {}", value)?,
			Self::Store { variable, value } => writeln!(f, "store d{} s{}", variable, value)?,

			Self::DowncastInteger { val, size } => writeln!(f, "dintcast {} {}", val, size)?,
			Self::DowncastFloat { val, size } => writeln!(f, "dfcast {} {}", val, size)?,
			Self::UpcastInteger { val, size } => writeln!(f, "uintcast {} {}", val, size)?,
			Self::UpcastFloat { val, size } => writeln!(f, "ufcast {} {}", val, size)?,

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

			Self::FloatSignedConstant { raw, size } => writeln!(f, "constfs {} {}", raw, size)?,
			Self::FloatUnsignedConstant { raw, size } => writeln!(f, "constfu {} {}", raw, size)?,

			Self::FixedSignedConstant { raw, number, fraction } => writeln!(f, "constffs {} {} {}", raw, number, fraction)?,
			Self::FixedUnsignedConstant { raw, number, fraction } => writeln!(f, "constffu {} {} {}", raw, number, fraction)?,

			Self::StaticStringConstant { raw } => writeln!(f, "conststr {}", raw)?,

			Self::StructInitializerConstant { struct_type: _, values } => {
				writeln!(f, "conststructinitrz ")?;
			
				for v in values {
					write!(f, "{}", v)?;
				}
			}

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
