//!
//! Module containing the core lexer algorithm
//! 

use std::{fs, hash::{DefaultHasher, Hash, Hasher}};

use commons::Position;
use errors::{IO_ERROR_READ, PARSE_INT, PARSE_OPERATOR, pos::BoundPosition };
use errors::{errs::{CompilerResult, ErrorKind, base::BaseError, normal::CompilerError}};

use crate::{token::{LexerToken, LexerTokenType}, toks::{comp::ComparingOperator, math::MathOperator}};

const SHADOWFUNC_KEYWORD_HASH: u64 = 8856473617513302734;
const FUNC_KEYWORD_HASH: u64 = 17439195341824537259;
const RET_KEYWORD_HASH: u64 = 9222097151127739705;
const VAR_KEYWORD_HASH: u64 = 10000921911505692860;
const STRUCT_KEYWORD_HASH: u64 = 9993791738993869954;
const LAYOUT_KEYWORD_HASH: u64 = 3186948275885044588;
const LAY_KEYWORD_HASH: u64 = 5285095872557529407;
const FALSE_KEYWORD_HASH: u64 = 8985926696363166359;
const TRUE_KEYWORD_HASH: u64 = 2326242343701258586;
const IF_KEYWORD_HASH: u64 = 14565880314940941169;
const ELSE_KEYWORD_HASH: u64 = 15870633062462684568;
const WHILE_KEYWORD_HASH: u64 = 10666076348663826897;
const FOR_KEYWORD_HASH: u64 = 8246706989536534387;
const STATIC_KEYWORD_HASH: u64 = 15057913784433987235;

/// Parses a file into a set of lexer tokens.
/// 
/// # Examples
/// 
/// ```
/// let result: LexerParseResult<Vec<LexerToken>> = lexer_parse_file("test_file.qf").expect("Lexer didn't work");
/// ```
pub fn lexer_parse_file(file_path: &String) -> CompilerResult<Vec<LexerToken>> {
    let contents: String = match fs::read_to_string(file_path) {
        Ok(v) => v,
        Err(_) => return Err(CompilerError::from_base_posless(BaseError::critical(IO_ERROR_READ!().to_string()))),
    };

    let mut tokens: Vec<LexerToken> = Vec::new();

    let mut i: usize = 0;

	let mut line: usize = 1;

	let mut last_line_break: usize = 0;
    
    while i < contents.len() {
        let c: char = contents.chars().nth(i).unwrap();
		
		if c == '\n' {
			i += 1;
			last_line_break = i;
			line += 1;
			continue;
		}

        if c.is_numeric() {
			let col = i - last_line_break;
            tokens.push(parse_number_token(&contents, &mut i, Position::new(file_path.to_string(), line, col))?);
            continue;
        }

        if c == '"' {
			let col = i - last_line_break;

            tokens.push(parse_string_token(&contents, &mut i, Position::new(file_path.to_string(), line, col)));
            continue;
        }

        if c.is_alphabetic() {
			let col = i - last_line_break;

            tokens.push(parse_keyword(&contents, &mut i, Position::new(file_path.to_string(), line, col)));
            continue;
        }

		if c == '+' || c == '-' || c == '*' || c == '/' {
			let col = i - last_line_break;

			tokens.push(parse_math_operator(&contents, &mut i, Position::new(file_path.to_string(), line, col))?);

			continue;
		}

		if c == '=' || c == '>' || c == '<' {
			let col = i - last_line_break;

			let parse = parse_comp_operator(&contents, &mut i, Position::new(file_path.to_string(), line, col));

			if parse.is_some() {
				tokens.push(parse.unwrap());
				continue;
			}

			i -= 2; // Try parsing operator as normal token.
		}

        i += 1;


		let col = i - last_line_break;

		let pos = Position::new(file_path.to_string(), line, col);

        match c {
            '{' => tokens.push(LexerToken::make_single_sized(pos, LexerTokenType::BracketOpen)),
            '}' => tokens.push(LexerToken::make_single_sized(pos, LexerTokenType::BracketClose)),
            '(' => tokens.push(LexerToken::make_single_sized(pos, LexerTokenType::ParenOpen)),
            ')' => tokens.push(LexerToken::make_single_sized(pos, LexerTokenType::ParenClose)),
            '[' => tokens.push(LexerToken::make_single_sized(pos, LexerTokenType::ArrayOpen)),
            ']' => tokens.push(LexerToken::make_single_sized(pos, LexerTokenType::ArrayClose)),
            '=' => tokens.push(LexerToken::make_single_sized(pos, LexerTokenType::EqualSign)),
            ',' => tokens.push(LexerToken::make_single_sized(pos, LexerTokenType::Comma)),
            '.' => tokens.push(LexerToken::make_single_sized(pos, LexerTokenType::Dot)),
			'!' => tokens.push(LexerToken::make_single_sized(pos, LexerTokenType::ExclamationMark)),
			'&' => tokens.push(LexerToken::make_single_sized(pos, LexerTokenType::Ampersand)),
            '<' => tokens.push(LexerToken::make_single_sized(pos, LexerTokenType::AngelBracketOpen)),
            '>' => tokens.push(LexerToken::make_single_sized(pos, LexerTokenType::AngelBracketClose)),
			_ => continue
        }

    }

    tokens.push(LexerToken::make_single_sized(Position::new(file_path.to_string(), line, i - last_line_break + 1), LexerTokenType::EndOfFile));

    Ok(tokens)
}

