//! The nodes inside of the AstoIR HIR. 

use astoir_typing::complete::CompleteType;
use lexer::toks::{comp::ComparingOperator, math::MathOperator};

use crate::structs::StructTypeContainer;

pub enum HIRNode {
	VarDeclaration { variable: usize, var_type: CompleteType, default_val: Option<Box<HIRNode>> },
	StaticVariableDeclaration { variable: usize, var_type: CompleteType, default_val: Option<Box<HIRNode>> },

	VarAssigment { variable: usize, val: Box<HIRNode> },
	
	MathOperation { left:  Box<HIRNode>, right: Box<HIRNode>, operation: MathOperator, assignment: bool },

	VariableReference { index: usize },
	FunctionReference { index: usize },

	StructLRU { left: Box<HIRNode>, right: Box<HIRNode> },

	StructDeclaration { type_name: usize, container: StructTypeContainer, layout: bool },
	FunctionDeclaration { func_name: usize, arguments: Vec<(u64, CompleteType)>, return_type: Option<CompleteType>, body: Vec<Box<HIRNode>> },
	ShadowFunctionDeclaration { func_name: usize, arguments: Vec<(u64, CompleteType)>, return_type: Option<CompleteType> },

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