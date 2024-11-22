/**
 * The parser of Quickfall.
 */

#include "../lexer/tokens.h"
#include "../lexer/lexer.h"
#include "./ast.h"
#include <stdio.h>
#include <string.h>

struct ASTNode* parseParameters(struct LexerResult result, int index);
struct ASTNode* parseFunctionDeclaration(struct LexerResult result, int index);
struct ASTNode* parseExpression(struct LexerResult result, int index, int end);
struct ASTNode* parseClassDeclaration(struct LexerResult result, int index);
struct ASTNode* parseInterfaceDeclaration(struct LexerResult result, int index);
struct ASTNode* parseClassBody(struct LexerResult result, int index, int allowSubclasses);

/**
 * Parses parameters of a function.
 */
struct ASTNode* parseParameters(struct LexerResult result, int index) {
    struct ASTNode* root = NULL;
    struct ASTNode* current = NULL;
    int mode = 0;

    for(; index < result.size + 1; ++index) {
        struct Token t = result.tokens[index];

        if(t.type == PAREN_CLOSE) {
            root->end = index;
            return root;
        }

        if(t.type != COMMA && t.type != KEYWORD) {
            printf("Error: Parameters must be literals! Got %d", t.type);
            return NULL;
        }

        if(t.type == COMMA) {
            if(mode != 2) {
                printf("Error: Parameters were not passed correctly!\n");
                return NULL;
            }

            struct ASTNode* node = createASTNode(AST_PARAM);
            current->next = node;
            current = node;
        }

        if(!root) {
            root = createASTNode(AST_PARAM);
            current = root;
        }

        root->end = index;

        if(!current->left) {
            current->left = createASTNode(AST_PARAM_TYPE);
            memcpy(current->left->value, result.tokens[index].value, strlen(result.tokens[index].value));
            mode = 1;
        }
        else if(!current->right) {
            current->right = createASTNode(AST_PARAM_NAME);
            memcpy(current->right->value, result.tokens[index].value, strlen(result.tokens[index].value));
            mode = 2;
        }
    }

    printf("Error: The paren wasn't closed!\n");
    return root;
}

/**
 * Parses a class declaration.
 */
struct ASTNode* parseClassDeclaration(struct LexerResult result, int index) {
    struct ASTNode* node = createASTNode(AST_CLASS_DEF);
    
    // Get class name
    if(result.tokens[index + 1].type != KEYWORD) {
        printf("Error: Expected class name!\n");
        return NULL;
    }
    
    node->left = createASTNode(AST_CLASS_NAME);
    memcpy(node->left->value, result.tokens[index + 1].value, strlen(result.tokens[index + 1].value));
    
    index += 2;
    
    // Check for extends
    if(result.tokens[index].type == EXTENDS) {
        if(result.tokens[index + 1].type != KEYWORD) {
            printf("Error: Expected parent class name after extends!\n");
            return NULL;
        }
        node->right = createASTNode(AST_CLASS_EXTENDS);
        memcpy(node->right->value, result.tokens[index + 1].value, strlen(result.tokens[index + 1].value));
        index += 2;
    }
    
    // Check for implements
    if(result.tokens[index].type == IMPLEMENTS) {
        struct ASTNode* implements = createASTNode(AST_CLASS_IMPLEMENTS);
        struct ASTNode* current = implements;
        
        index++;
        while(index < result.size && result.tokens[index].type == KEYWORD) {
            memcpy(current->value, result.tokens[index].value, strlen(result.tokens[index].value));
            if(result.tokens[index + 1].type == COMMA) {
                current->next = createASTNode(AST_CLASS_IMPLEMENTS);
                current = current->next;
                index += 2;
            } else {
                index++;
                break;
            }
        }
        
        if(!node->right) {
            node->right = implements;
        } else {
            node->right->next = implements;
        }
    }
    
    // Parse class body
    if(result.tokens[index].type != BRACKETS_OPEN) {
        printf("Error: Expected class body!\n");
        return NULL;
    }
    
    index++;
    struct ASTNode* body = parseClassBody(result, index, 1);
    if(!body) {
        return NULL;
    }
    
