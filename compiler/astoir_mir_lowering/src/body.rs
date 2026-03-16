use astoir_hir::nodes::HIRNode;
use astoir_mir::blocks::{refer::MIRBlockReference};
use compiler_errors::{IR_INVALID_NODE_TYPE, MATH_OP_NO_ASSIGN, errs::{BaseResult, base::BaseError}};

use crate::{MIRLoweringContext, math::lower_hir_math_operation, vars::lower_hir_variable_assignment};

pub fn lower_hir_body_member(block: MIRBlockReference, node: Box<HIRNode>, ctx: &mut MIRLoweringContext) -> BaseResult<bool> {
	return match *node {
		HIRNode::VarAssigment { .. } => lower_hir_variable_assignment(block, node, ctx),
		HIRNode::MathOperation { left: _, right: _, operation: _, assignment } => {
			if !assignment {
				return Err(BaseError::err(MATH_OP_NO_ASSIGN!().to_string()))
			}

			lower_hir_math_operation(block, node, ctx)?;

			return Ok(true);
		},

		_ => return Err(BaseError::err(IR_INVALID_NODE_TYPE!().to_string()))
	}
}

pub fn lower_hir_body(block: MIRBlockReference, nodes: Vec<Box<HIRNode>>, ctx: &mut MIRLoweringContext) -> BaseResult<bool> {
	for node in nodes {
		lower_hir_body_member(block, node, ctx)?;
	}

	return Ok(true);
}