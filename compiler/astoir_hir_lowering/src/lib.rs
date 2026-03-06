use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use astoir_hir::{ctx::{HIRBranchedContext, HIRContext}, nodes::HIRNode};
use compiler_errors::{IR_INVALID_NODE_TYPE, errs::{CompilerResult, ErrorKind, normal::CompilerError}};

use crate::{control::{lower_ast_for_block, lower_ast_if_statement, lower_ast_while_block}, func::lower_ast_function_call, math::lower_ast_math_operation, values::lower_ast_value, var::lower_ast_variable_declaration};

pub mod literals;
pub mod var;
pub mod types;
pub mod values;
pub mod func;
pub mod math;
pub mod bools;
pub mod control;

pub fn lower_ast_body_node(context: &HIRContext, curr_ctx: &mut HIRBranchedContext, node: Box<ASTTreeNode>) -> CompilerResult<Box<HIRNode>> {
	match node.kind.clone() {
		ASTTreeNodeKind::VarDeclaration { .. } => return lower_ast_variable_declaration(context, curr_ctx, node),
		ASTTreeNodeKind::FunctionCall { .. } => return lower_ast_function_call(context, curr_ctx, node),
		ASTTreeNodeKind::MathResult { .. } => return lower_ast_math_operation(context, curr_ctx, node, true),

		ASTTreeNodeKind::ReturnStatement { val } => {
			let v;

			if val.is_none() {
				v = None;
			} else {
				v = Some(lower_ast_value(context, curr_ctx, node)?)
			}

			return Ok(Box::new(HIRNode::ReturnStatement { value: v }))
 		},

		ASTTreeNodeKind::ForBlock { .. } => return lower_ast_for_block(context, curr_ctx, node),
		ASTTreeNodeKind::WhileBlock { .. } => return lower_ast_while_block(context, curr_ctx, node),
		ASTTreeNodeKind::IfStatement { .. } => return lower_ast_if_statement(context, curr_ctx, node),
		
		_ => return Err(CompilerError::from_ast(ErrorKind::Error, IR_INVALID_NODE_TYPE!().to_string(), &node.start, &node.end))
	}
}

pub fn lower_ast_body(context: &HIRContext, curr_ctx: &mut HIRBranchedContext, nodes: Vec<Box<ASTTreeNode>>, introduce_era: bool) -> CompilerResult<Vec<Box<HIRNode>>> {
	let mut hir_nodes = vec![];

	let branch;

	if introduce_era {
		branch = curr_ctx.start_branch();
	} else {
		branch = 0;
	}

	for n in nodes {
		hir_nodes.push(lower_ast_body_node(context, curr_ctx, n)?);
	}

	if introduce_era {
		curr_ctx.end_branch(branch);
	}

	return Ok(hir_nodes);
}