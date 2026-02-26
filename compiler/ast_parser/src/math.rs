use compiler_errors::{MATH_OP_NO_ASSIGN, errs::{CompilerResult, ErrorKind}};
use lexer::token::LexerToken;

use ast::{tree::{ASTTreeNode, ASTTreeNodeKind}};
use crate::value::parse_ast_value;

pub fn parse_math_operation(tokens: &Vec<LexerToken>, ind: &mut usize, original: Box<ASTTreeNode>, restricts_to_assigns: bool) -> CompilerResult<Box<ASTTreeNode>> {
	let oper = tokens[*ind].expects_math_operator()?;

	if !oper.1 && restricts_to_assigns {
		return Err(tokens[*ind].make_err(MATH_OP_NO_ASSIGN!().to_string(), ErrorKind::Error));
	}

	*ind += 1;

	let right_member = parse_ast_value(tokens, ind)?;

	let start = original.start.clone();
	let end = right_member.end.clone();

	return Ok(Box::new(ASTTreeNode::new(ASTTreeNodeKind::MathResult { lval: original, rval: right_member, operator: oper.0, assigns: oper.1 }, start, end)))
}