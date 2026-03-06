use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use astoir_hir::{ctx::{HIRBranchedContext, HIRContext}, nodes::HIRNode};
use compiler_errors::{IR_INVALID_NODE_TYPE, errs::{CompilerResult, ErrorKind, normal::CompilerError}};

use crate::{func::lower_ast_function_call, math::lower_ast_math_operation, values::lower_ast_value, var::lower_ast_variable_declaration};

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
 		}
		
		_ => return Err(CompilerError::from_ast(ErrorKind::Error, IR_INVALID_NODE_TYPE!().to_string(), &node.start, &node.end))
	}
}

pub fn lower_ast_body(context: &HIRContext, curr_ctx: &mut HIRBranchedContext, nodes: Vec<Box<ASTTreeNode>>) -> CompilerResult<Vec<Box<HIRNode>>> {
	let mut hir_nodes = vec![];

	for n in nodes {
		hir_nodes.push(lower_ast_body_node(context, curr_ctx, n)?);
	}

	return Ok(hir_nodes);
}