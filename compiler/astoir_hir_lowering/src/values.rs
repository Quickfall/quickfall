use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use astoir_hir::{ctx::{HIRBranchedContext, HIRContext}, nodes::HIRNode, structs::StructLRUStep};
use astoir_typing::complete::ComplexType;
use compiler_errors::{IR_FIND_ELEMENT, IR_INVALID_NODE_TYPE, errs::{CompilerResult, ErrorKind, normal::CompilerError}, pos};

pub(crate) fn lower_ast_lru_base(context: &HIRContext, curr_ctx: &HIRBranchedContext, node: Box<ASTTreeNode>, curr_steps: &mut Vec<StructLRUStep>, curr_type: &mut Option<ComplexType>) -> CompilerResult<bool> {
	let struct_descriptor;

	if curr_type.is_some() {
		struct_descriptor = match curr_type.as_ref().unwrap().get_concrete().base.get_struct_container() {
			Ok(v) => Some(v),
			Err(e) => return Err(CompilerError::from_base(e, &node.start, &node.end))
		}
	} else {
		struct_descriptor = None;
	}

	match node.kind {
		ASTTreeNodeKind::FunctionCall { func, args } => {
			let func_type;
			let ind: usize;

			if struct_descriptor.is_some() {
				ind = match struct_descriptor.unwrap().get_function(func.hash) {
					Ok(v) => v,
					Err(e) => return Err(CompilerError::from_base(e, &node.start, &node.end))
				};

				func_type = &struct_descriptor.unwrap().functions.vals[ind];
			} else {
				ind = match context.functions.get_index(func.hash) {
					Some(v) => v,
					None => return Err(CompilerError::from_ast(ErrorKind::Error, IR_FIND_ELEMENT!().to_string(), &node.start, &node.end))
				};

				func_type = &context.functions.vals[ind];
			}

			// TODO: argument checking here when they get parsed

			*curr_type = func_type.0.clone();
			
			curr_steps.push(StructLRUStep::FunctionCall { func: ind, args: vec![] });

			return Ok(true);
		},

		_ => return Err(CompilerError::from_ast(ErrorKind::Error, IR_INVALID_NODE_TYPE!().to_string(), &node.start, &node.end))

	}
}

pub fn lower_ast_lru(context: &HIRContext, curr_ctx: &HIRBranchedContext, node: Box<ASTTreeNode>) -> CompilerResult<Box<HIRNode>> {
	let mut steps: Vec<StructLRUStep> = vec![];
	let mut curr_type: Option<ComplexType> = None;

	lower_ast_lru_base(context, curr_ctx, node, &mut steps, &mut curr_type)?;

	return Ok(Box::new(HIRNode::StructLRU { steps }))
}