use compiler_utils::{Position, hash::HashedString};

use ast::{
    make_node,
    tree::{ASTTreeNode, ASTTreeNodeKind},
};
use diagnostics::{
    DiagnosticResult,
    builders::{make_expected_simple_error, make_unexpected_simple_error},
};
use lexer::token::{LexerToken, LexerTokenType};

use crate::literals::{parse_integer_literal, parse_string_literal};
use crate::math::parse_math_operation;
use crate::{
    arrays::parse_array_access,
    comp::parse_ast_compare,
    functions::parse_function_call,
    structs::val::parse_struct_initialize,
    unwraps::{parse_unwrap_condition, parse_unwrap_value},
};

pub fn parse_ast_value_dotacess(
    tokens: &Vec<LexerToken>,
    ind: &mut usize,
    original: DiagnosticResult<Box<ASTTreeNode>>,
) -> DiagnosticResult<Box<ASTTreeNode>> {
    match &tokens[*ind].tok_type {
        LexerTokenType::Dot => {
            let original = original?;
            if !original.kind.is_function_call() && !original.kind.is_var_access() {
                return Err(make_unexpected_simple_error(&tokens[*ind], &original).into());
            }

            *ind += 1;
            let r = parse_ast_value_dotacess_chain_member(tokens, ind, Ok(original))?;

            if tokens[*ind].tok_type == LexerTokenType::Dot {
                return parse_ast_value_dotacess(tokens, ind, Ok(r)); // Continue the chain until finished
            }

            return Ok(r);
        }

        _ => return original,
    }
}

pub fn parse_ast_value_dotacess_chain_member(
    tokens: &Vec<LexerToken>,
    ind: &mut usize,
    original: DiagnosticResult<Box<ASTTreeNode>>,
) -> DiagnosticResult<Box<ASTTreeNode>> {
    match &tokens[*ind].tok_type {
        LexerTokenType::Keyword(s, _) => {
            if tokens[*ind + 1].tok_type == LexerTokenType::ParenOpen {
                let r_member = parse_function_call(tokens, ind)?;
                let start = original.as_ref().unwrap().start.clone();
                let end = r_member.end.clone();

                return Ok(Box::new(ASTTreeNode::new(
                    ASTTreeNodeKind::StructLRFunction {
                        l: original?,
                        r: r_member,
                    },
                    start,
                    end,
                )));
            }

            let start = original.clone()?.start.clone();
            let end = tokens[*ind].get_end_pos();

            let r_member = Box::new(ASTTreeNode::new(
                ASTTreeNodeKind::VariableReference(HashedString::new(s.clone())),
                start.clone(),
                end,
            ));

            *ind += 1;

            let end_r = r_member.end.clone();

            return Ok(Box::new(ASTTreeNode::new(
                ASTTreeNodeKind::StructLRVariable {
                    l: original?,
                    r: r_member,
                },
                start,
                end_r,
            )));
        }

        _ => return original,
    };
}

