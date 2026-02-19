//! Critical AST -> IR conversion code module

use commons::err::{PositionlessError, PositionlessResult};
use parser::ast::tree::ASTTreeNode;

use crate::{conv::{func::parse_ir_function_decl, structs::parse_ir_struct_decl, val::parse_ir_value}, ctx::IRContext, irstruct::staticvars::IRStaticVariable};

pub mod val;
pub mod func;
pub mod control;
pub mod structs;

pub fn parse_ir_node_toplevel(ctx: &mut IRContext, node: Box<ASTTreeNode>) -> PositionlessResult<bool> {
	match *node {
		ASTTreeNode::StaticVariableDeclaration { name, var_type, val } => {
			let val = parse_ir_value(None, ctx, val, None, true)?;
			
			if val.obtain_tempstr().is_ok() {
				//let str_type = ctx.type_storage.get(STR)
				//let st = IRStaticVariable::from_str(ctx, &val.obtain_tempstr().unwrap(), name.val, t)
			}

			return Ok(true)

		},

		ASTTreeNode::FunctionDeclaration { .. } => {
			parse_ir_function_decl(ctx, node)?;

			return Ok(true);
		},

		ASTTreeNode::StructLayoutDeclaration { .. } =>  {
			parse_ir_struct_decl(ctx, node)?;

			return Ok(true);
		}

		_ => {
			return Err(PositionlessError::new(&format!("Invalid AST node {:#?} for top-level IR conversion!", node)));
		}
	}
} 