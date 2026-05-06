use ast::{
    ranges::ASTRange,
    tree::{ASTTreeNode, ASTTreeNodeKind},
};
use astoir_hir::{
    ctx::{HIRBranchedContext, HIRContext, get_variable},
    nodes::{HIRNode, HIRNodeKind},
    structs::{HIRRange, StructLRUStep},
};
use compiler_global_scope::key::EntryKey;
use compiler_typing::tree::Type;
use diagnostics::{
    DiagnosticResult, DiagnosticSpanOrigin,
    builders::{make_invalid_pointing, make_struct_missing_field, make_struct_missing_func},
};

use crate::{
    arrays::lower_ast_array_index_access,
    bools::{lower_ast_boolean_condition, lower_ast_operator_condition},
    func::lower_ast_function_call,
    literals::lower_ast_literal,
    math::lower_ast_math_operation,
    structs::lower_ast_struct_initializer,
    unwraps::{lower_ast_condition_unwrap, lower_ast_unwrap_value},
    var::lower_ast_variable_reference,
};

pub(crate) fn lower_ast_lru_base(
    context: &mut HIRContext,
    curr_ctx: &mut HIRBranchedContext,
    node: Box<ASTTreeNode>,
    curr_steps: &mut Vec<StructLRUStep>,
    curr_type: &mut Option<Type>,
) -> DiagnosticResult<bool> {
    match node.clone().kind {
        ASTTreeNodeKind::FunctionCall { func, args } => {
            let func_type;
            let ind: usize;

            if let Some(curr_type_val) = curr_type {
                let res = match curr_type_val.get_function(&context.global_scope.scope, func.hash) {
                    Ok(v) => v,
                    Err(_) => {
                        return Err(
                            make_struct_missing_func(&*node, curr_type_val, &func.val).into()
                        );
                    }
                };

                let abstract_func = res.1;

                let mut resolved_args = vec![];

                for arg in abstract_func.0 {
                    resolved_args.push((arg.0, arg.1.resolve(curr_type_val)));
                }

                let resolved_ret;

                if abstract_func.1.is_some() {
                    resolved_ret = Some(abstract_func.1.unwrap().resolve(curr_type_val));
                } else {
                    resolved_ret = None;
                }

                func_type = (resolved_ret, resolved_args, func.val.clone());

                ind = res.0;
            } else {
                let entry = EntryKey {
                    name_hash: func.hash,
                };

                ind = context.global_scope.get_ind(entry.clone(), &*node)?;
                func_type = context.global_scope.get_function_base(entry, &*node)?;
            }

            let mut hir_args = vec![];
            let mut iind = 0;

            for a in args {
                let lowered = lower_ast_value(context, curr_ctx, a)?;

                let lowered = Box::new(lowered.use_as(
                    context,
                    curr_ctx,
                    func_type.1[iind].1.clone(),
                    &*node,
                    None,
                )?);

                hir_args.push(lowered);

                iind += 1;
            }

            *curr_type = func_type.0.clone();

            curr_steps.push(StructLRUStep::FunctionCall {
                func: ind,
                args: hir_args,
            });

            return Ok(true);
        }

        ASTTreeNodeKind::VariableReference(str) => {
            let var_type;
            let ind: usize;

            if let Some(curr_type_val) = curr_type {
                let res = match curr_type_val.get_field(&context.global_scope.scope, str.hash) {
                    Ok(v) => v,
                    Err(_) => {
                        return Err(
                            make_struct_missing_field(&*node, curr_type_val, &str.val).into()
                        );
                    }
                };

                ind = res.0;
                var_type = res.1.resolve(curr_type_val);
            } else {
                let r = get_variable(context, curr_ctx, str.hash, &*node)?;

                ind = r.2;
                var_type = r.1;
            }

            curr_steps.push(StructLRUStep::VariableStep { variable: ind });
            *curr_type = Some(var_type);

            return Ok(true);
        }

        ASTTreeNodeKind::StructLRFunction { l, r } => {
            lower_ast_lru_base(context, curr_ctx, l, curr_steps, curr_type)?;
            lower_ast_lru_base(context, curr_ctx, r, curr_steps, curr_type)?;

            return Ok(true);
        }

        ASTTreeNodeKind::StructLRVariable { l, r } => {
            lower_ast_lru_base(context, curr_ctx, l, curr_steps, curr_type)?;
            lower_ast_lru_base(context, curr_ctx, r, curr_steps, curr_type)?;

            return Ok(true);
        }

        _ => panic!("Invalid node type"),
    }
}

pub fn lower_ast_lru(
    context: &mut HIRContext,
    curr_ctx: &mut HIRBranchedContext,
    node: Box<ASTTreeNode>,
) -> DiagnosticResult<Box<HIRNode>> {
    let mut steps: Vec<StructLRUStep> = vec![];
    let mut curr_type: Option<Type> = None;

    lower_ast_lru_base(context, curr_ctx, node.clone(), &mut steps, &mut curr_type)?;

    return Ok(Box::new(HIRNode::new(
        HIRNodeKind::StructLRU {
            steps,
            last: curr_type.unwrap(),
        },
        &node.start,
        &node.end,
    )));
}

