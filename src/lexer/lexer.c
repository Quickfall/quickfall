/**
 * The lexer of Quickfall
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

#include "./lexer.h"
#include "./tokens.h"


/**
 * Sets the token type of the currently selected token in the LexerResult with the provided token type.
 */
void pushToken(LEXER_RESULT* result, TOKEN_TYPE type) {
    result->tokens[result->size].type = type;
    result->size++;
}

LEXER_RESULT runLexer(char* string, int size) {
	LEXER_RESULT result;
	result.size = 0;

	result.tokens = malloc(sizeof(TOKEN) * 1024);

	char c;

	for(int i = 0; i < size; ++i) {

		c = string[i];

		int buffLen = 32;
		char* buff = malloc(buffLen);
		buff[0] = '\0';

		if(c == ' ' || c == '\t' || c == '\n') {
			continue;
		} 
		else if(c == '\0') {
			break;
		}
		else if (isdigit(c)) {
			int numLen = 0;

			int foundX = 0;
			while(isdigit(c) || c == 'x' || (foundX == 1 && c != '\0' && c != ' ' && c != '\n')) {
				buff[numLen] = c;
				numLen++;

				if(c == 'x') foundX = 1;

				++i;
				c = string[i];
			}

			buff[numLen] = '\0';

			pushToken(&result, NUMBER);
			result.tokens[result.size - 1].value = buff;

		} else if (c == '\"') {
			int strLen = 0;

			++i;
			c = string[i];

			while(c != '\"' && c != '\0') {
				buff[strLen] = c;
				strLen++;

				++i;
				c = string[i];
			} 

			buff[strLen] = '\0';
			++i;

			pushToken(&result, STRING);
			result.tokens[result.size - 1].value = buff;

		} else if(isalpha(c)) {

			int keywordLen = 0;
			
			while(isalpha(c) || isdigit(c)) {
				buff[keywordLen] = c;
				buff[keywordLen + 1] = '\0';
				keywordLen++;

				++i;
				c = string[i];
			}

			if(keywordLen > 0) --i;

			if(strcmp(buff, "func") == 0) {
				pushToken(&result, FUNCTION);
			}
			else if(strcmp(buff, "asmf") == 0) {
				pushToken(&result, ASM_FUNCTION);
			}
			else if(strcmp(buff, "true") == 0 || strcmp(buff, "false") == 0) {
				pushToken(&result, BOOLEAN_VALUE);
				result.tokens[result.size - 1].value = buff;
			}
			else if(strcmp(buff, "null") == 0) {
				pushToken(&result, NU);
			}
			else if(strcmp(buff, "use") == 0) {
				pushToken(&result, USE);
			}
			else if(strcmp(buff, "var") == 0) {
				pushToken(&result, VAR);
			}
			else if(strcmp(buff, "int32") == 0) {
				pushToken(&result, TYPE_INT32);
			}
			else if(strcmp(buff, "int24") == 0) {
				pushToken(&result, TYPE_INT24);
			}
			else if(strcmp(buff, "int16") == 0) {
				pushToken(&result, TYPE_INT16);
			}
			else if(strcmp(buff, "int8") == 0) {
				pushToken(&result, TYPE_INT8);
			}
			else if(strcmp(buff, "bit") == 0) {
				pushToken(&result, TYPE_BIT);
			}
			else {
				pushToken(&result, KEYWORD);
				result.tokens[result.size - 1].value = buff;
			}
		}
		else {
			switch(c) {
				case '{': pushToken(&result, BRACKETS_OPEN); break;
				case '}': pushToken(&result, BRACKETS_CLOSE); break;
				case '(': pushToken(&result, PAREN_OPEN); break;
				case ')': pushToken(&result, PAREN_CLOSE); break;
				case '[': pushToken(&result, ARRAY_OPEN); break;
				case ']': pushToken(&result, ARRAY_CLOSE); break;
				case ';': pushToken(&result, SEMICOLON); break;
				case ',': pushToken(&result, COMMA); break;
				case '=': pushToken(&result, DECLARE); break;
				case '?': pushToken(&result, NONE); break;
				
				case '+':
				case '-':
				case '*':
				case '/':
				case '^':
					pushToken(&result, MATH_OP);
					result.tokens[result.size - 1].value[0] = c;
			}
		}
	}

	return result;
}


