use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use astoir_hir::{ctx::{HIRBranchedContext, HIRContext}, nodes::HIRNode};
use compiler_errors::{IR_FIND_ELEMENT, IR_INVALID_NODE_TYPE, errs::{CompilerResult, ErrorKind, normal::CompilerError}};

use crate::values::lower_ast_value;

pub fn lower_ast_function_call(context: &HIRContext, curr_ctx: &HIRBranchedContext, node: Box<ASTTreeNode>) -> CompilerResult<Box<HIRNode>> {
	if let ASTTreeNodeKind::FunctionCall { func, args } = node.kind.clone() {
		let f_ind = match context.functions.get_index(func.hash) {
			Some(v) => v,
			None => return Err(CompilerError::from_ast(ErrorKind::Error, IR_FIND_ELEMENT!().to_string(), &node.start, &node.end))
		};

		let func = &context.functions.vals[f_ind];
		let mut hir_args = vec![];
		let mut ind = 0;

		for ast in args {
			let hir = lower_ast_value(context, curr_ctx, ast)?;

			if !hir.get_node_type(context, curr_ctx).unwrap().can_transmute_into(&func.1[ind]) {
				return Err(CompilerError::from_ast(ErrorKind::Error, IR_FIND_ELEMENT!().to_string(), &node.start, &node.end));
			}

			hir_args.push(hir);

			ind += 1;
		}

		return Ok(Box::new(HIRNode::FunctionCall { func_name: f_ind, arguments: hir_args }))
	}
	return Err(CompilerError::from_ast(ErrorKind::Error, IR_INVALID_NODE_TYPE!().to_string(), &node.start, &node.end))
}