pub fn lower_ast_value(
    context: &mut HIRContext,
    curr_ctx: &mut HIRBranchedContext,
    node: Box<ASTTreeNode>,
) -> DiagnosticResult<Box<HIRNode>> {
    match node.kind {
        ASTTreeNodeKind::StructLRFunction { .. } | ASTTreeNodeKind::StructLRVariable { .. } => {
            return lower_ast_lru(context, curr_ctx, node);
        }

        ASTTreeNodeKind::MathResult { .. } => {
            return lower_ast_math_operation(context, curr_ctx, node, false);
        }

        ASTTreeNodeKind::OperatorBasedConditionMember { .. } => {
            return lower_ast_operator_condition(context, curr_ctx, node);
        }

        ASTTreeNodeKind::BooleanBasedConditionMember { .. } => {
            return lower_ast_boolean_condition(context, curr_ctx, node);
        }

        ASTTreeNodeKind::ArrayIndexAccess { .. } => {
            return lower_ast_array_index_access(context, curr_ctx, node);
        }

        ASTTreeNodeKind::ArrayVariableInitializerValue { .. }
        | ASTTreeNodeKind::ArrayVariableInitializerValueSameValue { .. } => {
            return lower_ast_array_init(context, curr_ctx, node);
        }

        ASTTreeNodeKind::StructInitializer { .. } => {
            return lower_ast_struct_initializer(context, curr_ctx, node);
        }

        ASTTreeNodeKind::IntegerLit { .. } | ASTTreeNodeKind::StringLit(_) => {
            return lower_ast_literal(context, node);
        }

        ASTTreeNodeKind::FunctionCall { .. } => {
            return lower_ast_function_call(context, curr_ctx, node);
        }

        ASTTreeNodeKind::VariableReference(_) => {
            return lower_ast_variable_reference(context, curr_ctx, node, true);
        }

        ASTTreeNodeKind::PointerGrab(_) => return lower_ast_pointer(context, curr_ctx, node),

        ASTTreeNodeKind::ReferenceGrab(_) => return lower_ast_reference(context, curr_ctx, node),

        ASTTreeNodeKind::UnwrapCondition { .. } => {
            return lower_ast_condition_unwrap(context, curr_ctx, node);
        }

        ASTTreeNodeKind::UnwrapValue { .. } => {
            return lower_ast_unwrap_value(context, curr_ctx, node);
        }

        _ => panic!("Invalid AST value node"),
    }
}

pub fn lower_ast_range<K: DiagnosticSpanOrigin>(
    context: &mut HIRContext,
    curr_ctx: &mut HIRBranchedContext,
    range: ASTRange,
    ty: Type,
    origin: &K,
) -> DiagnosticResult<HIRRange> {
    let min = Box::new(
        lower_ast_value(context, curr_ctx, range.min.clone())?.use_as(
            context,
            curr_ctx,
            ty.clone(),
            origin,
            None,
        )?,
    );
    let max = Box::new(
        lower_ast_value(context, curr_ctx, range.max)?
            .use_as(context, curr_ctx, ty, origin, None)?,
    );

    Ok(HIRRange { min, max })
}

pub fn lower_ast_array_init(
    context: &mut HIRContext,
    curr_ctx: &mut HIRBranchedContext,
    node: Box<ASTTreeNode>,
) -> DiagnosticResult<Box<HIRNode>> {
    if let ASTTreeNodeKind::ArrayVariableInitializerValue { vals } = node.kind.clone() {
        let mut values = vec![];

        for val in vals {
            values.push(lower_ast_value(context, curr_ctx, val)?);
        }

        return Ok(Box::new(HIRNode::new(
            HIRNodeKind::ArrayVariableInitializerValue { vals: values },
            &node.start,
            &node.end,
        )));
    }

    if let ASTTreeNodeKind::ArrayVariableInitializerValueSameValue { size, v } = node.kind {
        return Ok(Box::new(HIRNode::new(
            HIRNodeKind::ArrayVariableInitializerValueSameValue {
                size,
                val: lower_ast_value(context, curr_ctx, v)?,
            },
            &node.start,
            &node.end,
        )));
    }

    panic!("Invalid node")
}

pub fn lower_ast_pointer(
    context: &mut HIRContext,
    curr_ctx: &mut HIRBranchedContext,
    node: Box<ASTTreeNode>,
) -> DiagnosticResult<Box<HIRNode>> {
    if let ASTTreeNodeKind::PointerGrab(val) = node.kind.clone() {
        let val = lower_ast_value(context, curr_ctx, val)?;

        if !val.is_variable_representative() {
            return Err(make_invalid_pointing(&*node).into());
        }

        let r = val.get_variable_represent();

        curr_ctx.introduce_variable_refer(r.0);

        return Ok(Box::new(HIRNode::new(
            HIRNodeKind::PointerGrab { val },
            &node.start,
            &node.end,
        )));
    }

    panic!("Invalid node")
}

pub fn lower_ast_reference(
    context: &mut HIRContext,
    curr_ctx: &mut HIRBranchedContext,
    node: Box<ASTTreeNode>,
) -> DiagnosticResult<Box<HIRNode>> {
    if let ASTTreeNodeKind::ReferenceGrab(val) = node.kind.clone() {
        let val = lower_ast_value(context, curr_ctx, val)?;

        if !val.is_variable_representative() {
            return Err(make_invalid_pointing(&*node).into());
        }

        let r = val.get_variable_represent();

        curr_ctx.introduce_variable_refer(r.0);

        return Ok(Box::new(HIRNode::new(
            HIRNodeKind::ReferenceGrab { val },
            &node.start,
            &node.end,
        )));
    }

    panic!("Invalid node")
}
