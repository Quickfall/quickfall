use errors::errs::CompilerResult;
use lexer::token::{LexerToken, LexerTokenType};
use utils::hash::WithHash;

use crate::ast::{parse_ast_value, tree::{ASTTreeNode, ASTTreeNodeKind}};

pub fn parse_variable_declaration(tokens: &Vec<LexerToken>, ind: &mut usize) -> CompilerResult<Box<ASTTreeNode>> {
	let start= tokens[*ind].pos.clone();

	*ind += 1;

	let type_name = tokens[*ind].expects_keyword()?;

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

	return Ok(Box::new(ASTTreeNode::new(ASTTreeNodeKind::VarDeclaration { var_name: WithHash::new(var_name.0), var_type: type_name.1, value: val }, start, end)));
}