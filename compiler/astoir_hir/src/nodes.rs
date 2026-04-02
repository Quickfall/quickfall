//! The nodes inside of the AstoIR HIR. 

use std::env::var;

use compiler_errors::{IR_TRANSMUTATION, errs::{BaseResult, base::BaseError}};
use compiler_typing::{references::TypeReference, storage::{BOOLEAN_TYPE, STATIC_STR}, structs::RawStructTypeContainer, transmutation::array::can_transmute_inner, tree::Type};
use compiler_utils::Position;
use diagnostics::{DiagnosticResult, DiagnosticSpanOrigin, builders::{make_diff_type, make_expected_simple_error}};
use lexer::toks::{comp::ComparingOperator, math::MathOperator};

use crate::{ctx::{HIRBranchedContext, HIRContext}, structs::{HIRIfBranch, StructLRUStep}};

#[derive(Debug, Clone)]
pub struct HIRNode {	
	pub kind: HIRNodeKind, 
	pub start: Position,
	pub end: Position
}

impl HIRNode {
	pub fn new(kind: HIRNodeKind, start: &Position, end: &Position) -> Self {
		HIRNode { kind, start: start.clone(), end: end.clone() }
	}
	
	pub fn with(&self, kind: HIRNodeKind) -> Self {
		HIRNode { kind, start: self.start.clone(), end: self.end.clone() }
	}
}

#[derive(Debug, Clone)]
pub enum HIRNodeKind {
	CastValue { intentional: bool, value: Box<HIRNode>, old_type: Type, new_type: Type }, 

	VarDeclaration { variable: usize, var_type: Type, default_val: Option<Box<HIRNode>> },
	StaticVariableDeclaration { variable: usize, var_type: Type, default_val: Option<Box<HIRNode>> },

	VarAssigment { variable: usize, val: Box<HIRNode> },
	
	MathOperation { left:  Box<HIRNode>, right: Box<HIRNode>, operation: MathOperator, assignment: bool },

	VariableReference { index: usize, is_static: bool },
	FunctionReference { index: usize },

	PointerGrab { val: Box<HIRNode> },
	ReferenceGrab { val: Box<HIRNode> },

	StructLRU { steps: Vec<StructLRUStep>, last: Type },

	StructDeclaration { type_name: usize, container: RawStructTypeContainer, layout: bool },
	StructFunctionDeclaration { func_name: usize, arguments: Vec<(u64, TypeReference)>, return_type: Option<TypeReference>, body: Vec<Box<HIRNode>>, ctx: HIRBranchedContext, requires_this: bool },
	
	ArrayVariableInitializerValue { vals: Vec<Box<HIRNode>> },
	ArrayVariableInitializerValueSameValue { size: usize, val: Box<HIRNode> },

	ArrayIndexAccess { val: Box<HIRNode>, index: Box<HIRNode> },
	ArrayIndexModify { array: Box<HIRNode>, index: Box<HIRNode>, new_val : Box<HIRNode> },

	StructVariableInitializerValue { t: Type, fields: Vec<Box<HIRNode>> },

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
		if let HIRNodeKind::VariableReference { .. } = self.kind {
			return true;
		}

