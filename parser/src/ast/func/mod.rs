use std::fmt::Debug;

use commons::err::PositionedResult;
use lexer::token::{LexerToken, LexerTokenType};

use crate::{ParserError, ParserResult, ast::{parse_ast_node_in_body, tree::{ASTTreeNode, FunctionDeclarationArgument}}};

pub mod decl;
pub mod call;

pub fn parse_node_body(tokens: &Vec<LexerToken>, ind: &mut usize) -> PositionedResult<Vec<Box<ASTTreeNode>>> {
    *ind += 1;

    let mut tok: &LexerToken = &tokens[*ind];
    let mut body: Vec<Box<ASTTreeNode>> = Vec::new();

    while tok.tok_type != LexerTokenType::END_OF_FILE && tok.tok_type != LexerTokenType::BRACKET_CLOSE {
        let n = parse_ast_node_in_body(tokens, ind)?;

        body.push(n);

        //println!("Func index: {}", *ind);

        tok = &tokens[*ind];
    }

    return Ok(body);
}


/// Parses functions arguments.
pub fn parse_function_arguments(tokens: &Vec<LexerToken>, ind: &mut usize) -> PositionedResult<Vec<FunctionDeclarationArgument>> {
	*ind += 1;

	let mut args: Vec<FunctionDeclarationArgument> = Vec::new();
	
	while *ind < tokens.len() && tokens[*ind].is_keyword() {
		let varType = tokens[*ind].expects_keyword()?;

		*ind += 1;
		let varName = tokens[*ind].expects_keyword()?;

		args.push(FunctionDeclarationArgument::new(varName.0, varType.1));

		*ind += 1;

		if tokens[*ind].tok_type == LexerTokenType::PAREN_CLOSE {
			break;
		}

		tokens[*ind].expects(LexerTokenType::COMMA)?;

		*ind += 1;
	}

	tokens[*ind].expects(LexerTokenType::PAREN_CLOSE)?;

	Ok(args)
}