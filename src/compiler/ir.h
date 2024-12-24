/**
 * The compiler's internal IR.
 */

#include "../parser/ast.h"

#include "../utils/hashmap.h"

#ifndef IR_H
#define IR_H

typedef enum {
	IR_FUNCTION,

	IR_ASM_FUNCTION,

	IR_TYPE_DEC,

	IR_VARIABLE,
	IR_FUNCTION_ARGUMENT,
	IR_FUNCTION_BODY_VARIABLE
} IR_TYPE;

/**
 * An IR Node.
 */
typedef struct IR_NODE {

	IR_TYPE nodeType;

	// Shared Properties
	
	char* nodeName;
	char* type;

	// Variable Properties
	void* value;
	int valueSize;

	// Function Properties
	struct IR_NODE** variables;
	int variableIndex;

	struct Hashmap* variableMap;

	AST_NODE* tree;
} IR_NODE;

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
