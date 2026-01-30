//!
//! Module containing the core lexer algorithm
//! 

use std::{fs, hash::{DefaultHasher, Hash, Hasher}, io::Error};

use crate::{LexerParseResult, LexerParsingError, token::LexerToken};

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
    
    while i < contents.len() {
        let c: char = contents.chars().nth(i).unwrap();

        if c.is_numeric() {
            tokens.push(parse_number_token(&contents, &mut i)?);
            continue;
        }

        if c == '"' {
            tokens.push(parse_string_token(&contents, &mut i));
            continue;
        }

        if c.is_alphabetic() {
            tokens.push(parse_keyword(&contents, &mut i));
            continue;
        }

        i += c.len_utf8();

        match c {
            '{' => tokens.push(LexerToken::BRACKET_OPEN),
            '}' => tokens.push(LexerToken::BRACKET_CLOSE),
            '(' => tokens.push(LexerToken::PAREN_OPEN),
            ')' => tokens.push(LexerToken::PAREN_CLOSE),
            '[' => tokens.push(LexerToken::ARRAY_OPEN),
            ']' => tokens.push(LexerToken::ARRAY_CLOSE),
            '=' => tokens.push(LexerToken::EQUAL_SIGN),
            ',' => tokens.push(LexerToken::COMMA),
            ';' => tokens.push(LexerToken::SEMICOLON),
            '.' => tokens.push(LexerToken::DOT),
            '<' => tokens.push(LexerToken::ANGEL_BRACKET_OPEN),
            '>' => tokens.push(LexerToken::ANGEL_BRACKET_CLOSE),
            _ => continue
        }

    }

    tokens.push(LexerToken::END_OF_FILE);

    Ok(tokens)
}

fn parse_number_token(str: &String, ind: &mut usize) -> LexerParseResult<LexerToken> {
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

    return Ok(LexerToken::INT_LIT(num));
}

fn parse_string_token(str: &String, ind: &mut usize) -> LexerToken {
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
    
    return LexerToken::STRING_LIT(slice.to_string());
}

fn parse_keyword(str: &String, ind: &mut usize) -> LexerToken {
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

    match hash {
        FUNC_KEYWORD_HASH => return LexerToken::FUNCTION,
        RET_KEYWORD_HASH => return LexerToken::RETURN,
		STRUCT_KEYWORD_HASH => return LexerToken::STRUCT,
		LAYOUT_KEYWORD_HASH => return LexerToken::LAYOUT,
		LAY_KEYWORD_HASH => return LexerToken::LAY,
		TRUE_KEYWORD_HASH => return LexerToken::TRUE,
		FALSE_KEYWORD_HASH => return LexerToken::FALSE,
		VAR_KEYWORD_HASH => return LexerToken::VAR,
		IF_KEYWORD_HASH => return LexerToken::IF,
		ELSE_KEYWORD_HASH => return LexerToken::ELSE,
		WHILE_KEYWORD_HASH => return LexerToken::WHILE,
		FOR_KEYWORD_HASH => return LexerToken::FOR,

        _ => {
            return LexerToken::KEYWORD(slice.to_string(), hash);
        }
    }
}