/// Parses the post side of an AST node that can and WILL be intrepreted as a value.
///
/// This function should only be called by `parse_ast_value`
///
/// # Parsing Layout
/// The `parse_ast_value` function only parses the post side of the expression (noted L) if expression is:
/// `R (pre) expression L (post) expression`
///
/// This layout allows us to seperate parsing from things like variable references, functions calls or even literals and
/// treat them as the same while parsing other elements such as math operations or conditions!
///
/// # Possible node results
/// `parse_ast_value_post_l` can possibly return the following node types:
/// - original type
/// - variable / function on type access
/// - math operation
/// - comparing
/// - boolean negation
///
pub fn parse_ast_value_post_l(
    tokens: &Vec<LexerToken>,
    ind: &mut usize,
    original: DiagnosticResult<Box<ASTTreeNode>>,
    invoked_on_body: bool,
) -> DiagnosticResult<Box<ASTTreeNode>> {
    match &tokens[*ind].tok_type {
        LexerTokenType::Plus
        | LexerTokenType::Minus
        | LexerTokenType::Asterisk
        | LexerTokenType::Divide => {
            let o = &original?;
            let k = Box::new(ASTTreeNode::clone(o.as_ref()));

            return Ok(parse_math_operation(tokens, ind, k, invoked_on_body)?);
        }

        LexerTokenType::ArrayOpen => {
            let k = parse_array_access(tokens, ind, original?)?;

            return parse_ast_value_post_l(tokens, ind, Ok(k), invoked_on_body);
        }

        LexerTokenType::EqualSign => {
            if tokens[*ind + 1].tok_type == LexerTokenType::EqualSign {
                return parse_ast_compare(tokens, ind, original.clone()?);
            }

            *ind += 1;

            if let Ok(v) = original.as_ref() {
                if let ASTTreeNodeKind::ArrayIndexAccess { val, index } = &v.kind {
                    let start = original.clone()?.start.clone();

                    let right_val = parse_ast_value(tokens, ind)?;

                    let end = right_val.end.clone();

                    let kind = ASTTreeNodeKind::ArrayIndexModifiy {
                        array: val.clone(),
                        index: index.clone(),
                        val: right_val,
                    };

                    return Ok(Box::new(ASTTreeNode::new(kind, start, end)));
                }
            }

            let start = original.clone()?.start.clone();

            let right_val = parse_ast_value(tokens, ind)?;

            let end = right_val.end.clone();

            let kind = ASTTreeNodeKind::VarValueChange {
                var: original?,
                value: right_val,
            };
            return Ok(Box::new(ASTTreeNode::new(kind, start, end)));
        }

        LexerTokenType::ExclamationMark
        | LexerTokenType::AngelBracketOpen
        | LexerTokenType::AngelBracketClose => {
            return parse_ast_compare(tokens, ind, original.clone()?);
        }

        _ => return original,
    }
}

pub fn parse_ast_condition_if_statement_value(
    tokens: &Vec<LexerToken>,
    ind: &mut usize,
) -> DiagnosticResult<Box<ASTTreeNode>> {
    match &tokens[*ind].tok_type {
        LexerTokenType::Unwrap | LexerTokenType::UnwrapUnsafe => {
            parse_unwrap_condition(tokens, ind)
        }
        _ => parse_ast_value(tokens, ind),
    }
}

/// Parses an AST node that can and WILL be intrepreted as a value
///
/// # Parsing Layout
/// The `parse_ast_value` function only parses the pre side of the expression (noted R) if expression is:
/// `R (pre) expression L (post) expression`
///
/// This layout allows us to seperate parsing from things like variable references, functions calls or even literals and
/// treat them as the same while parsing other elements such as math operations or conditions!
///
/// This function will call `parse_ast_value_post_l` to parse the L part of the expression.
///
/// # Recognized Nodes
/// Possible nodes recognized as values include:
/// - Function calls
/// - Variable refs
/// - Math operation results (both with or without value changing)
/// - Boolean negation result
/// - Boolean compare result
pub fn parse_ast_value(
    tokens: &Vec<LexerToken>,
    ind: &mut usize,
) -> DiagnosticResult<Box<ASTTreeNode>> {
    match &tokens[*ind].tok_type {
        LexerTokenType::ExclamationMark => {
            *ind += 1;
            let ast = parse_ast_value(tokens, ind)?;

            if ast.kind.is_function_call() || ast.kind.is_var_access() {
                let end = ast.end.clone();
                return Ok(Box::new(ASTTreeNode::new(
                    ASTTreeNodeKind::BooleanBasedConditionMember {
                        val: ast,
                        negate: true,
                    },
                    Position::clone(&tokens[*ind].pos),
                    end,
                )));
            }

            return Err(make_expected_simple_error(
                &tokens[*ind],
                &"function call or variable access".to_string(),
                &ast,
            )
            .into());
        }

        LexerTokenType::ArrayOpen => {
            return parse_ast_array_init(tokens, ind);
        }

        LexerTokenType::Asterisk => return parse_ast_pointer(tokens, ind),
        LexerTokenType::Ampersand => return parse_ast_reference(tokens, ind),

        LexerTokenType::IntLit(_, _) => {
            let int = parse_integer_literal(tokens, ind);
            return parse_ast_value_post_l(tokens, ind, int, false);
        }

        LexerTokenType::StringLit(_) => {
            let str = parse_string_literal(tokens, ind);
            return parse_ast_value_post_l(tokens, ind, str, false);
        }

        LexerTokenType::BracketOpen => {
            return parse_struct_initialize(tokens, ind);
        }

        LexerTokenType::Keyword(str, _) => {
            if tokens[*ind + 1].tok_type == LexerTokenType::ParenOpen {
                let call = parse_function_call(tokens, ind);
                return parse_ast_value_post_l(tokens, ind, call, false);
            }

            let n = Ok(make_node!(
                ASTTreeNodeKind::VariableReference(HashedString::new(str.clone())),
                &tokens[*ind],
                &tokens[*ind]
            ));

            *ind += 1;

            let chain = parse_ast_value_dotacess(tokens, ind, n);

            return parse_ast_value_post_l(tokens, ind, chain, false);
        }

        LexerTokenType::Unwrap | LexerTokenType::UnwrapUnsafe => parse_unwrap_value(tokens, ind),

        _ => {
            return Err(make_unexpected_simple_error(&tokens[*ind], &tokens[*ind].tok_type).into());
        }
    }
}

