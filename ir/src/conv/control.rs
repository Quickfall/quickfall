use errors::{INKWELL_FUNC_FAILED, IR_INVALID_NODE_TYPE, IR_OBTAIN_COND, IR_TYPE_BOOL, errs::{CompilerResult, ErrorKind, normal::CompilerError}};
use parser::ast::tree::{ASTTreeNode, ASTTreeNodeKind};

use crate::{conv::{func::{parse_ir_body, parse_ir_function_body_member}, val::parse_ir_value}, ctx::{IRContext}, irstruct::funcs::IRFunction};

pub fn parse_if_statement_ir(func: &mut IRFunction, ctx: &IRContext, node: Box<ASTTreeNode>) -> CompilerResult<bool> {
	if let ASTTreeNodeKind::IfStatement { cond, body, branches, depth: _ } = node.kind.clone() {
		let mut ir_branches = vec![];

		let initial_branch = ctx.inkwell_ctx.append_basic_block(func.inkwell_func, "ifbranch_then");

		let b = branches.clone();

		for branch in b {
			match branch.kind {
				ASTTreeNodeKind::IfElseStatement { cond: _, body: _ } => {
					ir_branches.push(ctx.inkwell_ctx.append_basic_block(func.inkwell_func, "ifelse_condition"));
					ir_branches.push(ctx.inkwell_ctx.append_basic_block(func.inkwell_func, "ifelse_then"));
				},

				ASTTreeNodeKind::ElseStatement { body: _ } => {
					ir_branches.push(ctx.inkwell_ctx.append_basic_block(func.inkwell_func, "else_body"));
				}

				_ => {}
			}
		}

		ir_branches.push(ctx.inkwell_ctx.append_basic_block(func.inkwell_func, "out"));

		let first_cond = parse_ir_value(Some(&func), ctx, cond, None, false)?;

		let ob = match first_cond.obtain(ctx) {
			Ok(v) => v,
			Err(b) => return Err(CompilerError::from_base(b, &node.start, &node.end))
		};
		

		let int = match ob.obtain_as_bool() {
			Some(v) => *v,
			None => return Err(CompilerError::from_ast(ErrorKind::Error, IR_OBTAIN_COND!().to_string(), &node.start, &node.end))
		};

		match ctx.builder.build_conditional_branch(int, initial_branch, ir_branches[0]) {
			Ok(_) => {},
			Err(e) => return Err(CompilerError::from_ast(ErrorKind::Error, format!(INKWELL_FUNC_FAILED!(), "build_conditional_branch", e), &node.start, &node.end))
		};

		ctx.builder.position_at_end(initial_branch);

		func.lctx.increment_body_depth();
		parse_ir_body(ctx, func, body, true)?;

		let mut ind = 0;
		for branch in branches {
			match branch.kind {
				ASTTreeNodeKind::IfElseStatement { cond, body } => {
					ctx.builder.position_at_end(ir_branches[ind]);

					let cond_val = parse_ir_value(Some(&func), ctx, cond.unwrap(), None, false)?;

					let ob = match cond_val.obtain(ctx) {
						Ok(v) => v,
						Err(b) => return Err(CompilerError::from_base(b, &node.start, &node.end))
					};
					

					let int_cond_val = match ob.obtain_as_bool() {
						Some(v) => *v,
						None => return Err(CompilerError::from_ast(ErrorKind::Error, IR_OBTAIN_COND!().to_string(), &branch.start, &branch.end))
					};

					match ctx.builder.build_conditional_branch(int_cond_val, ir_branches[ind + 1], ir_branches[ind + 2]) {
						Ok(_) => {},
						Err(e) => return Err(CompilerError::from_ast(ErrorKind::Error, format!(INKWELL_FUNC_FAILED!(), "build_conditional_branch", e), &branch.start, &branch.end))
					}

					ctx.builder.position_at_end(ir_branches[ind + 1]);

					func.lctx.increment_body_depth();
				
					parse_ir_body(ctx, func, body, true)?;

					match ctx.builder.build_unconditional_branch(ir_branches[ir_branches.len() - 1]) {
						Ok(_) => {},
						Err(e) => return Err(CompilerError::from_ast(ErrorKind::Error, format!(INKWELL_FUNC_FAILED!(), "build_unconditional_branch", e), &branch.start, &branch.end))
					}

					ind += 2;
				},

				ASTTreeNodeKind::ElseStatement { body } => {
					ctx.builder.position_at_end(ir_branches[ind]);

					func.lctx.increment_body_depth();
					parse_ir_body(ctx, func, body, true)?;

					match ctx.builder.build_unconditional_branch(ir_branches[ir_branches.len() - 1]) {
						Ok(_) => {},
						Err(e) => return Err(CompilerError::from_ast(ErrorKind::Error, format!(INKWELL_FUNC_FAILED!(), "build_unconditional_branch", e), &branch.start, &branch.end))
					}
				},

				_ => {}
			}
		}

		ctx.builder.position_at_end(ir_branches[ir_branches.len() - 1]);

		return Ok(true);
	}

	return Err(CompilerError::from_ast(ErrorKind::Critical, IR_INVALID_NODE_TYPE!().to_string(), &node.start, &node.end));
}

