//! The nodes inside of the AstoIR HIR. 

use compiler_errors::{IR_TRANSMUTATION, errs::{BaseResult, base::BaseError}};
use compiler_typing::{references::TypeReference, storage::{BOOLEAN_TYPE, POINTER_TYPE}, structs::RawStructTypeContainer, tree::Type};
use lexer::toks::{comp::ComparingOperator, math::MathOperator};

use crate::{ctx::{HIRBranchedContext, HIRContext}, structs::{HIRIfBranch, StructLRUStep}};

#[derive(Debug, Clone)]
pub enum HIRNode {
	CastValue { intentional: bool, value: Box<HIRNode>, old_type: Type, new_type: Type }, 

	VarDeclaration { variable: usize, var_type: Type, default_val: Option<Box<HIRNode>> },
	StaticVariableDeclaration { variable: usize, var_type: Type, default_val: Option<Box<HIRNode>> },

	VarAssigment { variable: usize, val: Box<HIRNode> },
	
	MathOperation { left:  Box<HIRNode>, right: Box<HIRNode>, operation: MathOperator, assignment: bool },

	VariableReference { index: usize, is_static: bool },
	FunctionReference { index: usize },

	StructLRU { steps: Vec<StructLRUStep>, last: Type },

	StructDeclaration { type_name: usize, container: RawStructTypeContainer, layout: bool },
	StructFunctionDeclaration { func_name: usize, arguments: Vec<(u64, TypeReference)>, return_type: Option<TypeReference>, body: Vec<Box<HIRNode>>, ctx: HIRBranchedContext, requires_this: bool },
	
	FunctionDeclaration { func_name: usize, arguments: Vec<(u64, Type)>, return_type: Option<Type>, body: Vec<Box<HIRNode>>, ctx: HIRBranchedContext, requires_this: bool },
	
	ShadowFunctionDeclaration { func_name: usize, arguments: Vec<(u64, Type)>, return_type: Option<Type> },

	FunctionCall { func_name: usize, arguments: Vec<Box<HIRNode>> },

	WhileBlock { condition: Box<HIRNode>, body: Vec<Box<HIRNode>> },
	ForBlock { initial_state: Box<HIRNode>, condition: Box<HIRNode>, incrementation: Box<HIRNode>, body: Vec<Box<HIRNode>> },

	IfStatement { branches: Vec<HIRIfBranch> },

	ReturnStatement { value: Option<Box<HIRNode>> },

	IntegerLiteral { value: i128, int_type: Type }, 
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
	
	pub fn use_as(self, context: &HIRContext, curr_ctx: &HIRBranchedContext, t: Type) -> BaseResult<HIRNode> {
		let self_type = match self.get_node_type(context, curr_ctx) {
			Some(v) => v,
			None => return Err(BaseError::err("This needs to be a value".to_string()))
		};

		if self_type == t {
			return Ok(self);
		}

		if self_type.can_transmute(&t, &context.type_storage) {
			match &self {
				HIRNode::IntegerLiteral { value, int_type: _ } => {
					return Ok(HIRNode::IntegerLiteral { value: *value, int_type: t });
				},

				_ => {
					return Ok(HIRNode::CastValue { intentional: false, old_type: self_type.clone(), value: Box::new(self), new_type: t });
				}
			}
		}

		return Err(BaseError::err(IR_TRANSMUTATION!().to_string()))
	}	

	pub fn get_node_type(&self, context: &HIRContext, curr_ctx: &HIRBranchedContext) -> Option<Type> {
		match self {
			HIRNode::VariableReference { index, is_static } => {
				if *is_static {
					return Some(context.static_variables.vals[*index].clone());
				}

				return Some(curr_ctx.variables[*index].variable_type.clone());
			},

			HIRNode::IntegerLiteral { value: _, int_type } => {
				return Some(int_type.clone());
			},

			HIRNode::StringLiteral { value: _ } => {
				let ind = match context.type_storage.types.get_index(POINTER_TYPE) {
					Some(v) => v,
					None => return None
				};

				return Some(Type::Generic(ind, vec![], vec![]))
			},

			HIRNode::StructLRU { steps: _, last } => {
				return Some(last.clone())
			},

			HIRNode::MathOperation { left, right: _, operation: _, assignment: _ } => {
				return left.get_node_type(context, curr_ctx)
			},

			HIRNode::BooleanOperator { .. } | HIRNode::BooleanCondition { .. } => {
				let t = match context.type_storage.types.get_index(BOOLEAN_TYPE) {
					Some(v) => v,
					None => return None
				};

				return Some(Type::Generic(t, vec![], vec![]))
			},

			HIRNode::FunctionCall { func_name, arguments: _ } => {
				let f = context.functions.vals[*func_name].0.clone();

				return f;
			},

			_ => return None
		}
	}
}