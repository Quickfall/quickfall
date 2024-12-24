/**
 * The parser of Quickfall.
 */

#include <stdio.h>

#include "../lexer/tokens.h"
#include "../lexer/lexer.h"

#include "./ast.h"

#include "./asts/variables.h"
#include "./asts/functions.h"

/**
 * Parses the lexer tokens into nodes starting from the index.
 * @param result the LexerResult provided by the lexer.
 * @param index the starting index.
 */
AST_NODE* parseNodes(LEXER_RESULT result, int index, AST_NODE_TYPE type) {
	AST_NODE* root = createASTNode(type);
	AST_NODE* current = root;

	for(; index <= result.size; ++index) {
		TOKEN t = result.tokens[index];

		AST_NODE* node = NULL;

		switch(t.type) {
			case BRACKETS_CLOSE:
				if(type == AST_FUNCTION_ROOT) {
					root->endingIndex = index;
					return root;
				}
				break;
			case FUNCTION:
				node = parseFunctionDeclaration(result, index + 1);
				if(node != NULL) {
					current->next = node;
					current = node;
					index = node->endingIndex;
				}

				break;
			case VAR:
				node = parseVariableDeclaration(result, index);
				if(node != NULL) {
					current->next = node;
					current = node;
					index = node->endingIndex;
				}
				break;
			case ASM_FUNCTION:
				node = parseASMFunctionDeclaration(result, index);

				if(node != NULL) {
					current->next = node;
					current = node;
					index = node->endingIndex;
				}
				break;
			case KEYWORD:
				if(result.tokens[index + 1].type == PAREN_OPEN) {
					node = parseFunctionInvoke(result, index);

					if(node != NULL) {
						current->next = node;
						current = node;
						index = node->endingIndex;
					}
				}
				if(result.tokens[index + 1].type == KEYWORD && result.tokens[index + 2].type == DECLARE) {
					node = parseVariableDeclaration(result, index);
					if(node != NULL) {
						current->next = node;
						current = node;
						index = node->endingIndex;
					}
				}

		}
	}

	return root;
}
