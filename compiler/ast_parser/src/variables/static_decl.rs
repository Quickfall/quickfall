use diagnostics::DiagnosticResult;
use lexer::token::{LexerToken, LexerTokenType};
use compiler_utils::hash::{HashedString};

use ast::{tree::{ASTTreeNode, ASTTreeNodeKind}};

use crate::{types::parse_type, value::parse_ast_value};

pub fn parse_static_variable_declaration(tokens: &Vec<LexerToken>, ind: &mut usize) -> DiagnosticResult<Box<ASTTreeNode>> {
	let start = tokens[*ind].pos.clone();

	*ind += 1;

	let var_type = parse_type(tokens, ind)?;
	*ind += 1;

	let var_name = tokens[*ind].expects_keyword()?;
	*ind += 1;

	tokens[*ind].expects(LexerTokenType::EqualSign)?;
	*ind += 1;

	let val = parse_ast_value(tokens, ind)?;
	let end = val.end.clone();

	return Ok(Box::new(ASTTreeNode::new(ASTTreeNodeKind::StaticVariableDeclaration { name: HashedString::new(var_name.0), val, var_type }, start, end)))
}
