//! The nodes inside of the AstoIR HIR. 

use astoir_typing::{complete::ComplexType, structs::StructTypeContainer};
use lexer::toks::{comp::ComparingOperator, math::MathOperator};

use crate::structs::{StructLRUStep};

pub enum HIRNode {
	VarDeclaration { variable: usize, var_type: ComplexType, default_val: Option<Box<HIRNode>> },
	StaticVariableDeclaration { variable: usize, var_type: ComplexType, default_val: Option<Box<HIRNode>> },

	VarAssigment { variable: usize, val: Box<HIRNode> },
	
	MathOperation { left:  Box<HIRNode>, right: Box<HIRNode>, operation: MathOperator, assignment: bool },

	VariableReference { index: usize },
	FunctionReference { index: usize },

	StructLRU { steps: Vec<StructLRUStep> },

	StructDeclaration { type_name: usize, container: StructTypeContainer, layout: bool },
	FunctionDeclaration { func_name: usize, arguments: Vec<(u64, ComplexType)>, return_type: Option<ComplexType>, body: Vec<Box<HIRNode>> },
	ShadowFunctionDeclaration { func_name: usize, arguments: Vec<(u64, ComplexType)>, return_type: Option<ComplexType> },

	FunctionCall { func_name: usize, arguments: Vec<Box<HIRNode>> },

	WhileBlock { condition: Box<HIRNode>, body: Vec<Box<HIRNode>> },
	ForBlock { initial_state: Box<HIRNode>, condition: Box<HIRNode>, incrementation: Box<HIRNode>, body: Vec<Box<HIRNode>> },

	IfStatement { condition: Box<HIRNode>, body: Vec<Box<HIRNode>>, branches: Vec<Box<HIRNode>>, depth: usize },
	IfElseStatement { condition: Box<HIRNode>, body: Vec<Box<HIRNode>> },
	ElseStatement { body: Vec<Box<HIRNode>> },

	ReturnStatement { value: Option<Box<HIRNode>> },

	IntegerLiteral { value: i128, int_type: usize }, 
	StringLiteral { value: String }, 

	BooleanOperator { left: Box<HIRNode>, right: Box<HIRNode>, operator: ComparingOperator },
	BooleanCondition { value: Box<HIRNode>, negation: bool }
}