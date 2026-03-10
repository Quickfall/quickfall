//! The definitions for instructions within the MIR. 

use crate::vals::base::BaseHIRValue;

/// An instruction inside of the MIR.
pub enum MIRInstruction {
	StackAlloc { alloc_size: usize },
	Load { value: BaseHIRValue }, // TODO: change this to pointer
	Store { variable: BaseHIRValue, value: BaseHIRValue }, // TODO: change this to pointer

	// Number casting
	DowncastInteger { val: BaseHIRValue, size: usize }, // make size smaller
	UpcastInteger { val: BaseHIRValue, size: usize },  // make size bigger

	DowncastFloat { val: BaseHIRValue, size: usize }, 
	UpcastFloat { val: BaseHIRValue, size: usize }, 

	// Arithmetrics
	IntegerAdd { left: BaseHIRValue, right: BaseHIRValue }, 
	IntegerSub { left: BaseHIRValue, right: BaseHIRValue }, 
	IntegerMul { left: BaseHIRValue, right: BaseHIRValue }, 
	IntegerDiv { left: BaseHIRValue, right: BaseHIRValue },
	IntegerNeg { val: BaseHIRValue }, 
	
	FloatAdd { left: BaseHIRValue, right: BaseHIRValue }, 
	FloatSub { left: BaseHIRValue, right: BaseHIRValue }, 
	FloatMul { left: BaseHIRValue, right: BaseHIRValue }, 
	FloatDiv { left: BaseHIRValue, right: BaseHIRValue },
	FloatNeg { val: BaseHIRValue }, 

	// Bitwise (int typed)
	BitwiseAnd { a: BaseHIRValue, b: BaseHIRValue }, 
	BitwiseOr { a: BaseHIRValue, b: BaseHIRValue }, 
	BitwiseXor { a: BaseHIRValue, b: BaseHIRValue }, 
	BitwiseNot { val: BaseHIRValue }, 
	
	ShiftLeft { a: BaseHIRValue, shift: BaseHIRValue }, 
	ShiftRight { a: BaseHIRValue, shift: BaseHIRValue }, 

	// Comparaison / Logical
	CompEq { a: BaseHIRValue, b: BaseHIRValue }, 
	CompNeg { a: BaseHIRValue, b: BaseHIRValue }, 
	CompLt { a: BaseHIRValue, b: BaseHIRValue}, // <
	CompLe { a: BaseHIRValue, b: BaseHIRValue}, // <=
	CompGt { a: BaseHIRValue, b: BaseHIRValue }, // >
	CompGe { a: BaseHIRValue, b: BaseHIRValue}, // >=

	// Constants
	IntegerSignedConstant { raw: usize, bitsize: usize },
	IntegerUnsignedConstant { raw: usize, bitsize: usize }, 
	FloatSignedConstant { raw: usize, bitsize: usize }, 
	FloatUnsignedConstant { raw: usize, bitsize: usize }, 
	FixedSignedConstant { raw: usize, bitsize: usize }, 
	FixedUnsignedConstant { raw: usize, bitsize: usize }, 

	// Control
	Return { val: BaseHIRValue }, 
	UnconditionalBranch { branch: BaseHIRValue }, // TODO: swap to branch
	ConditionalBranch { cond: BaseHIRValue, if_branch: BaseHIRValue, else_branch: BaseHIRValue }, 
	Phi { choices: Vec<(BaseHIRValue, BaseHIRValue)> },
	Select { cond: BaseHIRValue, if_val: BaseHIRValue, else_val: BaseHIRValue },

	Call { function: BaseHIRValue, arguments: Vec<BaseHIRValue> },

	// Pointer utils

	FieldPointer { val: BaseHIRValue, field: usize },
	IndexPointer { val: BaseHIRValue, index: usize }, 
	PointerAdd { pointer: BaseHIRValue, right: BaseHIRValue }, 
	PointerSub { pointer: BaseHIRValue, right: BaseHIRValue }, 

	/// Indicates to the IR processor that this given value's era is finished and thus we drop the value
	MarkerEraDrop { value: BaseHIRValue },
}