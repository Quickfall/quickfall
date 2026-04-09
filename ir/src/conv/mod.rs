//! Critical AST -> IR conversion code module

use errors::{IR_FIND_TYPE, IR_INVALID_NODE_TYPE, IR_STATIC_STR_TYPE, errs::{CompilerResult, ErrorKind, normal::CompilerError}};
use parser::ast::tree::{ASTTreeNode, ASTTreeNodeKind};

use crate::{conv::{func::{parse_ir_function_decl, parse_ir_shadow_function_decl}, structs::parse_ir_struct_decl, val::parse_ir_value}, ctx::IRContext, irstruct::staticvars::IRStaticVariable, types::STATICSTR_TYPE_HASH};

pub mod val;
pub mod func;
pub mod control;
pub mod structs;

pub fn parse_ir_node_toplevel(ctx: &mut IRContext, node: Box<ASTTreeNode>) -> CompilerResult<bool> {
	match node.kind {
		ASTTreeNodeKind::StaticVariableDeclaration { name, var_type, val } => {
			let val = parse_ir_value(None, ctx, val, None, true)?;
			
			let t = match ctx.type_storage.get(var_type) {
				Some(v) => v,
				None => return Err(CompilerError::from_ast(ErrorKind::Error, IR_FIND_TYPE!().to_string(), &node.start, &node.end))
			};

			if val.obtain_tempstr().is_ok() {
				let str_type = ctx.type_storage.get(STATICSTR_TYPE_HASH).expect("staticstr type was not found!");

				if !t.is_same(&str_type) {
					return Err(CompilerError::from_ast(ErrorKind::Error, IR_STATIC_STR_TYPE!().to_string(), &node.start, &node.end))
				}

				let tempstr = match val.obtain_tempstr() {
					Ok(v) => v,
					Err(b) => return Err(CompilerError::from_base(b, &node.start, &node.end))
				};

				let st = match IRStaticVariable::from_str(ctx, &tempstr, name.val, str_type) {
					Ok(v) => v,
					Err(b) => return Err(CompilerError::from_base(b, &node.start, &node.end))
				};

				match ctx.add_variable(name.hash, st) {
					Ok(_) => {},
					Err(b) => return Err(CompilerError::from_base(b, &node.start, &node.end))
				};

			} else {
				let ob = match val.obtain(ctx) {
					Ok(v) => v,
					Err(b) => return Err(CompilerError::from_base(b, &node.start, &node.end))
				};
				
				let st = match IRStaticVariable::from_val(name.val, t, ob) {
					Ok(v) => v,
					Err(b) => return Err(CompilerError::from_base(b, &node.start, &node.end))
				};

				match ctx.add_variable(name.hash, st) {
					Ok(_) => {},
					Err(b) => return Err(CompilerError::from_base(b, &node.start, &node.end))
				};
		
			}

			return Ok(true)

		},

		ASTTreeNodeKind::FunctionDeclaration { .. } => {
			parse_ir_function_decl(ctx, node)?;

			return Ok(true);
		},

		ASTTreeNodeKind::ShadowFunctionDeclaration { .. } => {
			parse_ir_shadow_function_decl(ctx, node)?;

			return Ok(true);
		}

		ASTTreeNodeKind::StructLayoutDeclaration { .. } =>  {
			parse_ir_struct_decl(ctx, node)?;

			return Ok(true);
		}

		_ => {
			return Err(CompilerError::from_ast(ErrorKind::Error, IR_INVALID_NODE_TYPE!().to_string(), &node.start, &node.end));
		}
	}
} 