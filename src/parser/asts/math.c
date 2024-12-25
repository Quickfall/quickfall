/**
 * Math ASTs for the Quickfall parser.
 */

#include <stdlib.h>

#include "../../lexer/lexer.h"

#include "../ast.h"
#include "./variables.h"

/**
 * Parses the mathematical operation.
 * @param result the lexer result.
 * @param index the starting index.
 */
AST_NODE* parseMathematicalOpNode(LEXER_RESULT result, int index) {
	AST_NODE* node = createNode(AST_BINARY_EXPRESSION);
	AST_NODE* header = createNode(AST_IDENTIFIER);
	addChild(node, header);

	AST_NODE* leftOperand = createValueNode(AST_IDENTIFIER, result.tokens[index].value);
	addChild(header, leftOperand);

	AST_NODE* operator = createValueNode(AST_IDENTIFIER, result.tokens[index + 1].value);
	addChild(header, operator);

	node->value = malloc(1);
	if(result.size >= index + 2 && result.tokens[index + 2].type == DECLARE) {
		node->value[0] = '1';
	}

	AST_NODE* rightOperand = parseVariableValue(result, index + 2);
	if (rightOperand) {
		addChild(node, rightOperand);
		node->endingIndex = rightOperand->endingIndex;
	}

	return node;
}
