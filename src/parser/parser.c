/**
 * The parser of Quickfall.
 */

#include <stdio.h>
#include <stdlib.h>

#include "../lexer/tokens.h"
#include "../lexer/lexer.h"

#include "./ast.h"

#include "./asts/variables.h"
#include "./asts/functions.h"
#include "./asts/imports.h"

#include "../utils/logging.c"

/**
 * Parses the lexer tokens into nodes starting from the index.
 * @param result the LexerResult provided by the lexer.
 * @param index the starting index.
 */
AST_NODE* parseNodes(LEXER_RESULT result, int startIndex, AST_NODE_TYPE parentType) {
	printf("  Starting parser at index %d\n", startIndex);
	AST_NODE* root = createNode(parentType);
	if (!root) {
		printf("%sError: Failed to create root AST node%s\n", TEXT_RED, RESET);
		return NULL;
	}

	AST_NODE* node = NULL;

	for(int i = startIndex; i < result.size; ++i) {
		printf("    Processing token %d of type %d\n", i, result.tokens[i].type);
		TOKEN token = result.tokens[i];

		switch(token.type) {
			case BRACKETS_CLOSE:
				if(parentType == AST_FUNCTION_ROOT) {
					root->endingIndex = i;
					return root;
				}
				break;
			case FUNCTION:
				node = parseFunctionDeclaration(result, i + 1);
				if(node != NULL) {
					addChild(root, node);
					i = node->endingIndex;
				}
				break;
			case VAR:
				node = parseVariableDeclaration(result, i);
				if(node != NULL) {
					addChild(root, node);
					i = node->endingIndex;
				}
				break;
			case ASM_FUNCTION:
				node = parseASMFunctionDeclaration(result, i);
				if(node != NULL) {
					addChild(root, node);
					i = node->endingIndex;
				}
				break;
			case KEYWORD:
				if(result.tokens[i + 1].type == PAREN_OPEN) {
					node = parseFunctionInvoke(result, i);
					if(node != NULL) {
						addChild(root, node);
						i = node->endingIndex;
					}
				}
				break;
			case USE:
				node = parseUseDeclaration(result, i);
				if(node != NULL) {
					AST_NODE* pathNode = findFirstChild(node, AST_STRING_LITERAL);
					if (!pathNode) {
						printf("%sError: Use declaration missing path%s\n", TEXT_RED, RESET);
						freeNode(node);
						return NULL;
					}

					FILE* importFile = fopen(pathNode->value, "r");
					if (importFile == NULL) {
						printf("%sError: Failed to open module file '%s'%s\n", 
							   TEXT_RED, pathNode->value, RESET);
						freeNode(node);
						return NULL;
					}

					// Read the entire file
					fseek(importFile, 0, SEEK_END);
					int importSize = ftell(importFile);
					fseek(importFile, 0, SEEK_SET);

					char* importBuff = malloc(importSize + 1);
					size_t bytesRead = fread(importBuff, 1, importSize, importFile);
					importBuff[bytesRead] = '\0';
					fclose(importFile);

					// Parse the imported file content
					LEXER_RESULT importResult = runLexer(importBuff, bytesRead);
					if (importResult.size == 0) {
						printf("%sError: Empty or invalid module file '%s'%s\n",
							   TEXT_RED, pathNode->value, RESET);
						free(importBuff);
						freeNode(node);
						return NULL;
					}

					AST_NODE* importTree = parseNodes(importResult, 0, AST_ROOT);
					free(importBuff);

					if (importTree != NULL) {
						// Add all imported nodes as children of the root
						AST_NODE* child = importTree->firstChild;
						while (child) {
							AST_NODE* next = child->next;
							removeNode(child);  // Remove from import tree
							addChild(root, child);  // Add to main tree
							child = next;
						}
						freeNode(importTree);  // Free the empty import root
					}

					freeNode(node);  // Free the USE node
					i = node->endingIndex;
				}
				break;
		}
	}

	return root;
}
