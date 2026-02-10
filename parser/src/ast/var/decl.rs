use commons::err::PositionedResult;
use lexer::token::{LexerToken, LexerTokenType};
use utils::hash::WithHash;

use crate::{ast::{parse_ast_value, tree::ASTTreeNode}};

pub fn parse_variable_declaration(tokens: &Vec<LexerToken>, ind: &mut usize) -> PositionedResult<Box<ASTTreeNode>> {
	*ind += 1;

	let typeName = tokens[*ind].expects_keyword()?;

	*ind += 1;

	let varName = tokens[*ind].expects_keyword()?;

	*ind += 1;

	let mut val: Option<Box<ASTTreeNode>> = None;

	if tokens[*ind].tok_type == LexerTokenType::EQUAL_SIGN {
		*ind += 1;
		
		val = Some(parse_ast_value(tokens, ind)?);
	}

	return Ok(Box::new(ASTTreeNode::VarDeclaration { varName: WithHash::new(varName.0), varType: typeName.1, value: val }));
}