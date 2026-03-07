use compiler_errors::errs::CompilerResult;
use lexer::token::{LexerToken, LexerTokenType};
use compiler_utils::hash::{HashedString};

use ast::{tree::{ASTTreeNode, ASTTreeNodeKind}};

use crate::{types::parse_type, value::parse_ast_value};

pub fn parse_variable_declaration(tokens: &Vec<LexerToken>, ind: &mut usize) -> CompilerResult<Box<ASTTreeNode>> {
	let start= tokens[*ind].pos.clone();

	*ind += 1;

	let t = parse_type(tokens, ind)?;

	*ind += 1;

	let var_name = tokens[*ind].expects_keyword()?;

	*ind += 1;

	let mut val: Option<Box<ASTTreeNode>> = None;
	let end;

	if tokens[*ind].tok_type == LexerTokenType::EqualSign {
		*ind += 1;
		
		val = Some(parse_ast_value(tokens, ind)?);
		end = val.as_ref().unwrap().end.clone();
	} else {
		end = tokens[*ind - 1].get_end_pos().clone();
	}

	return Ok(Box::new(ASTTreeNode::new(ASTTreeNodeKind::VarDeclaration { var_name: HashedString::new(var_name.0), var_type: t, value: val }, start, end)));
}