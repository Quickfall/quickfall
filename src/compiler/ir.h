/**
 * The compiler's internal IR.
 */

#include "../parser/ast.h"

#include "../utils/hashmap.h"

#ifndef IR_H
#define IR_H

typedef enum {
	IR_TYPE_FUNCTION,

	IR_TYPE_ASM_FUNCTION,

	IR_TYPE_TYPE_DEC,

	IR_TYPE_VARIABLE,
	IR_TYPE_FUNCTION_ARGUMENT,
	IR_TYPE_FUNCTION_BODY_VARIABLE
} IR_TYPE;

/**
 * An IR represented function.
 */
typedef struct IR_FUNCTION {
	
	char* funcName;

	unsigned char* returnType;

} IR_FUNCTION;

/**
 * An IR represented variable.
 */
typedef struct IR_VARIABLE {

	char* varName;
	unsigned char* type;

	unsigned char* value;
	int valueSize;

} IR_VARIABLE;

/**
 * A member of an IR represented type.
 */
typedef struct IR_TYPE_MEMBER {
	
	char* varName;
	unsigned char* type;

} IR_TYPE_MEMBER;

/**
 * A type declaration.
 */
typedef struct IR_TYPE_DEC {

	char* typeName;

	IR_TYPE_MEMBER* members;
	int memberCount;
	
} IR_TYPE_DEC;

/**
 * The overall IR context.
 */
typedef struct {
	IR_NODE** nodes;
	int nodeIndex;

	struct Hashmap* nodeMap;
} IR_CTX;

/**
 * Creates an IR node based on the type and the name given.
 * @param type the IR type of the node.
 * @param nodeName the name of the IR node.
 */
inline extern IR_NODE* createIRNode(IR_TYPE type, char* nodeName);

#endif
