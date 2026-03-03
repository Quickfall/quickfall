//! The nodes inside of the AstoIR HIR. 

use astoir_typing::complete::CompleteType;
use lexer::toks::math::MathOperator;

pub enum HIRNode {
	VarDeclaration { variable: usize, var_type: CompleteType, default_val: Option<Box<HIRNode>> },

	VarAssigment { variable: usize, val: Box<HIRNode> },
	
	MathOperation { left:  Box<HIRNode>, right: Box<HIRNode>, operation: MathOperator, assignment: bool },

	VariableRef { variable: usize },

	StructDeclaration { type_name: usize,  }

}