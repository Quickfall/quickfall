//! The definitions for instructions within the MIR. 

use crate::vals::{base::BaseMIRValue, float::MIRFloatValue, int::MIRIntValue};

/// An instruction inside of the MIR.
pub enum MIRInstruction {
	StackAlloc { alloc_size: usize },
	Load { value: BaseMIRValue }, // TODO: change this to pointer
	Store { variable: BaseMIRValue, value: BaseMIRValue }, // TODO: change this to pointer

	// Number casting
	DowncastInteger { val: MIRIntValue, size: usize }, // make size smaller
	UpcastInteger { val: MIRIntValue, size: usize },  // make size bigger

	DowncastFloat { val: MIRFloatValue, size: usize }, 
	UpcastFloat { val: BaseMIRValue, size: usize }, 

	// Arithmetrics
	IntegerAdd { left: MIRIntValue, right: MIRIntValue }, 
	IntegerSub { left: MIRIntValue, right: MIRIntValue }, 
	IntegerMul { left: MIRIntValue, right: MIRIntValue }, 
	IntegerDiv { left: MIRIntValue, right: MIRIntValue },
	IntegerNeg { val: MIRIntValue }, 
	
	FloatAdd { left: MIRFloatValue, right: MIRFloatValue }, 
	FloatSub { left: MIRFloatValue, right: MIRFloatValue }, 
	FloatMul { left: MIRFloatValue, right: MIRFloatValue }, 
	FloatDiv { left: MIRFloatValue, right: MIRFloatValue },
	FloatNeg { val: MIRFloatValue }, 

	// Bitwise (int typed)
	BitwiseAnd { a: MIRIntValue, b: MIRIntValue }, 
	BitwiseOr { a: MIRIntValue, b: MIRIntValue }, 
	BitwiseXor { a: MIRIntValue, b: MIRIntValue }, 
	BitwiseNot { val: MIRIntValue }, 
	
	ShiftLeft { a: MIRIntValue, shift: MIRIntValue }, 
	ShiftRight { a: MIRIntValue, shift: MIRIntValue }, 

	// Comparaison / Logical
	CompEq { a: BaseMIRValue, b: BaseMIRValue }, 
	CompNeg { a: BaseMIRValue, b: BaseMIRValue }, 
	CompLt { a: BaseMIRValue, b: BaseMIRValue}, // <
	CompLe { a: BaseMIRValue, b: BaseMIRValue}, // <=
	CompGt { a: BaseMIRValue, b: BaseMIRValue }, // >
	CompGe { a: BaseMIRValue, b: BaseMIRValue}, // >=

	// Constants
	IntegerSignedConstant { raw: usize, bitsize: usize },
	IntegerUnsignedConstant { raw: usize, bitsize: usize }, 
	FloatSignedConstant { raw: usize, bitsize: usize }, 
	FloatUnsignedConstant { raw: usize, bitsize: usize }, 
	FixedSignedConstant { raw: usize, bitsize: usize }, 
	FixedUnsignedConstant { raw: usize, bitsize: usize }, 

	// Control
	Return { val: BaseMIRValue }, 
	UnconditionalBranch { branch: BaseMIRValue }, // TODO: swap to branch
	ConditionalBranch { cond: MIRIntValue, if_branch: BaseMIRValue, else_branch: BaseMIRValue }, 
	Phi { choices: Vec<(BaseMIRValue, BaseMIRValue)> },
	Select { cond: BaseMIRValue, if_val: BaseMIRValue, else_val: BaseMIRValue },

	Call { function: BaseMIRValue, arguments: Vec<BaseMIRValue> },

	// Pointer utils

	FieldPointer { val: BaseMIRValue, field: usize },
	IndexPointer { val: BaseMIRValue, index: usize }, 
	PointerAdd { pointer: BaseMIRValue, right: BaseMIRValue }, 
	PointerSub { pointer: BaseMIRValue, right: BaseMIRValue }, 

	/// Indicates to the IR processor that this given value's era is finished and thus we drop the value
	MarkerEraDrop { value: BaseMIRValue },
}