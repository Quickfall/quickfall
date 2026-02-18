use std::sync::Condvar;

use commons::err::{PositionlessError, PositionlessResult};
use parser::ast::tree::ASTTreeNode;

use crate::{conv::{func::parse_ir_body, val::parse_ir_value}, ctx::{IRContext, IRLocalContext}, irstruct::funcs::IRFunction, types::BOOL_TYPE_HASH};

pub fn parse_if_statement_ir(func: &mut IRFunction, ctx: &IRContext, node: Box<ASTTreeNode>) -> PositionlessResult<bool> {
	if let ASTTreeNode::IfStatement { cond, body, branches, depth } = *node {
		let mut ir_branches = vec![];

		let initial_branch = ctx.inkwell_ctx.append_basic_block(func.inkwell_func, "ifbranch_then");

		let b = branches.clone();

		for branch in b {
			match *branch {
				ASTTreeNode::IfElseStatement { cond, body } => {
					ir_branches.push(ctx.inkwell_ctx.append_basic_block(func.inkwell_func, "ifelse_condition"));
					ir_branches.push(ctx.inkwell_ctx.append_basic_block(func.inkwell_func, "ifelse_then"));
				},

				ASTTreeNode::ElseStatement { body } => {
					ir_branches.push(ctx.inkwell_ctx.append_basic_block(func.inkwell_func, "ifelse_elseclause"))
				},

				_ => {}
			}
			ir_branches.push(ctx.inkwell_ctx.append_basic_block(func.inkwell_func, "ifbranch"));
		}

		ir_branches.push(ctx.inkwell_ctx.append_basic_block(func.inkwell_func, "out"));

		let first_cond = parse_ir_value(Some(&func.lctx), ctx, cond, None)?;

		let bool_type = ctx.type_storage.get(BOOL_TYPE_HASH).unwrap();

		let int = match first_cond.obtain(ctx)?.obtain_as_int(ctx, bool_type.clone()) {
			Some(v) => *v,
			None => return Err(PositionlessError::new("Cannot cast first cond as int"))
		};

		match ctx.builder.build_conditional_branch(int, initial_branch, ir_branches[0]) {
			Ok(_) => {},
			Err(_) => return Err(PositionlessError::new("build_conditional_branch initial failed!"))
		};

		ctx.builder.position_at_end(initial_branch);

		parse_ir_body(ctx, func, body)?;

		let mut ind = 0;
		for branch in branches {
			match *branch {
				ASTTreeNode::IfElseStatement { cond, body } => {
					ctx.builder.position_at_end(ir_branches[ind]);

					let cond_val = parse_ir_value(Some(&func.lctx), ctx, cond.unwrap(), None)?;

					let int_cond_val = match cond_val.obtain(ctx)?.obtain_as_int(ctx, bool_type.clone()) {
						Some(v) => *v,
						None => return Err(PositionlessError::new("Cannoit cast condition as int!"))
					};

					match ctx.builder.build_conditional_branch(int_cond_val, ir_branches[ind + 1], ir_branches[ind + 2]) {
						Ok(_) => {},
						Err(_) => return Err(PositionlessError::new("build_conditional_branch nested failed!"))
					}

					ctx.builder.position_at_end(ir_branches[ind + 1]);

					parse_ir_body(ctx, func, body)?;

					match ctx.builder.build_unconditional_branch(ir_branches[ir_branches.len() - 1]) {
						Ok(_) => {},
						Err(_) => return Err(PositionlessError::new("build_conditional_branch nested failed!"))
					}

					ind += 2;
				},

				ASTTreeNode::ElseStatement { body } => {
					ctx.builder.position_at_end(ir_branches[ind]);

					parse_ir_body(ctx, func, body)?;

					match ctx.builder.build_unconditional_branch(ir_branches[ir_branches.len() - 1]) {
						Ok(_) => {},
						Err(_) => return Err(PositionlessError::new("build_conditional_branch nested failed!"))
					}
				},

				_ => {}
			}
		}

	}

	return Err(PositionlessError::new("Cannot parse if statement as this is not an if!"));
}