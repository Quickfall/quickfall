use commons::err::PositionedResult;
use lexer::token::{LexerToken, LexerTokenType};
use utils::hash::WithHash;

use crate::ast::{parse_ast_value, tree::ASTTreeNode, var};

pub fn parse_static_function_declaration(tokens: &Vec<LexerToken>, ind: &mut usize) -> PositionedResult<Box<ASTTreeNode>> {
	*ind += 1;

	let var_type = tokens[*ind].expects_keyword()?;
	*ind += 1;

	let var_name = tokens[*ind].expects_keyword()?;
	*ind += 1;

	tokens[*ind].expects(LexerTokenType::EqualSign)?;
	*ind += 1;

	let val = parse_ast_value(tokens, ind)?;

	return Ok(Box::new(ASTTreeNode::StaticVariableDeclaration { name: WithHash::new(var_name.0), val, var_type: var_type.1 }))
}