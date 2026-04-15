use ast::{ctx::ParserCtx, tree::{ASTTreeNode, ASTTreeNodeKind}};
use astoir_hir::{ctx::{HIRBranchedContext, HIRContext}, nodes::{HIRNode, HIRNodeKind}};
use diagnostics::{DiagnosticResult, DiagnosticSpanOrigin, move_current_diagnostic_pos};
use prelude::apply_prelude;

use crate::{arrays::lower_ast_array_modify, control::{lower_ast_for_block, lower_ast_if_statement, lower_ast_while_block}, enums::lower_ast_enum, func::{lower_ast_function_call, lower_ast_function_declaration, lower_ast_shadow_function_declaration}, math::lower_ast_math_operation, structs::lower_ast_struct_declaration, uses::handle_ast_use_statement, values::lower_ast_value, var::{lower_ast_variable_assign, lower_ast_variable_declaration}};

pub mod literals;
pub mod var;
pub mod types;
pub mod values;
pub mod func;
pub mod math;
pub mod bools;
pub mod control;
pub mod structs;
pub mod arrays;
pub mod unwraps;
pub mod enums;
pub mod uses;

pub fn lower_ast_body_node(context: &mut HIRContext, curr_ctx: &mut HIRBranchedContext, node: Box<ASTTreeNode>) -> DiagnosticResult<Box<HIRNode>> {
	move_current_diagnostic_pos(node.get_pos());
	match node.kind.clone() {
		ASTTreeNodeKind::VarDeclaration { .. } => return lower_ast_variable_declaration(context, curr_ctx, node),
		ASTTreeNodeKind::FunctionCall { .. } =>  return lower_ast_function_call(context, curr_ctx, node),
		ASTTreeNodeKind::VarValueChange { .. } => return lower_ast_variable_assign(context, curr_ctx, node),
		ASTTreeNodeKind::ArrayIndexModifiy { .. } => return lower_ast_array_modify(context, curr_ctx, node),

		ASTTreeNodeKind::MathResult { .. } => return lower_ast_math_operation(context, curr_ctx, node, true),

		ASTTreeNodeKind::ReturnStatement { val } => {
			let v;

			if val.is_none() {
				v = None;
			} else {
				v = Some(lower_ast_value(context, curr_ctx, val.unwrap())?)
			}

			return Ok(Box::new(HIRNode::new(HIRNodeKind::ReturnStatement { value: v }, &node.start, &node.end)))
 		},

		ASTTreeNodeKind::ForBlock { .. } => return lower_ast_for_block(context, curr_ctx, node),
		ASTTreeNodeKind::WhileBlock { .. } => return lower_ast_while_block(context, curr_ctx, node),
		ASTTreeNodeKind::IfStatement { .. } => return lower_ast_if_statement(context, curr_ctx, node),
		
		_ => panic!("Invalid node type")
	}
}

pub fn lower_ast_body(context: &mut HIRContext, curr_ctx: &mut HIRBranchedContext, nodes: Vec<Box<ASTTreeNode>>, introduce_era: bool) -> DiagnosticResult<Vec<Box<HIRNode>>> {
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

pub fn lower_ast_toplevel(context: &mut HIRContext, node: Box<ASTTreeNode>) -> DiagnosticResult<bool> {
	match node.kind {
		ASTTreeNodeKind::FunctionDeclaration { .. } => {
			let func_decl = lower_ast_function_declaration(context, node)?;

			context.function_declarations.push(Some(func_decl));

			return Ok(true)
		},

		ASTTreeNodeKind::ShadowFunctionDeclaration { .. } => {
			let func_decl = lower_ast_shadow_function_declaration(context, node)?;
			
			context.function_declarations.push(Some(func_decl));
			
			return Ok(true);
		},

		ASTTreeNodeKind::StructLayoutDeclaration { .. } => {
			lower_ast_struct_declaration(context, node)?;

			return Ok(true)
		},

		ASTTreeNodeKind::EnumDeclaration { .. } => {
			lower_ast_enum(context, node)?;

			return Ok(true)
		}

		_ => panic!("Invalid node type")
	}
} 

pub fn lower_ast(ctx: ParserCtx) -> DiagnosticResult<HIRContext> {
	let mut hir_ctx = HIRContext::new();
	apply_prelude(&mut hir_ctx)?;

	for u in ctx.uses {
		handle_ast_use_statement(&mut hir_ctx, u)?;
	}

	for s in ctx.iter_order {
		let k = ctx.map[&s].clone();

		lower_ast_toplevel(&mut hir_ctx, k)?;
	}

	return Ok(hir_ctx);
}