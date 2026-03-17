//! The nodes inside of the AstoIR HIR. 

use astoir_typing::{complete::{ComplexType, ConcreteType}, hashes::{BOOLEAN_TYPE, STATIC_STR}, structs::StructTypeContainer};
use compiler_errors::errs::{BaseResult, base::BaseError};
use lexer::toks::{comp::ComparingOperator, math::MathOperator};

use crate::{ctx::{HIRBranchedContext, HIRContext}, structs::{HIRIfBranch, StructLRUStep}};

#[derive(Debug, Clone)]
pub enum HIRNode {
	VarDeclaration { variable: usize, var_type: ComplexType, default_val: Option<Box<HIRNode>> },
	StaticVariableDeclaration { variable: usize, var_type: ComplexType, default_val: Option<Box<HIRNode>> },

	VarAssigment { variable: usize, val: Box<HIRNode> },
	
	MathOperation { left:  Box<HIRNode>, right: Box<HIRNode>, operation: MathOperator, assignment: bool },

	VariableReference { index: usize, is_static: bool },
	FunctionReference { index: usize },

	StructLRU { steps: Vec<StructLRUStep>, last: ComplexType },

	StructDeclaration { type_name: usize, container: StructTypeContainer, layout: bool },
	FunctionDeclaration { func_name: usize, arguments: Vec<(u64, ComplexType)>, return_type: Option<ComplexType>, body: Vec<Box<HIRNode>>, ctx: HIRBranchedContext, requires_this: bool },
	ShadowFunctionDeclaration { func_name: usize, arguments: Vec<(u64, ComplexType)>, return_type: Option<ComplexType> },

	FunctionCall { func_name: usize, arguments: Vec<Box<HIRNode>> },

	WhileBlock { condition: Box<HIRNode>, body: Vec<Box<HIRNode>> },
	ForBlock { initial_state: Box<HIRNode>, condition: Box<HIRNode>, incrementation: Box<HIRNode>, body: Vec<Box<HIRNode>> },

	IfStatement { branches: Vec<HIRIfBranch> },

	ReturnStatement { value: Option<Box<HIRNode>> },

	IntegerLiteral { value: i128, int_type: usize }, 
	StringLiteral { value: String }, 

	BooleanOperator { left: Box<HIRNode>, right: Box<HIRNode>, operator: ComparingOperator },
	BooleanCondition { value: Box<HIRNode>, negation: bool }
}

impl HIRNode {
	pub fn is_variable_reference(&self) -> bool {
		if let HIRNode::VariableReference { .. } = self {
			return true;
		}

		return false;
	}

	pub fn as_variable_reference(&self) -> BaseResult<(usize, bool)> {
		if let HIRNode::VariableReference { index, is_static } = self {
			return Ok((*index, *is_static))
		}

		return Err(BaseError::err("Tried using as_variable_reference on a non var ref".to_string()))
	}
	
	pub fn get_node_type(&self, context: &HIRContext, curr_ctx: &HIRBranchedContext) -> Option<ComplexType> {
		match self {
			HIRNode::VariableReference { index, is_static } => {
				if *is_static {
					return Some(context.static_variables.vals[*index].clone());
				}

				return Some(curr_ctx.variables[*index].variable_type.clone());
			},

			HIRNode::IntegerLiteral { value: _, int_type } => {
				let t = context.type_storage.types[*int_type].clone();

				return Some(ComplexType::Concrete(ConcreteType { base: t, pointer: false, pointer_array: false, type_params: vec![], size_params: vec![] }))
			},

			HIRNode::StringLiteral { value: _ } => {
				let t = match context.type_storage.get_type(STATIC_STR) {
					Ok(v) => v,
					Err(_) => return None
				};

				return Some(ComplexType::Concrete(ConcreteType { base: t.1.clone(), pointer: false, pointer_array: false, type_params: vec![], size_params: vec![] }))
			},

			HIRNode::StructLRU { steps: _, last } => {
				return Some(last.clone())
			},

			HIRNode::MathOperation { left, right: _, operation: _, assignment: _ } => {
				return left.get_node_type(context, curr_ctx)
			},

			HIRNode::BooleanOperator { .. } | HIRNode::BooleanCondition { .. } => {
				let t = match context.type_storage.get_type(BOOLEAN_TYPE) {
					Ok(v) => v,
					Err(_) => return None
				};

				return Some(ComplexType::Concrete(ConcreteType { base: t.1.clone(), pointer: false, pointer_array: false, type_params: vec![], size_params: vec![] }))
			},

			HIRNode::FunctionCall { func_name, arguments: _ } => {
				let f = context.functions.vals[*func_name].0.clone();

				return f;
			},

			_ => return None
		}
	}
}