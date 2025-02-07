/**
 * The Quickfall Parser & AST Generator.
 */

#include <stdio.h>
#include <stdlib.h>

#include "./parser.h"

#include "./ast.h"

#include "../lexer/lexer.h"

#include "./structs/tree.h"
#include "./structs/variables.h"
#include "./structs/misc.h"

#include "./asts/variables.h"
#include "./asts/functions.h"
#include "./asts/misc.h"


/**
 * Parses the Lexer result into an AST root.
 * @param result the Lexer result.
 * @param startingIndex the starting Index.
 * @param type the output AST type.
 */
void* parseRoot(LEXER_RESULT result, int startingIndex, AST_TYPE type) {
    AST_TREE_BRANCH* root = malloc(sizeof(AST_TREE_BRANCH));
    AST_TREE_BRANCH* curr = root;

    root->type = 0x99;

    for(int i = startingIndex; i < result.size; ++i) {
        TOKEN t = result.tokens[i];

        switch(t.type) {
            case TYPE_INT32:
            case TYPE_INT24:
            case TYPE_INT16:
            case TYPE_INT8:
            case TYPE_BIT:
            case VAR:
                void* node = parseASTVariableDeclaration(result, i);
                if(node != NULL) {
                    curr->next = node;
                    curr = node;
                }
                i = ((AST_TREE_BRANCH*)node)->endingIndex;
                break;

            case KEYWORD:
                if(result.tokens[i + 1].type == DECLARE) {
                    void* node = parseVariableModification(result, i);
                    if(node != NULL) {
                        curr->next = node;
                        curr = node;
                    }
                    i = ((AST_TREE_BRANCH*)node)->endingIndex;
                    break;
                }
                break;

            case FUNCTION:
                node = parseFunctionDeclaration(result, i);
                if(node != NULL) {
                    curr->next = node;
                    curr = node;
                }
                i = ((AST_TREE_BRANCH*)node)->endingIndex;
                break;
            
            case ASM_FUNCTION:
                node = parseASMFunctionDeclaration(result, i);
                if(node != NULL) {
                    curr->next = node;
                    curr = node;
                }
                i = ((AST_TREE_BRANCH*)node)->endingIndex; // todo: fix asmf ending index
                break;
 
            case BRACKETS_CLOSE:
                if(type == AST_TYPE_FUNC_ROOT) {
                    curr->next = NULL;
                    root->endingIndex = i;
                    return root;
                }
                break;

            case USE:
                node = parseASTUseSTDStatement(result, i);
                if(node != NULL) {
                    curr->next = node;
                    curr = node;
                }
                i = ((AST_TREE_BRANCH*)node)->endingIndex;
                break;
            
            case IMPORT:
                node = parseASTUseImportStatement(result, i);
                if(node != NULL) {
                    curr->next = node;
                    curr = node;
                }
                i = ((AST_TREE_BRANCH*)node)->endingIndex;
                break;

            default:
                printf("Error: Unexcepted token %d!\n", t.type);
                break;
        }
        root->endingIndex = i;
    }

    curr->next = NULL;
    return root;
}

/**
 * Appends a new node in the AST tree.
 * @param curr the current AST node.
 * @param root the root of the AST tree.
 * @param node the node to add to the tree.
 */
void append(AST_TREE_BRANCH* curr, AST_TREE_BRANCH* root, void* node) {
    if(node == NULL) return;

    if(curr == NULL) {
        root = node;
        curr = root;
    }
    else {
        curr->next = node;
        curr = node;
    }
}