use commons::err::PositionedResult;
use lexer::token::LexerToken;

use crate::ast::{parse_ast_value, tree::ASTTreeNode};

pub fn parse_math_operation(tokens: &Vec<LexerToken>, ind: &mut usize, original: Box<ASTTreeNode>, restricts_to_assigns: bool) -> PositionedResult<Box<ASTTreeNode>> {
	let oper = tokens[*ind].expects_math_operator()?;

	if !oper.1 && restricts_to_assigns {
		return Err(tokens[*ind].make_err("Using math operation without assigments is forbidden here!"));
	}

	*ind += 1;

	let rightMember = parse_ast_value(tokens, ind)?;

	return Ok(Box::new(ASTTreeNode::MathResult { lval: original, rval: rightMember, operator: oper.0, assigns: oper.1 }))
}