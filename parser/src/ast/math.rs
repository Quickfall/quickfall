//! AST parsing for math related elements (math operations, ...)

use errors::{MATH_OP_NO_ASSIGN, errs::{CompilerResult}};
use lexer::token::LexerToken;

use crate::ast::{parse_ast_value, tree::ASTTreeNode};

pub fn parse_math_operation(tokens: &Vec<LexerToken>, ind: &mut usize, original: Box<ASTTreeNode>, restricts_to_assigns: bool) -> CompilerResult<Box<ASTTreeNode>> {
	let oper = tokens[*ind].expects_math_operator()?;

	if !oper.1 && restricts_to_assigns {
		return Err(tokens[*ind].make_err(MATH_OP_NO_ASSIGN!().to_string(), errors::errs::ErrorKind::Error));
	}

	*ind += 1;

	let right_member = parse_ast_value(tokens, ind)?;

	return Ok(Box::new(ASTTreeNode::MathResult { lval: original, rval: right_member, operator: oper.0, assigns: oper.1 }))
}
