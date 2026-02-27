use compiler_errors::errs::CompilerResult;
use lexer::token::{LexerToken, LexerTokenType};
use compiler_utils::hash::WithHash;

use ast::{tree::{ASTTreeNode, ASTTreeNodeKind}};

use crate::value::parse_ast_value;

pub fn parse_static_variable_declaration(tokens: &Vec<LexerToken>, ind: &mut usize) -> CompilerResult<Box<ASTTreeNode>> {
	let start = tokens[*ind].pos.clone();

	*ind += 1;

	let var_type = tokens[*ind].expects_keyword()?;
	*ind += 1;

	let var_name = tokens[*ind].expects_keyword()?;
	*ind += 1;

	tokens[*ind].expects(LexerTokenType::EqualSign)?;
	*ind += 1;

	let val = parse_ast_value(tokens, ind)?;
	let end = val.end.clone();

	return Ok(Box::new(ASTTreeNode::new(ASTTreeNodeKind::StaticVariableDeclaration { name: WithHash::new(var_name.0), val, var_type: var_type.1 }, start, end)))
}