pub fn parse_for_statement_ir(func: &mut IRFunction, ctx: &IRContext, node: Box<ASTTreeNode>) -> CompilerResult<bool> {
	if let ASTTreeNodeKind::ForBlock { initial_state, cond, increment, body } = node.kind {
		let for_cond_block = ctx.inkwell_ctx.append_basic_block(func.inkwell_func, "for_cond");
		let for_body_block = ctx.inkwell_ctx.append_basic_block(func.inkwell_func, "for_inner");
		let post_block = ctx.inkwell_ctx.append_basic_block(func.inkwell_func, "for_out");

		parse_ir_function_body_member(ctx, func, initial_state)?;

		println!("Post initial state");

		match ctx.builder.build_unconditional_branch(for_cond_block) {
			Ok(_) => {},
			Err(e) => return Err(CompilerError::from_ast(ErrorKind::Error, format!(INKWELL_FUNC_FAILED!(), "build_unconditional_branch", e), &node.start, &node.end))
		}

		ctx.builder.position_at_end(for_cond_block);

		let cond_val = parse_ir_value(Some(&func), ctx, cond, None, false)?;

		let ob = match cond_val.obtain(ctx) {
			Ok(v) => v,
			Err(b) => return Err(CompilerError::from_base(b, &node.start, &node.end))
		};

		let cond_int = match ob.obtain_as_bool() {
			Some(v) => v,
			None => return Err(CompilerError::from_ast(ErrorKind::Error, IR_TYPE_BOOL!().to_string(), &node.start, &node.end))
		};

		match ctx.builder.build_conditional_branch(*cond_int, for_body_block, post_block) {
			Ok(_) => {},
			Err(e) => return Err(CompilerError::from_ast(ErrorKind::Error, format!(INKWELL_FUNC_FAILED!(), "build_conditional_branch", e), &node.start, &node.end))
		}
		
		ctx.builder.position_at_end(for_body_block);

		parse_ir_body(ctx, func, body, false)?;

		parse_ir_function_body_member(ctx, func, increment)?;

		match ctx.builder.build_unconditional_branch(for_cond_block) {
			Ok(_) => {},
			Err(e) => return Err(CompilerError::from_ast(ErrorKind::Error, format!(INKWELL_FUNC_FAILED!(), "build_unconditional_branch", e), &node.start, &node.end))
		}

		ctx.builder.position_at_end(post_block);
		return Ok(true);
	}


	return Err(CompilerError::from_ast(ErrorKind::Error, IR_INVALID_NODE_TYPE!().to_string(), &node.start, &node.end));
}