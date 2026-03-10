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
	
	FloatAdd { left: BaseHIRValue, right: BaseHIRValue }, 
	FloatSub { left: BaseHIRValue, right: BaseHIRValue }, 
	FloatMul { left: BaseHIRValue, right: BaseHIRValue }, 
	FloatDiv { left: BaseHIRValue, right: BaseHIRValue },

	// Control
	Return { val: BaseHIRValue }, 
	UnconditionalBranch { branch: BaseHIRValue }, // TODO: swap to branch
	ConditionalBranch { cond: BaseHIRValue, if_branch: BaseHIRValue, else_branch: BaseHIRValue }, 
	Phi { choices: Vec<(BaseHIRValue, BaseHIRValue)> },
	Select { cond: BaseHIRValue, if_val: BaseHIRValue, else_val: BaseHIRValue },

	Call { function: BaseHIRValue, arguments: Vec<BaseHIRValue> },

	/// Indicates to the IR processor that this given value's era is finished and thus we drop the value
	MarkerEraDrop { value: BaseHIRValue },
}