pub fn parse_ast_array_init(
    tokens: &Vec<LexerToken>,
    ind: &mut usize,
) -> DiagnosticResult<Box<ASTTreeNode>> {
    let start = tokens[*ind].pos.clone();
    *ind += 1;

    if tokens[*ind + 1].tok_type == LexerTokenType::Comma {
        let val = parse_ast_value(tokens, ind)?;

        tokens[*ind].expects(LexerTokenType::Comma)?;
        *ind += 1;

        let count = tokens[*ind].expects_int_lit()?;

        return Ok(Box::new(ASTTreeNode::new(
            ASTTreeNodeKind::ArrayVariableInitializerValueSameValue {
                size: count.0 as usize,
                v: val,
            },
            start,
            tokens[*ind].get_end_pos(),
        )));
    }

    let mut vals = vec![];

    loop {
        let val = parse_ast_value(tokens, ind)?;

        if tokens[*ind].tok_type == LexerTokenType::ArrayClose {
            break;
        }

        tokens[*ind].expects(LexerTokenType::Comma)?;
        *ind += 1;

        vals.push(val);
    }

    return Ok(Box::new(ASTTreeNode::new(
        ASTTreeNodeKind::ArrayVariableInitializerValue { vals },
        start,
        tokens[*ind].get_end_pos(),
    )));
}

pub fn parse_ast_pointer(
    tokens: &Vec<LexerToken>,
    ind: &mut usize,
) -> DiagnosticResult<Box<ASTTreeNode>> {
    let start = tokens[*ind].pos.clone();

    tokens[*ind].expects(LexerTokenType::Asterisk)?;
    *ind += 1;

    let value = parse_ast_value(tokens, ind)?;

    return Ok(Box::new(ASTTreeNode::new(
        ASTTreeNodeKind::PointerGrab(value),
        start,
        tokens[*ind].get_end_pos(),
    )));
}

pub fn parse_ast_reference(
    tokens: &Vec<LexerToken>,
    ind: &mut usize,
) -> DiagnosticResult<Box<ASTTreeNode>> {
    let start = tokens[*ind].pos.clone();

    tokens[*ind].expects(LexerTokenType::Ampersand)?;
    *ind += 1;

    let value = parse_ast_value(tokens, ind)?;

    return Ok(Box::new(ASTTreeNode::new(
        ASTTreeNodeKind::ReferenceGrab(value),
        start,
        tokens[*ind].get_end_pos(),
    )));
}