    if(node->right) {
        node->right->next = body;
    } else {
        node->right = body;
    }
    
    node->end = body->end;
    return node;
}

/**
 * Parses a class body, optionally allowing subclasses
 */
struct ASTNode* parseClassBody(struct LexerResult result, int index, int allowSubclasses) {
    struct ASTNode* root = createASTNode(AST_GENERIC);
    struct ASTNode* current = root;
    
    for(; index < result.size + 1; ++index) {
        struct Token t = result.tokens[index];
        
        if(t.type == BRACKETS_CLOSE) {
            root->end = index;
            return root;
        }
        
        if(t.type == CLASS) {
            if(!allowSubclasses) {
                printf("Error: Nested classes cannot have their own subclasses!\n");
                return NULL;
            }
            struct ASTNode* subclass = parseClassDeclaration(result, index);
            if(!subclass) return NULL;
            current->next = subclass;
            current = subclass;
            index = subclass->end;
        }
        else if(t.type == FUNCTION) {
            struct ASTNode* func = parseFunctionDeclaration(result, index);
            if(!func) return NULL;
            current->next = func;
            current = func;
            index = func->end;
        }
        else if(t.type == KEYWORD && result.tokens[index + 1].type == DECLARE) {
            struct ASTNode* var = parseVariableDefinition(result, index);
            if(!var) return NULL;
            current->next = var;
            current = var;
            index = var->end;
        }
    }
    
    printf("Error: Class body not closed!\n");
    return NULL;
}

/**
 * Parses an interface declaration.
 */
struct ASTNode* parseInterfaceDeclaration(struct LexerResult result, int index) {
    struct ASTNode* node = createASTNode(AST_CLASS_DEF);
    
    // Get interface name
    if(result.tokens[index + 1].type != KEYWORD) {
        printf("Error: Expected interface name!\n");
        return NULL;
    }
    
    node->left = createASTNode(AST_CLASS_NAME);
    memcpy(node->left->value, result.tokens[index + 1].value, strlen(result.tokens[index + 1].value));
    
    index += 2;
    
    // Check for extends (interfaces can extend multiple interfaces)
    if(result.tokens[index].type == EXTENDS) {
        struct ASTNode* extends = createASTNode(AST_CLASS_EXTENDS);
        struct ASTNode* current = extends;
        
        index++;
        while(index < result.size && result.tokens[index].type == KEYWORD) {
            memcpy(current->value, result.tokens[index].value, strlen(result.tokens[index].value));
            if(result.tokens[index + 1].type == COMMA) {
                current->next = createASTNode(AST_CLASS_EXTENDS);
                current = current->next;
                index += 2;
            } else {
                index++;
                break;
            }
        }
        
        node->right = extends;
    }
    
    // Parse interface body
    if(result.tokens[index].type != BRACKETS_OPEN) {
        printf("Error: Expected interface body!\n");
        return NULL;
    }
    
    index++;
    struct ASTNode* body = parseClassBody(result, index, 0);
    if(!body) {
        return NULL;
    }
    
    if(node->right) {
        node->right->next = body;
    } else {
        node->right = body;
    }
    
    node->end = body->end;
    return node;
}

/**
 * Parses a function declaration.
 */
struct ASTNode* parseFunctionDeclaration(struct LexerResult result, int index) {
    struct ASTNode* node = createASTNode(AST_FUNCTION_DEF);

    if(result.tokens[index + 2].type != PAREN_OPEN) {
        printf("Error: Excepted a paren after function name!\n");
        return NULL;
    }

    struct ASTNode* parameters = parseParameters(result, index + 3);

    node->left = createASTNode(AST_FUNCTION_TEMPLATE);
    node->left->left = createASTNode(AST_FUNCTION_NAME);
    memcpy(node->left->left->value, result.tokens[index + 1].value, strlen(result.tokens[index + 1].value));
    node->left->right = parameters;

    if(!parameters) {
        printf("Error: Argument parsing went wrong!\n");
        return NULL;
    }

    index = parameters->end + 1;

