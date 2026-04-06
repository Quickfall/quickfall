use astoir_hir::nodes::{HIRNode, HIRNodeKind};
use astoir_mir::{blocks::refer::MIRBlockReference, builder::{build_static_array_const, build_static_array_one_const}, vals::base::BaseMIRValue};
use diagnostics::{DiagnosticResult, DiagnosticSpanOrigin, move_current_diagnostic_pos, unsure_panic};

use crate::{MIRLoweringContext, arrays::lower_hir_aray_index_access, funcs::lower_hir_function_call, math::lower_hir_math_operation, type_tools::{lower_hir_unwrap_cond, lower_hir_unwrap_value}, values::{booleans::{lower_hir_boolean_operator, lowering_hir_boolean_condition}, consts::lower_hir_literal, structs::lower_hir_struct_init}, vars::{lower_hir_variable_reference, lower_hir_variable_reference_value}};

pub mod consts;
pub mod booleans;
pub mod structs;

pub fn lower_hir_value(block: MIRBlockReference, node: Box<HIRNode>, ctx: &mut MIRLoweringContext) -> DiagnosticResult<BaseMIRValue> {
	move_current_diagnostic_pos(node.get_pos());
	
	match node.kind {
		HIRNodeKind::IntegerLiteral { .. } | HIRNodeKind::StringLiteral { .. } => return lower_hir_literal(node, ctx),
		HIRNodeKind::VariableReference { .. } => return lower_hir_variable_reference_value(block, node, ctx),
		HIRNodeKind::ReferenceGrab { val } => return Ok(lower_hir_variable_reference(block, &val, ctx)?.as_pointer_ref()?.into()),
		HIRNodeKind::PointerGrab { val } => return Ok(lower_hir_variable_reference(block, &val, ctx)?.as_pointer_ref()?.into()),
		HIRNodeKind::BooleanCondition { .. } => return Ok(lowering_hir_boolean_condition(block, node, ctx)?.into()),
		HIRNodeKind::BooleanOperator { .. } => return Ok(lower_hir_boolean_operator(block, node, ctx)?.into()),
		HIRNodeKind::MathOperation { .. } => return Ok(lower_hir_math_operation(block, node, ctx)?),
		HIRNodeKind::ArrayIndexAccess { .. } => return Ok(lower_hir_aray_index_access(block, node, ctx)?),
		HIRNodeKind::StructVariableInitializerValue { .. } => return Ok(lower_hir_struct_init(block, node, ctx)?.into()),
		HIRNodeKind::UnwrapValue { .. } => lower_hir_unwrap_value(block, node, ctx),
		HIRNodeKind::UnwrapCondition { .. } => lower_hir_unwrap_cond(block, node, ctx),
		HIRNodeKind::ArrayVariableInitializerValue { .. } | HIRNodeKind::ArrayVariableInitializerValueSameValue { .. } => lower_array_init(block, node, ctx),
		HIRNodeKind::FunctionCall { .. } => {
			let res = lower_hir_function_call(block, node, ctx)?;

			if res.is_none() {
				unsure_panic!("expected val func");
			}

			return Ok(res.unwrap());
		}

		_ => panic!("Invalid node")
	}
}

pub fn lower_array_init(block: MIRBlockReference, node: Box<HIRNode>, ctx: &mut MIRLoweringContext) -> DiagnosticResult<BaseMIRValue> {
	match node.kind.clone() {
		HIRNodeKind::ArrayVariableInitializerValue { vals } => {

			let mut v = vec![];

			for val in vals {
				v.push(lower_hir_value(block, val, ctx)?)
			}

			return Ok(build_static_array_const(&mut ctx.mir_ctx, v)?.into())
		},

		HIRNodeKind::ArrayVariableInitializerValueSameValue { size, val } => {
			let v = lower_hir_value(block, val, ctx)?;

			return Ok(build_static_array_one_const(&mut ctx.mir_ctx, v, size)?.into());
		},

		_ => panic!("Invalid node")
	}
}