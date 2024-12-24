/**
 * Type-related AST parsing.
 */

#include "../ast.h"

#include "../../lexer/lexer.h"
#include "../../lexer/tokens.h"

#include "../../std/types.h"

#include "../../utils/hash.h"

AST_NODE* parseTypeDefinition(LEXER_RESULT result, int index) {
	AST_NODE* node = createASTNode(AST_TYPE_DECLARATION);

	node->value = result.tokens[index].value;
	
	AST_NODE* root = NULL;
	AST_NODE* curr = root;

	index += 2;

	int stack = 0;

	for(; index < result.size; ++index) {
		TOKEN t = result.tokens[index];

		if(t.type == KEYWORD) {
			if(result.tokens[index + 1].type == KEYWORD) {
				if(root == NULL) {
					root = createASTNode(AST_VARIABLE_DECLARATION);
					curr = root;
				}
				else {
					curr->next = createASTNode(AST_VARIABLE_DECLARATION);
					curr = curr->next;
				}

				curr->right = createASTNode(AST_TYPE);
				
				int i = hashstr(result.tokens[index].value);
				
				switch(i) {
					case 1283:
						curr->right->value[0] = TYPE_NUMBER;
						break;
					case 1731:
						curr->right->value[0] = TYPE_STRING;
						break;
					case 1831:
						curr->right->value[0] = TYPE_BOOL;
						break;
					default:
						curr->right->value[0] = TYPE_CUSTOM;

						curr->right->value[1] = (i >> 24) & 0xFF;
						curr->right->value[2] = (i >> 16) & 0xFF;
						curr->right->value[3] = (i >> 8) & 0xFF;
						curr->right->value[4] = i & 0xFF;
						break;

				}
				
				curr->value = result.tokens[index + 1].value;

				index++;
			}
		}
		if(t.type == SEMICOLON) continue;
	}

	node->left = root;

	return node;
}
