/**
 * Variable-related AST parsing.
 */

#include "../../lexer/lexer.h"
#include "../../lexer/tokens.h"

#include "./math.h"

#include "../ast.h"

AST_NODE* parseVariableValue(struct LexerResult result, int index) {
	struct Token t = result.tokens[index];

	if(t.type == NUMBER || t.type == STRING || t.type == BOOLEAN_VALUE) {
		AST_NODE* node = createASTNode(AST_VARIABLE_VALUE);
		node->endingIndex = index + 1;
		node->left = createASTNode(AST_TYPE);

		switch(t.type) {
			case NUMBER:
				if(result.size >= index + 1 && result.tokens[index + 1].type == MATH_OP) return parseMathematicalOpNode(result, index);

				node->left->value = "n";
					
			case STRING:
				node->left->value = "s";
				break;
			default:
				node->left->value = "b";
		
		}

		node->value = t.value;
		return node;
	}

	if(t.type == KEYWORD) {
		AST_NODE* node = createASTNode(AST_VARIABLE_REFERENCE);
		node->endingIndex = index + 1;
		node->value = t.value;

		return node;
	}

}

/**
 * Parses a variable declaration.
 * @param result the lexer result.
 * @param index the starting index.
 */
AST_NODE* parseVariableDeclaration(struct LexerResult result, int index) {
	AST_NODE* node = createASTNode(AST_VARIABLE_DECLARATION);

	if(result.tokens[index].type == VAR) {
		node->value = "none";	
	}
	else {
		node->value = result.tokens[index].value;
	}

	node->left = createASTNode(AST_VARIABLE_NAME);
	node->left->value = result.tokens[index + 1].value;

	node->right = parseVariableValue(result, index + 2);

	if(node->right != NULL) node->endingIndex = node->right->endingIndex;
	else node->endingIndex = index + 2;

	return node;
}