		return false;
	}

	pub fn get_variable_represent(&self) -> BaseResult<(usize, bool)> {
		match &self.kind {
			HIRNodeKind::VariableReference { index, is_static} => return Ok((*index, *is_static)),
			HIRNodeKind::ArrayIndexAccess { val, index: _ } => return val.get_variable_represent(),

			_ => return Err(BaseError::err("Used get_variable_represent on a non representing var".to_string()))
		}
	}

	pub fn is_variable_representative(&self) -> bool {
		if let HIRNodeKind::ArrayIndexAccess { .. } = self.kind {
			return true;
		}

		if let HIRNodeKind::VariableReference { .. } = self.kind {
			return true;
		}

		return false;
	}

	pub fn as_variable_reference(&self) -> (usize, bool) {
		if let HIRNodeKind::VariableReference { index, is_static } = &self.kind {
			return (*index, *is_static)
		}

		panic!("Tried using as_variable_reference on a non var ref")
	}
	
	pub fn use_as<K: DiagnosticSpanOrigin>(&self, context: &HIRContext, curr_ctx: &HIRBranchedContext, t: Type, origin: &K, var_origin: Option<&K>) -> Result<HIRNode, ()> {
		let self_type = match self.get_node_type(context, curr_ctx) {
			Some(v) => v,
			_ => return Err(())
		};

		if self_type == t {
			return Ok(self.clone());
		}

		if self_type.can_transmute(&t, &context.type_storage) {
			match &self.kind {
				HIRNodeKind::IntegerLiteral { value, int_type: _ } => {
					return Ok(self.with(HIRNodeKind::IntegerLiteral { value: *value, int_type: t }));
				},

				HIRNodeKind::ArrayVariableInitializerValue { vals } => {
					if can_transmute_inner(&self_type, &t, &context.type_storage) {
						let mut new_vals = vec![];
						let inner = t.get_inner_type();

						for val in vals {
							new_vals.push(Box::new(val.use_as(context, curr_ctx, *inner.clone(), origin, var_origin)?));
						}

						return Ok(self.with(HIRNodeKind::ArrayVariableInitializerValue { vals: new_vals }))
					}
				},

				HIRNodeKind::ArrayVariableInitializerValueSameValue { size, val } => {
					if can_transmute_inner(&self_type, &t, &context.type_storage) {
						let new_val = Box::new(val.use_as(context, curr_ctx, *t.get_inner_type(), origin, var_origin)?);

						return Ok(self.with(HIRNodeKind::ArrayVariableInitializerValueSameValue { size: *size, val: new_val }))		
					}
				},

				_ => {
					return Ok(self.with(HIRNodeKind::CastValue { intentional: false, old_type: self_type.clone(), value: Box::new(self.clone()), new_type: t }));
				}
			}
		}

		return Err(())
	}	

	pub fn get_node_type(&self, context: &HIRContext, curr_ctx: &HIRBranchedContext) -> Option<Type> {
		match &self.kind {
			HIRNodeKind::VariableReference { index, is_static } => {
				if *is_static {
					return Some(context.static_variables.vals[*index].clone());
				}

				return Some(curr_ctx.variables[*index].variable_type.clone());
			},

			HIRNodeKind::PointerGrab { val } => {
				return Some(Type::Pointer(false, Box::new(val.get_node_type(context, curr_ctx).unwrap())));
			},

			HIRNodeKind::ReferenceGrab { val } => {
				return Some(Type::Reference(Box::new(val.get_node_type(context, curr_ctx).unwrap())))
			}

			HIRNodeKind::ArrayIndexAccess { val, index: _ } => {
				let t = val.get_node_type(context, curr_ctx).unwrap();

				return Some(*t.get_inner_type())
			}

			HIRNodeKind::IntegerLiteral { value: _, int_type } => {
				return Some(int_type.clone());
			},

			HIRNodeKind::StringLiteral { value: _ } => {
				let ind = match context.type_storage.types.get_index(STATIC_STR) {
					Some(v) => v,
					None => return None
				};

				return Some(Type::Generic(ind, vec![], vec![]))
			},

			HIRNodeKind::ArrayVariableInitializerValue { vals } => return Some(Type::Array(vals.len(), Box::new(vals[0].get_node_type(context, curr_ctx).unwrap()))),
			HIRNodeKind::ArrayVariableInitializerValueSameValue { size, val } => return Some(Type::Array(*size, Box::new(val.get_node_type(context, curr_ctx).unwrap()))),

			HIRNodeKind::StructLRU { steps: _, last } => {
				return Some(last.clone())
			},

			HIRNodeKind::MathOperation { left, right: _, operation: _, assignment: _ } => {
				return left.get_node_type(context, curr_ctx)
			},

			HIRNodeKind::BooleanOperator { .. } | HIRNodeKind::BooleanCondition { .. } => {
				let t = match context.type_storage.types.get_index(BOOLEAN_TYPE) {
					Some(v) => v,
					None => return None
				};

				return Some(Type::Generic(t, vec![], vec![]))
			},

			HIRNodeKind::StructVariableInitializerValue { t, fields: _ } => {
				return Some(t.clone())
			}

			HIRNodeKind::FunctionCall { func_name, arguments: _ } => {
				let f = context.functions.vals[*func_name].0.clone();

				return f;
			},

			_ => return None
		}
	}
}