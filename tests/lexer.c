/**
 * Lexer test for Quickfall.
 */

#include <string.h>

#include "../src/lexer/lexer.h"

/**
 * Runs the lexer test.
 */
int runLexerTest(char* buff) {
	LEXER_RESULT result = runLexer(buff, strlen(buff));

	for(int i = 0; i < result.size; ++i) {
		printf("Token type: %d\n", result.tokens[i].type);
	}

	if(result.size == 0) return -1;

	return 0;
}
