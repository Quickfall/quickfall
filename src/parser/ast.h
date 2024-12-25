/**
 * The header file of AST nodes in Quickfall.
 */

#ifndef AST_H
#define AST_H

#include <stdbool.h>

/**
 * The type of AST Node(s).
 */
typedef enum {
	AST_ROOT,
	AST_FUNCTION_DECLARATION,
	AST_ASM_FUNCTION_DECLARATION,
	AST_VARIABLE_DECLARATION,
	AST_FUNCTION_INVOKE,
	AST_USE_DECLARATION,
	AST_FUNCTION_ROOT,
	AST_ARGUMENT_LIST,
	AST_IDENTIFIER,
	AST_STRING_LITERAL,
	AST_NUMBER_LITERAL,
	AST_BOOLEAN_LITERAL,
	AST_NULL_LITERAL,
	AST_BINARY_EXPRESSION,
	AST_RETURN_STATEMENT,
	AST_BLOCK,
	AST_ERROR,
	AST_TYPE,
	AST_VARIABLE_VALUE,
	AST_VARIABLE_REFERENCE,
	AST_VARIABLE_NAME
} AST_NODE_TYPE;

/**
 * An AST Node. Has a tree-ish structure.
 */
typedef struct AST_NODE {
	AST_NODE_TYPE type;
	

	int endingIndex;
	
	// Node data
	char* value;        // For literals, identifiers, etc.
	int valueSize;      // For binary data (e.g., ASM)
	bool isConst;       // For constant values
	
	// Symbol table info
	char* symbolName;   // Name in symbol table
	char* symbolType;   // Type information
	
	// Tree structure (using child list for better flexibility)
	struct AST_NODE* firstChild;  // First child in list
	struct AST_NODE* lastChild;   // Last child for faster appending
	struct AST_NODE* next;        // Next sibling
	struct AST_NODE* parent;      // Parent node
} AST_NODE;

/**
 * Creates a new AST Node.
 * @param type the AST type of the node.
 */
AST_NODE* createNode(AST_NODE_TYPE type);

/**
 * Creates a new AST Node with a value.
 * @param type the AST type of the node.
 * @param value the value to store.
 */
AST_NODE* createValueNode(AST_NODE_TYPE type, const char* value);

/**
 * Frees an AST node and all its children recursively.
 * @param node the node to free.
 */
void freeNode(AST_NODE* node);

/**
 * Adds a child to the given parent node.
 * @param parent the parent node.
 * @param child the child node to add.
 */
void addChild(AST_NODE* parent, AST_NODE* child);

/**
 * Inserts a new node after the given node.
 * @param node the node to insert after.
 * @param newNode the new node to insert.
 */
void insertAfter(AST_NODE* node, AST_NODE* newNode);

/**
 * Removes a node from the tree.
 * @param node the node to remove.
 */
void removeNode(AST_NODE* node);

/**
 * Clones a node.
 * @param node the node to clone.
 * @return the cloned node.
 */
AST_NODE* cloneNode(const AST_NODE* node);

/**
 * Finds the first child of a given type.
 * @param parent the parent node.
 * @param type the type of the child to find.
 * @return the first child of the given type.
 */
AST_NODE* findFirstChild(AST_NODE* parent, AST_NODE_TYPE type);

/**
 * Finds the next sibling of a given type.
 * @param node the node to find the next sibling of.
 * @param type the type of the sibling to find.
 * @return the next sibling of the given type.
 */
AST_NODE* findNextSibling(AST_NODE* node, AST_NODE_TYPE type);

/**
 * Checks if a node has a child of a given type.
 * @param parent the parent node.
 * @param type the type of the child to check for.
 * @return true if the node has a child of the given type, false otherwise.
 */
bool hasChildOfType(AST_NODE* parent, AST_NODE_TYPE type);

/**
 * Prints an AST tree for debugging.
 * @param node the root node.
 * @param depth current indentation depth.
 */
void printAST(AST_NODE* node, int depth);

/**
 * Gets the name of an AST node type.
 * @param type the type of the node.
 * @return the name of the node type.
 */
const char* getNodeTypeName(AST_NODE_TYPE type);

#endif
