/**
 * The compiler of Quickfall.
 */

#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdint.h>

#include "./compiler.h"
#include "./ir.h"

#include "../lexer/lexer.h"

#include "../parser/ast.h"

#include "../utils/hash.h"

#include "../utils/logging.c"

#include "../utils/hashmap.h"

#include "./pe/pe.h"

/**
 * Parses the AST tree into a context.
 * @param tree the AST tree.
 */
IR_CTX* makeContext(AST_NODE* tree) {
	printf("  Creating IR context...\n");
	IR_CTX* ctx = malloc(sizeof(IR_CTX));
	if (!ctx) {
		printf("%sError: Failed to allocate IR context%s\n", TEXT_RED, RESET);
		return NULL;
	}

	int buffSize = 32;
	ctx->nodes = malloc(sizeof(IR_NODE) * buffSize);
	if (!ctx->nodes) {
		printf("%sError: Failed to allocate IR nodes array%s\n", TEXT_RED, RESET);
		free(ctx);
		return NULL;
	}

	ctx->nodeIndex = 0;
	ctx->nodeMap = createHashmap(512,200);
	if (!ctx->nodeMap) {
		printf("%sError: Failed to create hashmap%s\n", TEXT_RED, RESET);
		free(ctx->nodes);
		free(ctx);
		return NULL;
	}

	while(tree != NULL) {
		IR_NODE* node;
		AST_NODE* nameNode;
		AST_NODE* header;
		AST_NODE* params;
		AST_NODE* param;
		int hash;

		switch(tree->type) {
			case AST_VARIABLE_DECLARATION:
				nameNode = findFirstChild(tree, AST_VARIABLE_NAME);
				if (!nameNode) return NULL;
				
				hash = hashstr(nameNode->value);

				if(hashGet(ctx->nodeMap, hash) != NULL) {
					printf("Variable %s is already declared!\n", nameNode->value);
					return NULL;
				}

				node = createIRNode(IR_VARIABLE, nameNode->value);
				node->type = tree->value;

				AST_NODE* valueNode = findFirstChild(tree, AST_VARIABLE_VALUE);
				if (valueNode && valueNode->value)
					node->value = valueNode->value;

				ctx->nodes[ctx->nodeIndex] = node;
				ctx->nodeIndex++;

				hashPut(ctx->nodeMap, hash, node);

				if(ctx->nodeIndex > buffSize) {
					buffSize = buffSize * 1.5;
					ctx->nodes = realloc(ctx->nodes, buffSize);
				}

				break;

			case AST_FUNCTION_DECLARATION:
				header = findFirstChild(tree, AST_FUNCTION_ROOT);
				if (!header) return NULL;
				
				nameNode = findFirstChild(header, AST_VARIABLE_NAME);
				if (!nameNode) return NULL;
				
				hash = hashstr(nameNode->value);

				if(hashGet(ctx->nodeMap, hash) != NULL) {
					printf("Function %s was already declared!\n", nameNode->value);
					return NULL;
				}

				node = createIRNode(IR_FUNCTION, nameNode->value);
				node->type = header->value;

				params = findFirstChild(header, AST_ARGUMENT_LIST);
				param = params ? params->firstChild : NULL;
				while(param) {
					AST_NODE* paramName = findFirstChild(param, AST_IDENTIFIER);
					if (!paramName) break;
					
					IR_NODE* var = createIRNode(IR_FUNCTION_ARGUMENT, paramName->value);
					node->variables[node->variableIndex] = var;
					node->variableIndex++;
					hashPut(node->variableMap, hashstr(paramName->value), var);
					param = param->next;
				}

				AST_NODE* body = findNextSibling(header, AST_FUNCTION_ROOT);
				if (body) node->tree = body;

				ctx->nodes[ctx->nodeIndex] = node;
				ctx->nodeIndex++;

				hashPut(ctx->nodeMap, hash, node);
				break;

			case AST_ASM_FUNCTION_DECLARATION:
				header = findFirstChild(tree, AST_FUNCTION_ROOT);
				if (!header) return NULL;
				
				nameNode = findFirstChild(header, AST_VARIABLE_NAME);
				if (!nameNode) return NULL;
				
				hash = hashstr(nameNode->value);

				if(hashGet(ctx->nodeMap, hash) != NULL) {
					printf("Assembly function %s is already defined!\n", nameNode->value);
					return NULL;
				}

				node = createIRNode(IR_ASM_FUNCTION, nameNode->value);

				node->value = tree->value;
				node->valueSize = tree->valueSize;

				params = findFirstChild(header, AST_ARGUMENT_LIST);
				param = params ? params->firstChild : NULL;
				while(param) {
					AST_NODE* paramName = findFirstChild(param, AST_IDENTIFIER);
					if (!paramName) break;
					
					IR_NODE* var = createIRNode(IR_FUNCTION_ARGUMENT, paramName->value);
					node->variables[node->variableIndex] = var;
					node->variableIndex++;
					hashPut(node->variableMap, hashstr(paramName->value), var);
					param = param->next;
				}

				ctx->nodes[ctx->nodeIndex] = node;
				ctx->nodeIndex++;

				hashPut(ctx->nodeMap, hash, node);

				break;

			case AST_USE_DECLARATION:
				// Skip USE nodes as they've already been processed by the parser
				break;
		}

		tree = tree->next;
	}

	return ctx;
}

/**
 * Compiles the Context tree to an executable named the provided file name.
 * @param ctx the IR context.
 * @param out the output file.
 */
void compile(IR_CTX* ctx, FILE* out) {
	uint8_t* buff = malloc(sizeof(uint8_t) * 512);
	int i = 0;

	int h = hashstr("main");

	if(hashGet(ctx->nodeMap, h) == NULL) {
		printf("Error: the main function wasn't defined!\n");
		return;
	}

	IR_NODE* node = hashGet(ctx->nodeMap, h);

	if(node->nodeType != IR_FUNCTION) {
		printf("Error: main must be a function!\n");
		return;
	}

	while(node->tree != NULL) {
		if(node->tree->type == AST_FUNCTION_INVOKE) {
			int hash = hashstr(node->tree->value);

			IR_NODE* wa = hashGet(ctx->nodeMap, hash);

			if(wa == NULL) {
				printf("Error: The %s function doesn't exist!\n", node->tree->value);
				return;
			}

			if(wa->nodeType == IR_ASM_FUNCTION) {
				unsigned char* ptr = (unsigned char*) wa->value;

				for(int ii = 0; ii < wa->valueSize; ++ii) {
					buff[i] = ptr[ii];
					++i;
				}
			}
		}

		node->tree = node->tree->next;
	}

	//todo: change format based on user
	compilePE(out, buff, i);
}
