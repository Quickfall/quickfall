/**
 * Type-related AST parsing.
 */

#include "../ast.h"

#include "../../lexer/lexer.h"
#include "../../lexer/tokens.h"

AST_NODE* parseTypeDefinition(LEXER_RESULT result, int index) {
	AST_NODE* node = createASTNode(AST_TYPE_DECLARATION);

	index += 2;

	int stack = 0;

	for(; index < result.size; ++index) {
		TOKEN t = result.tokens[index];

		if(t.type == SEMICOLON) {
			continue;
		}
	}
}
