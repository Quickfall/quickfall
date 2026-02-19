//! Critical AST -> IR conversion code module

use commons::err::{PositionlessError, PositionlessResult};
use parser::ast::tree::ASTTreeNode;

use crate::{conv::{func::parse_ir_function_decl, structs::parse_ir_struct_decl, val::parse_ir_value}, ctx::IRContext, irstruct::staticvars::IRStaticVariable, types::STATICSTR_TYPE_HASH};

pub mod val;
pub mod func;
pub mod control;
pub mod structs;

pub fn parse_ir_node_toplevel(ctx: &mut IRContext, node: Box<ASTTreeNode>) -> PositionlessResult<bool> {
	match *node {
		ASTTreeNode::StaticVariableDeclaration { name, var_type, val } => {
			let val = parse_ir_value(None, ctx, val, None, true)?;
			
			let t = match ctx.type_storage.get(var_type) {
				Some(v) => v,
				None => return Err(PositionlessError::new(&format!("Cannot find type {}", var_type)))
			};

			if val.obtain_tempstr().is_ok() {
				let str_type = ctx.type_storage.get(STATICSTR_TYPE_HASH).expect("staticstr type was not found!");

				if !t.is_same(&str_type) {
					return Err(PositionlessError::new("Expected type staticstr for static string variable!"))
				}

				let st = IRStaticVariable::from_str(ctx, &val.obtain_tempstr().unwrap(), name.val, str_type)?;

				ctx.add_variable(name.hash, st)?;
			} else {
				let st = IRStaticVariable::from_val(name.val, t, val.obtain(ctx)?)?;

				ctx.add_variable(name.hash, st)?;
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