    if(result.tokens[index].type != BRACKETS_OPEN) {
        printf("Error: Excepted function body declaration got %d instead!\n", result.tokens[index - 1].type);
        printf("Dump:\n");
        for(;index < result.size +1; ++index) {
            printf("Index: %d, Type: %d\n", index, result.tokens[index].type);
        }
        return NULL;
    }

    index++;

    int start = index;

    for(;index < result.size + 1; ++index) {
        struct Token t = result.tokens[index];

        if(t.type == BRACKETS_CLOSE) {
            node->right = parseExpression(result, start, index);
        }

        printf("Token in method body: %d\n", t.type);
    }

    node->end = index;
    return node;
}

struct ASTNode* parseFunctionInvoke(struct LexerResult result, int index) {
    struct ASTNode* node = createASTNode(AST_FUNCTION_CALL);
    node->left = createASTNode(AST_INVOKE_TARGET);
    node->right = createASTNode(AST_INVOKE_PARAMETERS);
    struct ASTNode* current = node->right;
    memcpy(node->left->value, result.tokens[index].value, strlen(result.tokens[index].value));

    index += 2;

    int i = 0;
    for(; index < result.size + 1; ++index) {
        struct Token t = result.tokens[index];

        if(t.type == PAREN_CLOSE) {
            node->end = index;
            return node;
        }

        if(t.type == COMMA) {
            if(i == 0) {
                printf("Error: Invoker arguments were passed wrongly!\n");
                return NULL;
            }
            i = 0;
            continue;
        }

        struct ASTNode* n = createASTNode(AST_PARAM);

        memcpy(n->value, result.tokens[index].value, strlen(result.tokens[index].value));

        current->next = n;

        current = n;
        i = 1;
    }

    node->end = index;
    return node;
}

struct ASTNode* parseVariableDefinition(struct LexerResult result, int index) {
    struct ASTNode* node = createASTNode(AST_VARIABLE_DEF);
    node->left = createASTNode(AST_VARIABLE_NAME);

    memcpy(node->left->value, result.tokens[index].value, strlen(result.tokens[index].value));

    struct Token val = result.tokens[index + 2];

    if(val.type != KEYWORD && val.type != NUMBER && val.type != STRING && val.type != BOOLEAN) {
        printf("Error: Disallowed token as variable value: %d\n", val.type);
        return NULL;
    }

    node->right = createASTNode(AST_VARIABLE_VALUE);
    memcpy(node->right->value, result.tokens[index + 2].value, strlen(result.tokens[index + 2].value));

    node->end = index + 2;
    return node;
}

/**
 * Parses an expression in the specified range.
 */
struct ASTNode* parseExpression(struct LexerResult result, int index, int end) {
    struct ASTNode* root = createASTNode(AST_GENERIC);
    struct ASTNode* current = root;

    for(; index < end; ++index) {
        struct Token t = result.tokens[index];
        struct Token next = result.tokens[index + 1];

        if(t.type == FUNCTION) {
            if(next.type == KEYWORD) {
                struct ASTNode* node = parseFunctionDeclaration(result, index);

                if(node != NULL) {
                    index = node->end;
                    current->next = node;
                    current = node;
                }
            }
            else {
                printf("Error: Excepted function name after func!\n");
            }
        }
        else if(t.type == CLASS) {
            struct ASTNode* node = parseClassDeclaration(result, index);
            if(node != NULL) {
                index = node->end;
                current->next = node;
                current = node;
            }
        }
        else if(t.type == KEYWORD) {
            if(next.type == PAREN_OPEN) {
                struct ASTNode* node = parseFunctionInvoke(result, index);

                if(node != NULL) {
                    index = node->end;
                    current->next = node;
                    current = node;
                }
            }
            if(next.type == DECLARE) {
                struct ASTNode* node = parseVariableDefinition(result, index);

                if(node != NULL) {
                    index = node->end;
                    current->next = node;
                    current = node;
                }
            }
        }
        else {
            printf("Error: Unexcepted token %d\n", t.type);
        }
    }

    return root;
}

struct ASTNode* runParser(struct LexerResult result) {
    return parseExpression(result, 0, result.size);
}
