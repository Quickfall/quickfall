use astoir_hir::nodes::{HIRNode, HIRNodeKind};
use astoir_mir::{blocks::refer::MIRBlockReference, insts::MIRInstruction};
use diagnostics::{DiagnosticResult, DiagnosticSpanOrigin, builders::make_math_operation_req_assign, move_current_diagnostic_pos};

use crate::{MIRLoweringContext, arrays::lower_hir_array_modify, control::{forloop::lower_hir_for_loop, ifstatement::lower_hir_if_statement}, funcs::lower_hir_function_call, introductions::handle_var_introduction_queue, math::lower_hir_math_operation, values::lower_hir_value, vars::{lower_hir_variable_assignment, lower_hir_variable_declaration}};

pub fn lower_hir_body_member(block: MIRBlockReference, node: Box<HIRNode>, ctx: &mut MIRLoweringContext) -> DiagnosticResult<bool> {
	move_current_diagnostic_pos(node.get_pos());

	let queue = ctx.block_introduction_var_queue.clone();
	for n in queue {
		handle_var_introduction_queue(block, n.clone(), ctx)?;
	}
	
	return match node.kind.clone() {
		HIRNodeKind::VarAssigment { .. } => lower_hir_variable_assignment(block, node, ctx),
		HIRNodeKind::VarDeclaration { .. } => lower_hir_variable_declaration(block, node, ctx),
		HIRNodeKind::MathOperation { left: _, right: _, operation: _, assignment } => {
			if !assignment {
				return Err(make_math_operation_req_assign(&*node).into())
			}

			lower_hir_math_operation(block, node, ctx)?;

			return Ok(true);
		},

		HIRNodeKind::ArrayIndexModify { .. } => lower_hir_array_modify(block, node, ctx),

		HIRNodeKind::ForBlock { .. } => lower_hir_for_loop(block, node, ctx),
		HIRNodeKind::IfStatement { .. } => lower_hir_if_statement(block, node, ctx),
		HIRNodeKind::FunctionCall { .. } => {
			lower_hir_function_call(block, node, ctx)?;

			return Ok(true)
		},

		HIRNodeKind::ReturnStatement { value } => {
			if value.is_some() {
				let val = lower_hir_value(block, value.unwrap(), ctx)?;

				ctx.mir_ctx.append_inst(MIRInstruction::Return { val: Some(val) });
				return Ok(true);
			}

			ctx.mir_ctx.append_inst(MIRInstruction::Return { val: None });

			return Ok(true);
		}

		_ => panic!("Invalid node")
	}
}

pub fn lower_hir_body(block: MIRBlockReference, nodes: Vec<Box<HIRNode>>, ctx: &mut MIRLoweringContext) -> DiagnosticResult<bool> {
	for node in nodes {
		lower_hir_body_member(block, node, ctx)?;
	}

	return Ok(true);
}