//! The definitions for instructions within the MIR. 

use crate::vals::base::BaseMIRValue;

/// An instruction inside of the MIR.
pub enum MIRInstruction {
	StackAlloc { alloc_size: usize },
	Load { value: BaseMIRValue }, // TODO: change this to pointer
	Store { variable: BaseMIRValue, value: BaseMIRValue }, // TODO: change this to pointer

	// Number casting
	DowncastInteger { val: BaseMIRValue, size: usize }, // make size smaller
	UpcastInteger { val: BaseMIRValue, size: usize },  // make size bigger

	DowncastFloat { val: BaseMIRValue, size: usize }, 
	UpcastFloat { val: BaseMIRValue, size: usize }, 

	// Arithmetrics
	IntegerAdd { left: BaseMIRValue, right: BaseMIRValue }, 
	IntegerSub { left: BaseMIRValue, right: BaseMIRValue }, 
	IntegerMul { left: BaseMIRValue, right: BaseMIRValue }, 
	IntegerDiv { left: BaseMIRValue, right: BaseMIRValue },
	IntegerNeg { val: BaseMIRValue }, 
	
	FloatAdd { left: BaseMIRValue, right: BaseMIRValue }, 
	FloatSub { left: BaseMIRValue, right: BaseMIRValue }, 
	FloatMul { left: BaseMIRValue, right: BaseMIRValue }, 
	FloatDiv { left: BaseMIRValue, right: BaseMIRValue },
	FloatNeg { val: BaseMIRValue }, 

	// Bitwise (int typed)
	BitwiseAnd { a: BaseMIRValue, b: BaseMIRValue }, 
	BitwiseOr { a: BaseMIRValue, b: BaseMIRValue }, 
	BitwiseXor { a: BaseMIRValue, b: BaseMIRValue }, 
	BitwiseNot { val: BaseMIRValue }, 
	
	ShiftLeft { a: BaseMIRValue, shift: BaseMIRValue }, 
	ShiftRight { a: BaseMIRValue, shift: BaseMIRValue }, 

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
	ConditionalBranch { cond: BaseMIRValue, if_branch: BaseMIRValue, else_branch: BaseMIRValue }, 
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