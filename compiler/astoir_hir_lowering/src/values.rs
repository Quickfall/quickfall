use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use astoir_hir::{ctx::{HIRBranchedContext, HIRContext, get_variable}, nodes::HIRNode, structs::StructLRUStep};
use compiler_errors::{IR_FIND_ELEMENT, IR_INVALID_NODE_TYPE, errs::{CompilerResult, ErrorKind, normal::CompilerError}, make_invalid_type_err};
use compiler_typing::tree::Type;

use crate::{bools::{lower_ast_boolean_condition, lower_ast_operator_condition}, func::lower_ast_function_call, literals::lower_ast_literal, math::lower_ast_math_operation, var::lower_ast_variable_reference};

pub(crate) fn lower_ast_lru_base(context: &HIRContext, curr_ctx: &HIRBranchedContext, node: Box<ASTTreeNode>, curr_steps: &mut Vec<StructLRUStep>, curr_type: &mut Option<Type>) -> CompilerResult<bool> {
	match node.kind {
		ASTTreeNodeKind::FunctionCall { func, args } => {
			let func_type;
			let ind: usize;

			if let Some(curr_type_val) = curr_type {
				let res = match curr_type_val.get_function(&context.type_storage, func.hash) {
					Ok(v) => v,
					Err(e) => return Err(CompilerError::from_base(e, &node.start, &node.end))
				};

				let abstract_func = res.1;

				let mut resolved_args = vec![];

				for arg in abstract_func.0 {
					resolved_args.push((arg.0, arg.1.resolve(curr_type_val)));
				}

				let resolved_ret;

				if abstract_func.1.is_some() {
					resolved_ret = Some(abstract_func.1.unwrap().resolve(curr_type_val));
				} else {
					resolved_ret = None;
				}

				func_type = (resolved_ret, resolved_args, func.val.clone());
				
				ind = res.0;

			} else {
				ind = match context.functions.get_index(func.hash) {
					Some(v) => v,
					None => return Err(CompilerError::from_ast(ErrorKind::Error, IR_FIND_ELEMENT!().to_string(), &node.start, &node.end))
				};

				func_type = context.functions.vals[ind].clone();
			}

			let mut hir_args = vec![];
			let mut iind = 0;

			for a in args {
				let lowered = lower_ast_value(context, curr_ctx, a)?;

				let lowered = match lowered.use_as(context, curr_ctx, func_type.1[iind].1.clone()) {
					Ok(v) => Box::new(v),
					Err(e) => return Err(CompilerError::from_base(e, &node.start, &node.end))
				};

				hir_args.push(lowered);

				iind += 1;
			}

			*curr_type = func_type.0.clone();
			
			curr_steps.push(StructLRUStep::FunctionCall { func: ind, args: hir_args });

			return Ok(true);
		},

		ASTTreeNodeKind::VariableReference(str) => {
			let var_type;
			let ind: usize;

			if let Some(curr_type_val) = curr_type {
				let res = match curr_type_val.get_field(&context.type_storage, str.hash) {
					Ok(v) => v,
					Err(e) => return Err(CompilerError::from_base(e, &node.start, &node.end))
				};

				ind = res.0;
				var_type = res.1.resolve(curr_type_val);

			} else {
				match get_variable(context, curr_ctx, str.hash) {
					Ok(v) => {
						var_type = v.1;
						ind = v.2;
					}
					Err(e) => return Err(CompilerError::from_base(e, &node.start, &node.end))
				};
			}

			curr_steps.push(StructLRUStep::VariableStep { variable: ind });
			*curr_type = Some(var_type);

			return Ok(true)
		},

		ASTTreeNodeKind::StructLRFunction { l, r } => {
			lower_ast_lru_base(context, curr_ctx, l, curr_steps, curr_type)?;
			lower_ast_lru_base(context, curr_ctx, r, curr_steps, curr_type)?;

			return Ok(true);
		},

		ASTTreeNodeKind::StructLRVariable { l, r } => {
			lower_ast_lru_base(context, curr_ctx, l, curr_steps, curr_type)?;
			lower_ast_lru_base(context, curr_ctx, r, curr_steps, curr_type)?;

			return Ok(true);
		}

		_ => return Err(CompilerError::from_ast(ErrorKind::Error, IR_INVALID_NODE_TYPE!().to_string(), &node.start, &node.end))

	}
}

pub fn lower_ast_lru(context: &HIRContext, curr_ctx: &HIRBranchedContext, node: Box<ASTTreeNode>) -> CompilerResult<Box<HIRNode>> {
	let mut steps: Vec<StructLRUStep> = vec![];
	let mut curr_type: Option<Type> = None;

	lower_ast_lru_base(context, curr_ctx, node, &mut steps, &mut curr_type)?;

	return Ok(Box::new(HIRNode::StructLRU { steps, last: curr_type.unwrap() }))
}

pub fn lower_ast_value(context: &HIRContext, curr_ctx: &HIRBranchedContext, node: Box<ASTTreeNode>) -> CompilerResult<Box<HIRNode>> {
	match node.kind {
		ASTTreeNodeKind::StructLRFunction { .. } | ASTTreeNodeKind::StructLRVariable { .. } => {
			return lower_ast_lru(context, curr_ctx, node);
		},

		ASTTreeNodeKind::MathResult { .. } => {
			return lower_ast_math_operation(context, curr_ctx, node, false)
		},

		ASTTreeNodeKind::OperatorBasedConditionMember { .. } => {
			return lower_ast_operator_condition(context, curr_ctx, node)
		},

		ASTTreeNodeKind::BooleanBasedConditionMember { .. } => {
			return lower_ast_boolean_condition(context, curr_ctx, node)
		},

		ASTTreeNodeKind::IntegerLit { .. } | ASTTreeNodeKind::StringLit(_) => {
			return lower_ast_literal(context, node);
		},

		ASTTreeNodeKind::FunctionCall { .. } => {
			return lower_ast_function_call(context, curr_ctx, node);
		}

		ASTTreeNodeKind::VariableReference(_) => {
			return lower_ast_variable_reference(context, curr_ctx, node, true)
		},

		_ => make_invalid_type_err!(node)
	}
}