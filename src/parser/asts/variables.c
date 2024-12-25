/**
 * Variable-related AST parsing.
 */

#include "../../lexer/lexer.h"
#include "../../lexer/tokens.h"

#include "./math.h"

#include "../ast.h"

AST_NODE* parseVariableValue(LEXER_RESULT result, int index) {
	TOKEN t = result.tokens[index];

	if(t.type == NUMBER || t.type == STRING || t.type == BOOLEAN_VALUE) {
		AST_NODE* node = createNode(AST_VARIABLE_VALUE);
		node->endingIndex = index;
		AST_NODE* typeNode = createNode(AST_TYPE);
		addChild(node, typeNode);

		switch(t.type) {
			case NUMBER:
				if(result.size >= index + 1 && result.tokens[index + 1].type == MATH_OP) return parseMathematicalOpNode(result, index);

				typeNode->value = "n";
				break;
			case STRING:
				typeNode->value = "s";
				break;
			default:
				typeNode->value = "b";
		}

		node->value = t.value;
		return node;
	}

	if(t.type == KEYWORD) {
		AST_NODE* node = createNode(AST_VARIABLE_REFERENCE);
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
	AST_NODE* node = createNode(AST_VARIABLE_DECLARATION);

	if(result.tokens[index].type == VAR) {
		node->value = "none";	
	}
	else {
		node->value = result.tokens[index].value;
	}

	AST_NODE* nameNode = createNode(AST_VARIABLE_NAME);
	nameNode->value = result.tokens[index + 1].value;
	addChild(node, nameNode);

	AST_NODE* valueNode = parseVariableValue(result, index + 3);
	if (valueNode) {
		addChild(node, valueNode);
		node->endingIndex = valueNode->endingIndex;
	} else {
		node->endingIndex = index + 2;
	}

	return node;
}
