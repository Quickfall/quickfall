//!
//! Module containing the core lexer algorithm
//! 

use std::fs;

use compiler_utils::{Position, hash};
use diagnostics::DiagnosticResult;
use diagnostics::builders::{make_unexpected_simple_error_outside};
use diagnostics::diagnostic::SpanPosition;

use crate::{token::{LexerToken, LexerTokenType}, toks::{comp::ComparingOperator, math::MathOperator}};

const SHADOWFUNC_KEYWORD_HASH: u64 = hash!("shadowfunc");
const FUNC_KEYWORD_HASH: u64 = hash!("func");
const RET_KEYWORD_HASH: u64 = hash!("ret");
const VAR_KEYWORD_HASH: u64 = hash!("var");
const STRUCT_KEYWORD_HASH: u64 = hash!("struct");
const LAYOUT_KEYWORD_HASH: u64 = hash!("layout");
const LAY_KEYWORD_HASH: u64 = hash!("lay");
const FALSE_KEYWORD_HASH: u64 = hash!("false");
const TRUE_KEYWORD_HASH: u64 = hash!("true");
const IF_KEYWORD_HASH: u64 = hash!("if");
const ELSE_KEYWORD_HASH: u64 = hash!("else");
const WHILE_KEYWORD_HASH: u64 = hash!("while");
const FOR_KEYWORD_HASH: u64 = hash!("for");
const STATIC_KEYWORD_HASH: u64 = hash!("static");
const THIS_KEYWORD_HASH: u64 = hash!("this");
const NEW_KEYWORD_HASH: u64 = hash!("new");
const UNWRAP_KEYWORD_HASH: u64 = hash!("unwrap");
const UNWRAP_UNSAFE_KEYWORD_HASH: u64 = hash!("unsafe_unwrap");

/// Parses a file into a set of lexer tokens.
/// 
/// # Examples
/// 
/// ```
/// let result: LexerParseResult<Vec<LexerToken>> = lexer_parse_file("test_file.qf").expect("Lexer didn't work");
/// ```
pub fn lexer_parse_file(file_path: &String) -> DiagnosticResult<Vec<LexerToken>> {
    let contents: String = match fs::read_to_string(file_path) {
        Ok(v) => v,
        Err(_) => panic!("Couldn't read the file"),
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

		if c == '/' {
			let cc = contents.chars().nth(i + 1).unwrap();
			if cc == '/' {
				let col = i - last_line_break;

				tokens.push(parse_comment(&contents, &mut i, Position::new(file_path.to_string(), line, col))?);
				continue;
			} else if cc == '.' {
				let col = i - last_line_break;

				tokens.push(parse_global_comment(&contents, &mut i, Position::new(file_path.to_string(), line, col))?);
				continue;
			}
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
			'*' => tokens.push(LexerToken::make_single_sized(pos, LexerTokenType::Asterisk)),
			':' => tokens.push(LexerToken::make_single_sized(pos, LexerTokenType::Collon)),
			_ => continue
        }

    }

    tokens.push(LexerToken::make_single_sized(Position::new(file_path.to_string(), line, i - last_line_break + 1), LexerTokenType::EndOfFile));

    Ok(tokens)
}

fn parse_comment(contents: &String, ind: &mut usize, start_pos: Position) -> DiagnosticResult<LexerToken> {
	*ind += 2;

	let start = *ind;
	let mut end = start;

	for (i, c) in contents[start..].char_indices() {
		if c == '\n' || c == '\0' {
			end = start + i + c.len_utf8();
			break;
		}

		end = start + i + c.len_utf16();
	}

	let slice = &contents[*ind + 1..end - 1];

	*ind = end;

	return Ok(LexerToken::new(start_pos, end - start, LexerTokenType::Comment(slice.to_string())))
}

fn parse_global_comment(contents: &String, ind: &mut usize, start_pos: Position) -> DiagnosticResult<LexerToken> {
	*ind += 2;

	let start = *ind;
	let mut end = start;

	for (i, c) in contents[start..].char_indices() {
		if c == '\n' || c == '\0' {
			end = start + i + c.len_utf8();
			break;
		}

		end = start + i + c.len_utf16();
	}

	let slice = &contents[*ind + 1..end - 1];

	*ind = end;

	return Ok(LexerToken::new(start_pos, end - start, LexerTokenType::GlobalComment(slice.to_string())))
}

fn parse_math_operator(contents: &String, ind: &mut usize, start_pos: Position) -> DiagnosticResult<LexerToken> {
	let operator_char = contents.chars().nth(*ind).unwrap();

	let operator = match operator_char {
		'+' => MathOperator::ADD,
		'-' => MathOperator::SUBSTRACT,
		'*' => MathOperator::MULTIPLY,
		'/' => MathOperator::DIVIDE,
		_ => return Err(make_unexpected_simple_error_outside(&operator_char, SpanPosition::from_pos(start_pos.clone(), start_pos.col + 1)).into())
	};

	*ind += 1;

	if contents.chars().nth(*ind).unwrap() != '=' {
		return Ok(LexerToken::make_single_sized(start_pos, LexerTokenType::Asterisk));
	}

	let assigns = match contents.chars().nth(*ind) {
		Some(v) => {
			if v != ' ' && v != '=' {
				return Err(make_unexpected_simple_error_outside(&v, SpanPosition::from_pos(start_pos.clone(), start_pos.col + 2).into()).into())
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

fn parse_number_token(str: &String, ind: &mut usize, start_pos: Position) -> DiagnosticResult<LexerToken> {
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
		Err(_) => return Err(make_unexpected_simple_error_outside(&slice, SpanPosition::from_pos(start_pos.clone(), start_pos.col + (end - start))).into())
    };

    *ind = end;

	let mut hash = hash!("s64"); // s64

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

    let slice = &str[start..end];
    
	let hash = hash!(slice);

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
		THIS_KEYWORD_HASH => LexerTokenType::This,
		NEW_KEYWORD_HASH => LexerTokenType::New,
		UNWRAP_KEYWORD_HASH => LexerTokenType::Unwrap,
		UNWRAP_UNSAFE_KEYWORD_HASH => LexerTokenType::UnwrapUnsafe,
        _ => LexerTokenType::Keyword(slice.to_string(), hash)
    };

	return LexerToken::new(start_pos, end - start, token_type);
}