/**
 * Variable-related AST parsing.
 */

#include "../../utils/hash.h"

#include "../../std/types.h"

#include "../../lexer/lexer.h"
#include "../../lexer/tokens.h"

#include "./math.h"

#include "../ast.h"

AST_NODE* parseVariableValue(LEXER_RESULT result, int index) {
	TOKEN t = result.tokens[index];

	if(t.type == NUMBER || t.type == STRING || t.type == BOOLEAN_VALUE) {
		AST_NODE* node = createASTNode(AST_VARIABLE_VALUE);
		node->endingIndex = index;
		node->left = createASTNode(AST_TYPE);

		switch(t.type) {
			case NUMBER:
				if(result.size >= index + 1 && result.tokens[index + 1].type == MATH_OP) return parseMathematicalOpNode(result, index);

				node->left->value[0] = TYPE_NUMBER;
				break;
			case STRING:
				node->left->value[0] = TYPE_STRING;
				break;
			case BOOLEAN_VALUE:
				node->left->value[0] = TYPE_BOOL;
				break;
			default:
				node->left->value[0] = TYPE_VOID;
		
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
AST_NODE* parseVariableDeclaration(LEXER_RESULT result, int index) {
	AST_NODE* node = createASTNode(AST_VARIABLE_DECLARATION);

	if(result.tokens[index].type == VAR) {
		node->value[0] = TYPE_VOID;	
	}
	else {
		int hash = hashstr(result.tokens[index].value);

		switch(hash) {
			case 1283: // "int"
				node->value[0] = TYPE_NUMBER;
				break;
			case 1731: // "str"
				node->value[0] = TYPE_STRING;
				break;
			case 1831: // "boolean"
				node->value[0] = TYPE_BOOL;
				break;
			default:
				node->value[0] = TYPE_CUSTOM;
				
		}
	}

	node->left = createASTNode(AST_VARIABLE_NAME);
	node->left->value = result.tokens[index + 1].value;

	node->right = parseVariableValue(result, index + 3);

	if(node->value[0] != TYPE_VOID && node->value[0] != node->right->value[0]) {
		printf("Error: Variable type mismatch!\n");
	}	

	if(node->right != NULL) node->endingIndex = node->right->endingIndex;
	else node->endingIndex = index + 2;

	return node;
}
