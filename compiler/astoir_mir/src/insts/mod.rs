//! The definitions for instructions within the MIR. 

use crate::{blocks::{MIRBlock, refer::MIRBlockReference}, vals::{base::{BaseMIRValue, BaseValueType}, float::MIRFloatValue, int::MIRIntValue, ptr::MIRPointerValue}};

pub mod val;

/// An instruction inside of the MIR.
#[derive(Clone)]
pub enum MIRInstruction {
	StackAlloc { alloc_size: usize, t: BaseValueType },
	Load { value: MIRPointerValue }, // TODO: change this to pointer
	Store { variable: MIRPointerValue, value: BaseMIRValue }, // TODO: change this to pointer

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
	IntegerSignedConstant { raw: usize, bitsize: usize },
	IntegerUnsignedConstant { raw: usize, bitsize: usize }, 
	FloatSignedConstant { raw: usize, bitsize: usize }, 
	FloatUnsignedConstant { raw: usize, bitsize: usize }, 
	FixedSignedConstant { raw: usize, bitsize: usize }, 
	FixedUnsignedConstant { raw: usize, bitsize: usize }, 

	// Control
	Return { val: BaseMIRValue }, 
	UnconditionalBranch { branch: MIRBlockReference },
	ConditionalBranch { cond: MIRIntValue, if_branch: MIRBlockReference, else_branch: MIRBlockReference }, 
	Phi { choices: Vec<(MIRBlockReference, BaseMIRValue)> },
	Select { cond: BaseMIRValue, if_val: BaseMIRValue, else_val: BaseMIRValue },

	Call { function: BaseMIRValue, arguments: Vec<BaseMIRValue> },

	// Pointer utils

	FieldPointer { val: MIRPointerValue, field: usize },
	IndexPointer { val: MIRPointerValue, index: usize }, 
	PointerAdd { pointer: MIRPointerValue, right: MIRIntValue }, 
	PointerSub { pointer: MIRPointerValue, right: MIRIntValue }, 

	/// Indicates to the IR processor that this given value's era is finished and thus we drop the value
	MarkerEraDrop { value: BaseMIRValue },
}

impl MIRInstruction {
	pub fn has_return(&self) -> bool {
		match self {
			Self::MarkerEraDrop { .. } | Self::UnconditionalBranch { .. } | Self::ConditionalBranch { .. } | Self::Return { .. } => {
				return false;
			},

			_ => true
		}
	}

	pub fn get_return_type(&self) -> BaseValueType {
		match self {
			Self::StackAlloc { .. } => return BaseValueType::PointerValue,
			Self::Load { .. } => return BaseValueType::AnyValue,

			Self::DowncastInteger { val: _, size } => return BaseValueType::IntValue(*size),
			Self::UpcastInteger { val: _, size } => return BaseValueType::IntValue(*size),

			Self::DowncastFloat { val: _, size } => return BaseValueType::FloatValue(*size),
			Self::UpcastFloat { val: _, size } => return BaseValueType::IntValue(*size),

			Self::IntegerAdd { signed: _, left, right: _ } => return BaseValueType::IntValue(left.size), 
			Self::IntegerSub { signed: _, left, right: _ } => return BaseValueType::IntValue(left.size), 
			Self::IntegerMul { signed: _, left, right: _ } => return BaseValueType::IntValue(left.size), 
			Self::IntegerDiv { signed: _, left, right: _ } => return BaseValueType::IntValue(left.size), 
			Self::IntegerMod { signed: _, left, right: _ } => return BaseValueType::IntValue(left.size), 
			Self::IntegerNeg { val } => return BaseValueType::IntValue(val.size),

			Self::FloatAdd { signed: _, left, right: _ } => return BaseValueType::FloatValue(left.size),
			Self::FloatSub { signed: _, left, right: _ } => return BaseValueType::FloatValue(left.size),
			Self::FloatMul { signed: _, left, right: _ } => return BaseValueType::FloatValue(left.size),
			Self::FloatDiv { signed: _, left, right: _ } => return BaseValueType::FloatValue(left.size),
			Self::FloatNeg { val } => return BaseValueType::FloatValue(val.size),

			Self::BitwiseAnd { a, b: _ } => return BaseValueType::IntValue(a.size),
			Self::BitwiseOr { a, b: _ } => return BaseValueType::IntValue(a.size),
			Self::BitwiseXor { a, b: _ } => return BaseValueType::IntValue(a.size),
			Self::BitwiseNot { val } => return BaseValueType::IntValue(val.size),

			Self::ShiftLeft { a, shift: _ } => return BaseValueType::IntValue(a.size),
			Self::ShiftRight { a, shift: _ } => return BaseValueType::IntValue(a.size),

			Self::CompEq { .. } => return BaseValueType::IntValue(1),
			Self::CompNeg { .. } => return BaseValueType::IntValue(1),
			Self::CompLt { .. } => return BaseValueType::IntValue(1),
			Self::CompLe { .. } => return BaseValueType::IntValue(1),
			Self::CompGt { .. } => return BaseValueType::IntValue(1),
			Self::CompGe { .. } => return BaseValueType::IntValue(1),

			Self::IntegerSignedConstant { raw: _, bitsize } => return BaseValueType::IntValue(*bitsize),
			Self::IntegerUnsignedConstant { raw: _, bitsize } => return BaseValueType::IntValue(*bitsize),
			Self::FloatUnsignedConstant { raw: _, bitsize } => return BaseValueType::FloatValue(*bitsize),
			Self::FloatSignedConstant { raw: _, bitsize } => return BaseValueType::FloatValue(*bitsize),
			Self::FixedSignedConstant { raw: _, bitsize } => return BaseValueType::IntValue(*bitsize),
			Self::FixedUnsignedConstant { raw: _, bitsize } => return BaseValueType::IntValue(*bitsize),
 
			Self::Phi { choices } => {
				return choices[0].1.vtype.clone();
			},

			Self::Select { cond: _, if_val, else_val: _ } => return if_val.vtype.clone(),

			Self::Call { .. } => return BaseValueType::AnyValue,
			
			Self::FieldPointer { .. } => return BaseValueType::PointerValue,
			Self::IndexPointer { .. } => return BaseValueType::PointerValue,

			Self::PointerAdd { .. } => return BaseValueType::PointerValue,
			Self::PointerSub { .. } => return BaseValueType::PointerValue, 

			_ => panic!("Tried using get_return_type on non returning type!")
		}
	}

}