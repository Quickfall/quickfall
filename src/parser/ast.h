#ifndef AST_H
#define AST_H

/**
 * The type of AST nodes
 */
enum ASTNodeType {
    AST_VARIABLE_DEF,
    AST_FUNCTION_DEF,
    AST_FUNCTION_NAME,
    AST_FUNCTION_TEMPLATE,
    AST_FUNCTION_BODY,
    AST_FUNCTION_CALL,
    AST_VARIABLE,
    AST_PARAMETERS,
    AST_PARAM,
    AST_PARAM_TYPE,
    AST_PARAM_NAME,
    AST_GENERIC,
    AST_INVOKE_TARGET,
    AST_INVOKE_PARAMETERS,
    AST_VARIABLE_TYPE,
    AST_VARIABLE_META,
    AST_VARIABLE_NAME,
    AST_VARIABLE_VALUE,
    AST_CLASS_DEF,
    AST_CLASS_NAME,
    AST_CLASS_EXTENDS,
    AST_CLASS_IMPLEMENTS,
    AST_NEW
};

/**
 * An AST node.
 */
struct ASTNode {
    enum ASTNodeType type;
    char value[32];
    struct ASTNode* left;
    struct ASTNode* right;
    struct ASTNode* next;
    int end;
};

/**
 * Creates an AST node.
 */
struct ASTNode* createASTNode(enum ASTNodeType type);

#endif