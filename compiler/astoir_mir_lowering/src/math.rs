use astoir_hir::nodes::{HIRNode, HIRNodeKind};
use astoir_mir::{
    blocks::refer::MIRBlockReference,
    builder::{
        build_float_add, build_float_div, build_float_mod, build_float_mul, build_float_sub,
        build_int_add, build_int_div, build_int_mod, build_int_mul, build_int_sub,
        build_shift_left, build_shift_right,
    },
    vals::base::BaseMIRValue,
};
use compiler_typing::raw::RawType;
use compiler_utils::operators::{MathOperator, MathOperatorType};
use diagnostics::{
    DiagnosticResult,
    builders::{make_math_operation_req_assign, make_req_type_kind},
    unsure_panic,
};

use crate::{MIRLoweringContext, values::lower_hir_value, vars::lower_hir_variable_reference};

pub fn lower_hir_math_operation(
    block: MIRBlockReference,
    node: Box<HIRNode>,
    ctx: &mut MIRLoweringContext,
) -> DiagnosticResult<BaseMIRValue> {
    if let HIRNodeKind::MathOperation {
        left,
        right,
        operation,
    } = node.clone().kind
    {
        if operation.assigns && !left.is_variable_reference() {
            return Err(make_math_operation_req_assign(&*node).into());
        }

        let ptr;

        if operation.assigns {
            ptr = Some(lower_hir_variable_reference(block, &left, ctx)?);
        } else {
            ptr = None
        }

        let left_val = lower_hir_value(block, left, ctx)?;
        let right_val = lower_hir_value(block, right, ctx)?;

        let val = match left_val.vtype.get_generic(&ctx.hir_ctx.type_storage) {
            RawType::Integer(_, _) | RawType::FixedPoint(_, _, _) => {
                lower_hir_math_operation_int(left_val, right_val, operation.clone(), ctx)?
            }
            RawType::Floating(_, _) => {
                lower_hir_math_operation_float(left_val, right_val, operation.clone(), ctx, &*node)?
            }

            _ => unsure_panic!("Cannot use lower_hir_math_operator on this given value kind!"),
        };

        if operation.assigns {
            let v = ptr.unwrap();

            v.write(
                block,
                &mut ctx.mir_ctx,
                val.clone(),
                &ctx.hir_ctx.type_storage,
            )?;
        }

        return Ok(val);
    }

    panic!("Invalid node")
}

pub fn lower_hir_math_operation_int(
    left: BaseMIRValue,
    right: BaseMIRValue,
    operator: MathOperator,
    ctx: &mut MIRLoweringContext,
) -> DiagnosticResult<BaseMIRValue> {
    let left = left.as_int()?;
    let right = right.as_int()?;

    let signed = left.signed;

    let res = match operator.operator {
        MathOperatorType::Add => {
            build_int_add(&mut ctx.mir_ctx, left, right, signed, operator.fast)?
        }
        MathOperatorType::Subtract => {
            build_int_sub(&mut ctx.mir_ctx, left, right, signed, operator.fast)?
        }
        MathOperatorType::Multiply => {
            build_int_mul(&mut ctx.mir_ctx, left, right, signed, operator.fast)?
        }
        MathOperatorType::Divide => {
            build_int_div(&mut ctx.mir_ctx, left, right, signed, operator.fast)?
        }
        MathOperatorType::ShiftLeft => build_shift_left(&mut ctx.mir_ctx, left, right)?,
        MathOperatorType::ShiftRight => build_shift_right(&mut ctx.mir_ctx, left, right)?,
        MathOperatorType::Modulo => {
            build_int_mod(&mut ctx.mir_ctx, left, right, signed, operator.fast)?
        }
    };

    return Ok(res.into());
}

pub fn lower_hir_math_operation_float(
    left: BaseMIRValue,
    right: BaseMIRValue,
    operator: MathOperator,
    ctx: &mut MIRLoweringContext,
    node: &HIRNode,
) -> DiagnosticResult<BaseMIRValue> {
    let left = left.as_float()?;
    let right = right.as_float()?;

    let signed = left.signed;

    let res = match operator.operator {
        MathOperatorType::Add => {
            build_float_add(&mut ctx.mir_ctx, left, right, signed, operator.fast)?
        }
        MathOperatorType::Subtract => {
            build_float_sub(&mut ctx.mir_ctx, left, right, signed, operator.fast)?
        }
        MathOperatorType::Multiply => {
            build_float_mul(&mut ctx.mir_ctx, left, right, signed, operator.fast)?
        }
        MathOperatorType::Divide => {
            build_float_div(&mut ctx.mir_ctx, left, right, signed, operator.fast)?
        }
        MathOperatorType::Modulo => {
            build_float_mod(&mut ctx.mir_ctx, left, right, signed, operator.fast)?
        }

        _ => return Err(make_req_type_kind(node, &"integer".to_string()).into()),
    };

    return Ok(res.into());
}
