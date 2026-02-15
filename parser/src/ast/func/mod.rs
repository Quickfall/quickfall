//! AST parsing for function related elements (function declarations, arguments, calls, ...)

use commons::err::PositionedResult;
use lexer::token::{LexerToken, LexerTokenType};

use crate::ast::{parse_ast_node_in_body, parse_ast_value, tree::{ASTTreeNode, FunctionDeclarationArgument}};

pub mod decl;
pub mod call;

pub fn parse_node_body(tokens: &Vec<LexerToken>, ind: &mut usize) -> PositionedResult<Vec<Box<ASTTreeNode>>> {
    *ind += 1;

    let mut tok: &LexerToken = &tokens[*ind];
    let mut body: Vec<Box<ASTTreeNode>> = Vec::new();

    while tok.tok_type != LexerTokenType::EndOfFile && tok.tok_type != LexerTokenType::BracketClose {
        let n = parse_ast_node_in_body(tokens, ind)?;

        body.push(n);

        tok = &tokens[*ind];
    }

	*ind += 1;

    return Ok(body);
}


/// Parses functions arguments.
pub fn parse_function_arguments(tokens: &Vec<LexerToken>, ind: &mut usize) -> PositionedResult<Vec<FunctionDeclarationArgument>> {
	*ind += 1;

	let mut args: Vec<FunctionDeclarationArgument> = Vec::new();
	
	while *ind < tokens.len() && tokens[*ind].is_keyword() {
		let var_type = tokens[*ind].expects_keyword()?;

		*ind += 1;
		let var_name = tokens[*ind].expects_keyword()?;

		args.push(FunctionDeclarationArgument::new(var_name.0, var_type.1));

		*ind += 1;

		if tokens[*ind].tok_type == LexerTokenType::ParenClose {
			break;
		}

		tokens[*ind].expects(LexerTokenType::Comma)?;

		*ind += 1;
	}

	tokens[*ind].expects(LexerTokenType::ParenClose)?;

	Ok(args)
}

pub fn parse_function_return_statement(tokens: &Vec<LexerToken>, ind: &mut usize) -> PositionedResult<Box<ASTTreeNode>> {
	*ind += 1;

	let val = parse_ast_value(tokens, ind)?;

	return Ok(Box::new(ASTTreeNode::ReturnStatement { val: Some(val) }))
}