fn parse_math_operator(contents: &String, ind: &mut usize, start_pos: Position) -> CompilerResult<LexerToken> {
	let operator_char = contents.chars().nth(*ind).unwrap();

	let operator = match operator_char {
		'+' => MathOperator::ADD,
		'-' => MathOperator::SUBSTRACT,
		'*' => MathOperator::MULTIPLY,
		'/' => MathOperator::DIVIDE,
		_ => return Err(CompilerError::new(ErrorKind::Error, format!(PARSE_OPERATOR!(), operator_char.to_string()), BoundPosition::from_size(start_pos, 1)))
	};

	*ind += 1;

	let assigns = match contents.chars().nth(*ind) {
		Some(v) => {
			if v != ' ' && v != '=' {
				return Err(CompilerError::new(ErrorKind::Error, format!(PARSE_OPERATOR!(), contents.chars().nth(*ind).unwrap()), BoundPosition::from_size(start_pos, 2)));
			}

			v == '='
		}
		None => false
	};

	if assigns {
		*ind += 1;
	}

	let mut increment_count = 1;

	if assigns {
		increment_count += 1;
	}

	return Ok(LexerToken::new(start_pos, increment_count, LexerTokenType::MathOperator(operator, assigns)));

}

fn parse_comp_operator(contents: &String, ind: &mut usize, start_pos: Position) -> Option<LexerToken> {
	let first_char = contents.chars().nth(*ind).unwrap();
	*ind += 1;
	let second_char = contents.chars().nth(*ind).unwrap();

	*ind += 1;

	if second_char != '=' && second_char != ' ' {
		return None;
	}

	match first_char {
		'=' => {
			if second_char != '=' {
				return None;
			}

			return Some(LexerToken::new(start_pos, 2, LexerTokenType::ComparingOperator(ComparingOperator::Equal)));
		},

		'>' => {
			if second_char == '=' {
				return Some(LexerToken::new(start_pos, 2, LexerTokenType::ComparingOperator(ComparingOperator::HigherEqual)));
			}

			return Some(LexerToken::new(start_pos, 2, LexerTokenType::ComparingOperator(ComparingOperator::Higher)));
		},
		
		'<' => {
			if second_char == '=' {
				return Some(LexerToken::new(start_pos, 2, LexerTokenType::ComparingOperator(ComparingOperator::LowerEqual)));
			}

			return Some(LexerToken::new(start_pos, 2, LexerTokenType::ComparingOperator(ComparingOperator::Lower)));
		},

		_ => {
			return None;
		}
	}

}

fn parse_number_token(str: &String, ind: &mut usize, start_pos: Position) -> CompilerResult<LexerToken> {
    let start = *ind + 1;
    let mut end: usize = start;
    
    for(i, c) in str[start..].char_indices() {
        if !c.is_ascii_digit() {
            break;
        }

        end = start + i + c.len_utf8();
    }

    let slice = &str[*ind..end];
    let num: i128 = match slice.parse() {
        Ok(v) => v,
        Err(_) => return Err(CompilerError::new(ErrorKind::Error, PARSE_INT!().to_string(), BoundPosition::from_size(start_pos, end - start)))
    };

    *ind = end;

	let mut hash = 7572830400006405400; // s64

	let endpos = start_pos.increment_by(end - start);

	if str.chars().nth(*ind).unwrap() == '_' {
		*ind += 1;
		
		let tok = parse_keyword(str, ind, endpos.clone());
		let k = match tok.expects_keyword() {
			Ok(v) => v,
			Err(e) => return Err(e)
		};

		hash = k.1;
	}

    return Ok(LexerToken::new(start_pos, end - start, LexerTokenType::IntLit(num, hash)));
}

fn parse_string_token(str: &String, ind: &mut usize, start_pos: Position) -> LexerToken {
    let start = *ind + 1;
    let mut end: usize = start;

    for(i, c) in str[start..].char_indices() {
        if c == '"' {
            end = start + i + c.len_utf8();
            break;
        }

        end = start + i + c.len_utf8();
    }

    let slice = &str[*ind + 1..end - 1];

    *ind = end;
    
    return LexerToken::new(start_pos, end - start, LexerTokenType::StringLit(slice.to_string()));
}

fn parse_keyword(str: &String, ind: &mut usize, start_pos: Position) -> LexerToken {
    let start = *ind;
    let mut end: usize = start;
    
    for(i, c) in str[start..].char_indices() {
        if !c.is_alphabetic() && !c.is_numeric() && c != '_' && c != '-' {
            break;
        }

        end = start + i + c.len_utf8();
    }

    let mut hasher: DefaultHasher = DefaultHasher::new();

    let slice = &str[start..end];
    slice.hash(&mut hasher);

    let hash: u64 = hasher.finish();

    *ind = end;

    let token_type = match hash {
        FUNC_KEYWORD_HASH => LexerTokenType::Function,
		SHADOWFUNC_KEYWORD_HASH => LexerTokenType::ShadowFunction,
        RET_KEYWORD_HASH => LexerTokenType::Return,
		STRUCT_KEYWORD_HASH => LexerTokenType::Struct,
		LAYOUT_KEYWORD_HASH => LexerTokenType::Layout,
		LAY_KEYWORD_HASH => LexerTokenType::Lay,
		TRUE_KEYWORD_HASH => LexerTokenType::True,
		FALSE_KEYWORD_HASH => LexerTokenType::False,
		VAR_KEYWORD_HASH => LexerTokenType::Var,
		IF_KEYWORD_HASH => LexerTokenType::If,
		ELSE_KEYWORD_HASH => LexerTokenType::Else,
		WHILE_KEYWORD_HASH => LexerTokenType::While,
		FOR_KEYWORD_HASH => LexerTokenType::For,
		STATIC_KEYWORD_HASH => LexerTokenType::Static,
        _ => LexerTokenType::KEYWORD(slice.to_string(), hash)
    };

	return LexerToken::new(start_pos, end - start, token_type);
}