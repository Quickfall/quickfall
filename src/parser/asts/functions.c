/**
 * Function-related AST parsing.
 */

#include <stdint.h>
#include <stdlib.h>

#include "./variables.h"

#include "../parser.h"
#include "../ast.h"

#include "../../lexer/tokens.h"
#include "../../lexer/lexer.h"

#include "../../utils/logging.c"

/**
 * Parse the parameters from a function definition (for example).
 * @param result the lexer result.
 * @param index the starting index of the parsing.
 */
AST_NODE* parseParameters(LEXER_RESULT result, int index) {
	AST_NODE* root = createNode(AST_ARGUMENT_LIST);
	AST_NODE* current = root;

	int stack = 0;	

	for(; index < result.size + 1; ++index) {
		TOKEN t = result.tokens[index];

		switch(t.type) {
			case COMMA:
				if(stack == 0) {
					return NULL;
				}

				stack = 0;
				AST_NODE* param = createNode(AST_ARGUMENT_LIST);
				addChild(root, param);
				current = param;
				break;
			case NONE:
			case KEYWORD:
				if(stack == 2) {
					return NULL;
				}

				TOKEN next = result.tokens[index + 1];

				if(next.type == NONE || next.type == KEYWORD) {
					AST_NODE* type = createValueNode(AST_TYPE, next.value);
					addChild(current, type);
				}
				else {
					AST_NODE* name = createValueNode(AST_IDENTIFIER, t.value);
					addChild(current, name);
				}

				stack++;
				break;
			case PAREN_CLOSE:
				root->endingIndex = index;
				return root;
			case PAREN_OPEN:
				continue;
			default:
				printf("Type: %d", t.type);
				return NULL;

		}
	}
}

/**
 * Parses the arguments passed during a function call (for example).
 * @param result the result of the lexer.
 * @param index the starting index of the parsing.
 */
AST_NODE* parseArguments(LEXER_RESULT result, int index) {
	AST_NODE* root = NULL;
	AST_NODE* current = root;

	for(; index < result.size + 1; ++index) {
		TOKEN t = result.tokens[index];

		if(t.type == PAREN_CLOSE) {
			return root;
		}

		AST_NODE* arg = parseVariableValue(result, index);
		
		if(arg == NULL) return NULL;

		index = arg->endingIndex;

		if(root == NULL) {
			root = arg;
			current = root;
		}
		else {
			current->next = arg;	
		}
	}

	return NULL;
}

AST_NODE* parseFunctionDeclaration(LEXER_RESULT result, int index) {
	AST_NODE* node = createNode(AST_FUNCTION_DECLARATION);
	AST_NODE* header = createNode(AST_FUNCTION_ROOT);
	addChild(node, header);

	if(result.tokens[index].type != KEYWORD) {
		return NULL;
	}

	int off = 1;

	switch(result.tokens[index + 1].type) {
		AST_NODE* name;
		case KEYWORD:
			header->value = result.tokens[index].value;
			name = createValueNode(AST_VARIABLE_NAME, result.tokens[index + 1].value);
			addChild(header, name);
			++off;
			break;
		case PAREN_OPEN:
			header->value = "void";
			name = createValueNode(AST_VARIABLE_NAME, result.tokens[index].value);
			addChild(header, name);
			break;
		default:
			return NULL;
	}

	AST_NODE* params = parseParameters(result, index + off);
	if(params == NULL) return NULL;

	addChild(header, params);

	AST_NODE* body = parseNodes(result, params->endingIndex, AST_FUNCTION_ROOT);
	if(body) {
		addChild(node, body);
		node->endingIndex = body->endingIndex;
	}

	return node;
}

AST_NODE* parseASMFunctionDeclaration(LEXER_RESULT result, int index) {
	AST_NODE* node = createNode(AST_ASM_FUNCTION_DECLARATION);
	AST_NODE* header = createNode(AST_FUNCTION_ROOT);
	addChild(node, header);

	if(result.tokens[index + 1].type != KEYWORD) {
		return NULL;
	}

	AST_NODE* name = createValueNode(AST_VARIABLE_NAME, result.tokens[index + 1].value);
	addChild(header, name);

	AST_NODE* params = parseParameters(result, index + 2);
	if(params == NULL) return NULL;

	addChild(header, params);

	index = params->endingIndex + 2;

	int buffSize = 32;
	int buffIndex = 0;
	uint8_t* buff = malloc(sizeof(uint8_t) * buffSize);

	for(; index <= result.size; ++index) {
		TOKEN t = result.tokens[index];

		if(t.type == BRACKETS_CLOSE) {
			break;
		}

		if(t.type != NUMBER) {
			return NULL;
		}

		buff[buffIndex] = strtol(t.value, NULL, 16);
		buffIndex++;
	}

	node->endingIndex = index;

	buff = realloc(buff, sizeof(uint8_t) * buffIndex);
	
	node->valueSize = buffIndex;
	node->value = buff;

	return node;
}

AST_NODE* parseFunctionInvoke(LEXER_RESULT result, int index) {
	AST_NODE* node = createNode(AST_FUNCTION_INVOKE);

	node->value = result.tokens[index].value;
	
	AST_NODE* args = parseArguments(result, index + 2);
	node->endingIndex = index;

	if(args != NULL) {
		addChild(node, args);
		node->endingIndex = args->endingIndex;
	}

	return node;
}
