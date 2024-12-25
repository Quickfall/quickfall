/**
 * The lexer of Quickfall
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

#include "./lexer.h"
#include "./tokens.h"
#include "../parser/ast.h"

#include "../utils/hashes.h"
#include "../utils/logging.c"

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
	printf("  Initializing lexer...\n");

	result.tokens = malloc(sizeof(TOKEN) * 1024);
	if (!result.tokens) {
		printf("%sError: Failed to allocate token array%s\n", TEXT_RED, RESET);
		return result;
	}

	char c;
	int tokenCount = 0;

	for(int i = 0; i < size; ++i) {
		c = string[i];

		int buffLen = 32;
		char* buff = malloc(buffLen);
		if (!buff) {
			printf("%sError: Failed to allocate token buffer%s\n", TEXT_RED, RESET);
			continue;
		}
		buff[0] = '\0';

		if(c == ' ' || c == '\t' || c == '\n') {
			free(buff);
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
			++i;  // Skip the opening quote
			c = string[i];
			
			while(c != '\"' && i < size) {  // Add size check
				buff[strLen] = c;
				strLen++;
				
				++i;
				c = string[i];
			}
			buff[strLen] = '\0';  // Properly terminate the string
			
			pushToken(&result, STRING);
			result.tokens[result.size - 1].value = strdup(buff);  // Make a copy of the string
			free(buff);  // Free the temporary buffer

		} else if(isalpha(c)) {

			int keywordLen = 0;
			
			while(isalpha(c)) {
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


