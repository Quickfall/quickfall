use commons::err::PositionedResult;
use lexer::token::{LexerToken, LexerTokenType};
use utils::hash::WithHash;

use crate::{ast::{parse_ast_value, tree::ASTTreeNode}};

pub fn parse_variable_declaration(tokens: &Vec<LexerToken>, ind: &mut usize) -> PositionedResult<Box<ASTTreeNode>> {
	*ind += 1;

	let type_name = tokens[*ind].expects_keyword()?;

	*ind += 1;

	let var_name = tokens[*ind].expects_keyword()?;

	*ind += 1;

	let mut val: Option<Box<ASTTreeNode>> = None;

	if tokens[*ind].tok_type == LexerTokenType::EqualSign {
		*ind += 1;
		
		val = Some(parse_ast_value(tokens, ind)?);
	}

	return Ok(Box::new(ASTTreeNode::VarDeclaration { var_name: WithHash::new(var_name.0), var_type: type_name.1, value: val }));
}