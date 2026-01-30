use lexer::token::LexerToken;
use utils::hash::WithHash;

use crate::{ParserError, ParserResult, ast::{parse_ast_node, tree::ASTTreeNode}};

pub fn parse_variable_declaration(tokens: &Vec<LexerToken>, ind: &mut usize) -> ParserResult<Box<ASTTreeNode>> {
	*ind += 1;

	let typeName = match tokens[*ind].as_keyword() {
		Ok(val) => val,
		Err(e) => return Err(ParserError::new(String::from("Type name isn't a keyword"), 0))
	};

	*ind += 1;

	let varName = match tokens[*ind].as_keyword() {
		Ok(val) => val,
		Err(e) => return Err(ParserError::new(String::from("Var name isn't a keyword!"), 0))
	};

	*ind += 1;

	let mut val: Option<Box<ASTTreeNode>> = None;

	if tokens[*ind] == LexerToken::EQUAL_SIGN {
		*ind += 1;
		
		val = match parse_ast_node(tokens, ind) {
			Ok(v) => Some(v),
			Err(e) => None
		};
	}

	return Ok(Box::new(ASTTreeNode::VarDeclaration { varName: WithHash::new(varName.0), varType: typeName.1, value: val }));
}