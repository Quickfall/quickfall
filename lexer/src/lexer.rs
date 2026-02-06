//!
//! Module containing the core lexer algorithm
//! 

use std::{fs, hash::{DefaultHasher, Hash, Hasher}, io::Error};

use commons::Position;

use crate::{LexerParseResult, LexerParsingError, token::{LexerToken, LexerTokenType}, toks::math::MathOperator};

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

/// Parses a file into a set of lexer tokens.
/// 
/// # Examples
/// 
/// ```
/// let result: LexerParseResult<Vec<LexerToken>> = lexer_parse_file("test_file.qf").expect("Lexer didn't work");
/// ```
pub fn lexer_parse_file(file_path: &String) -> LexerParseResult<Vec<LexerToken>> {
    let contents: String = match fs::read_to_string(file_path) {
        Ok(v) => v,
        Err(_) => return Err(LexerParsingError::new("File couldn't be read!".to_string(), 0)),
    };

    let mut tokens: Vec<LexerToken> = Vec::new();

    let mut i: usize = 0;

	let mut line: usize = 1;

	let mut last_line_break: usize = 0;
    
    while i < contents.len() {
        let c: char = contents.chars().nth(i).unwrap();
		
		if c == '\n' {
			i += c.len_utf8();
			last_line_break = i;
			line += 1;
			continue;
		}

        if c.is_numeric() {
			let col = i - last_line_break + 1;
            tokens.push(parse_number_token(&contents, &mut i, Position::new(file_path.to_string(), line, col))?);
            continue;
        }

        if c == '"' {
			let col = i - last_line_break + 1;

            tokens.push(parse_string_token(&contents, &mut i, Position::new(file_path.to_string(), line, col)));
            continue;
        }

        if c.is_alphabetic() {
			let col = i - last_line_break + 1;

            tokens.push(parse_keyword(&contents, &mut i, Position::new(file_path.to_string(), line, col)));
            continue;
        }

        i += c.len_utf8();


		let col = i - last_line_break + 1;

		let pos = Position::new(file_path.to_string(), line, col);

        match c {
            '{' => tokens.push(LexerToken::make_single_sized(pos, LexerTokenType::BRACKET_OPEN)),
            '}' => tokens.push(LexerToken::make_single_sized(pos, LexerTokenType::BRACKET_CLOSE)),
            '(' => tokens.push(LexerToken::make_single_sized(pos, LexerTokenType::PAREN_OPEN)),
            ')' => tokens.push(LexerToken::make_single_sized(pos, LexerTokenType::PAREN_CLOSE)),
            '[' => tokens.push(LexerToken::make_single_sized(pos, LexerTokenType::ARRAY_OPEN)),
            ']' => tokens.push(LexerToken::make_single_sized(pos, LexerTokenType::ARRAY_CLOSE)),
            '=' => tokens.push(LexerToken::make_single_sized(pos, LexerTokenType::EQUAL_SIGN)),
            ',' => tokens.push(LexerToken::make_single_sized(pos, LexerTokenType::COMMA)),
            '.' => tokens.push(LexerToken::make_single_sized(pos, LexerTokenType::DOT)),
			'!' => tokens.push(LexerToken::make_single_sized(pos, LexerTokenType::EXCLAMATION_MARK)),
			'&' => tokens.push(LexerToken::make_single_sized(pos, LexerTokenType::AMPERSAND)),
            '<' => tokens.push(LexerToken::make_single_sized(pos, LexerTokenType::ANGEL_BRACKET_OPEN)),
            '>' => tokens.push(LexerToken::make_single_sized(pos, LexerTokenType::ANGEL_BRACKET_CLOSE)),
			'+' | '-' | '*' | '/' => tokens.push(parse_math_operator(&contents, &mut i, pos)?),
            _ => continue
        }

    }

    tokens.push(LexerToken::make_single_sized(Position::new(file_path.to_string(), line, i - last_line_break + 1), LexerTokenType::END_OF_FILE));

    Ok(tokens)
}

fn parse_math_operator(contents: &String, ind: &mut usize, start_pos: Position) -> LexerParseResult<LexerToken> {
	let operatorChar = contents.chars().nth(*ind).unwrap();

	let operator = match operatorChar {
		'+' => MathOperator::ADD,
		'-' => MathOperator::SUBSTRACT,
		'*' => MathOperator::MULTIPLY,
		'/' => MathOperator::DIVIDE,
		_ => return Err(LexerParsingError::new(String::from("Invalid operator sign!"), 0))
	};

	*ind += 1;

	let assigns = match contents.chars().nth(*ind + 1) {
		Some(v) => v == '=',
		None => false
	};

	let mut incrementCount = 1;

	if assigns {
		incrementCount += 1;
	}

	let end = start_pos.increment_by(incrementCount);

	return Ok(LexerToken::new(start_pos, end, LexerTokenType::MATH_OPERATOR(operator, assigns)));

}

fn parse_number_token(str: &String, ind: &mut usize, start_pos: Position) -> LexerParseResult<LexerToken> {
    let start = *ind + 1;
    let mut end: usize = start;
    
    for(i, c) in str[start..].char_indices() {
        if !c.is_ascii_digit() {
            break;
        }

        end = start + i + c.len_utf8();
    }

    let slice = &str[*ind..end];
    let num: i64 = match slice.parse() {
        Ok(v) => v,
        Err(_) => return Err(LexerParsingError::new("Couldn't parse int lit!".to_string(), *ind)),
    };

    *ind = end;

	let endpos = start_pos.increment_by(end - start);
    return Ok(LexerToken::new(start_pos, endpos, LexerTokenType::INT_LIT(num)));
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

    let slice = &str[*ind..end];

    *ind = end;
    
	let endpos: Position = start_pos.increment_by(end - start);
    return LexerToken::new(start_pos, endpos, LexerTokenType::STRING_LIT(slice.to_string()));
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
        FUNC_KEYWORD_HASH => LexerTokenType::FUNCTION,
        RET_KEYWORD_HASH => LexerTokenType::RETURN,
		STRUCT_KEYWORD_HASH => LexerTokenType::STRUCT,
		LAYOUT_KEYWORD_HASH => LexerTokenType::LAYOUT,
		LAY_KEYWORD_HASH => LexerTokenType::LAY,
		TRUE_KEYWORD_HASH => LexerTokenType::TRUE,
		FALSE_KEYWORD_HASH => LexerTokenType::FALSE,
		VAR_KEYWORD_HASH => LexerTokenType::VAR,
		IF_KEYWORD_HASH => LexerTokenType::IF,
		ELSE_KEYWORD_HASH => LexerTokenType::ELSE,
		WHILE_KEYWORD_HASH => LexerTokenType::WHILE,
		FOR_KEYWORD_HASH => LexerTokenType::FOR,
        _ => LexerTokenType::KEYWORD(slice.to_string(), hash)
    };

	let endpos: Position = start_pos.increment_by(end - start);
	return LexerToken::new(start_pos, endpos, token_type);
}