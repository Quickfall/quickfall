//! Critical AST -> IR conversion code module

use commons::err::{PositionlessError, PositionlessResult};
use parser::ast::tree::ASTTreeNode;

use crate::{conv::func::parse_ir_function_decl, ctx::IRContext};

pub mod val;
pub mod func;
pub mod control;

pub fn parse_ir_node_toplevel(ctx: &mut IRContext, node: Box<ASTTreeNode>) -> PositionlessResult<bool> {
	match *node {
		ASTTreeNode::FunctionDeclaration { .. } => {
			parse_ir_function_decl(ctx, node)?;

			return Ok(true);
		},

		_ => {
			return Err(PositionlessError::new(&format!("Invalid AST node {:#?} for top-level IR conversion!", node)));
		}
	}
} 