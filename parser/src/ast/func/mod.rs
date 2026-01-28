use lexer::token::LexerToken;

use crate::{ParserError, ParserResult, ast::{parse_ast_node, tree::{ASTTreeNode, FunctionDeclarationArgument}}};

pub mod decl;

pub fn parse_node_body(tokens: &Vec<LexerToken>, ind: &mut usize) -> ParserResult<Vec<Box<ASTTreeNode>>> {
    *ind += 1;

    let mut tok: &LexerToken = &tokens[*ind];
    let mut body: Vec<Box<ASTTreeNode>> = Vec::new();

    while tok != &LexerToken::END_OF_FILE && tok != &LexerToken::BRACKET_CLOSE {
        let n = match parse_ast_node(tokens, ind) {
			Ok(val) => val,
			Err(e) => return Err(e)
		};

        body.push(n);

        //println!("Func index: {}", *ind);

        tok = &tokens[*ind];
    }

    return Ok(body);
}


/// Parses functions arguments.
pub fn parse_function_arguments(tokens: &Vec<LexerToken>, ind: &mut usize) -> ParserResult<Vec<FunctionDeclarationArgument>> {
	*ind += 1;

	let mut args: Vec<FunctionDeclarationArgument> = Vec::new();
	
	while *ind < tokens.len() && tokens[*ind].is_keyword() {
		let varType = match tokens[*ind].as_keyword() {
			Ok(val) => val,
			Err(e) => return Err(ParserError::new(String::from("Malformed function arguments"), 0))
		};

		*ind += 1;
		let varName = match tokens[*ind].as_keyword() {
			Ok(val) => val,
			Err(e ) => return Err(ParserError::new(String::from("Malformed argument name"), 0))
		};

		args.push(FunctionDeclarationArgument::new(varName.0, varType.1));

		*ind += 1;
		if tokens[*ind] != LexerToken::SEMICOLON {
			return Err(ParserError::new(String::from("Arguments must be seperated with semicolons"), 0));
		}

		*ind += 1;
	}

	if *ind >= tokens.len() || tokens[*ind] != LexerToken::PAREN_CLOSE {
		return Err(ParserError::new(String::from("Arguments must end with a paren close!"), 0));
	}

	Ok(args)
}