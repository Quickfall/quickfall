/**
 * The AST Nodes in Quickfall.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

#include "./ast.h"
#include "../utils/logging.c"

/**
 * Creates a new AST Node.
 * @param type the AST type of the node.
 */
AST_NODE* createNode(AST_NODE_TYPE type) {
	AST_NODE* node = malloc(sizeof(AST_NODE));
	if (!node) {
		printf("%sError: Failed to allocate AST node%s\n", TEXT_RED, RESET);
		return NULL;
	}

	node->type = type;
	node->endingIndex = 0;
	node->value = NULL;
	node->valueSize = 0;
	node->isConst = false;
	node->symbolName = NULL;
	node->symbolType = NULL;
	node->firstChild = NULL;
	node->lastChild = NULL;
	node->next = NULL;
	node->parent = NULL;

	return node;
}

AST_NODE* createValueNode(AST_NODE_TYPE type, const char* value) {
	AST_NODE* node = createNode(type);
	if (!node) return NULL;
	
	if (value) {
		node->value = strdup(value);
		if (!node->value) {
			printf("%sError: Failed to duplicate value string%s\n", TEXT_RED, RESET);
			free(node);
			return NULL;
		}
	}
	
	return node;
}

void freeNode(AST_NODE* node) {
	if (!node) return;
	
	// First free all children
	AST_NODE* child = node->firstChild;
	while (child) {
		AST_NODE* next = child->next;
		freeNode(child);
		child = next;
	}
	
	// Free allocated strings
	free(node->value);
	free(node->symbolName);
	free(node->symbolType);
	
	// Free the node itself
	free(node);
}

void addChild(AST_NODE* parent, AST_NODE* child) {
	if (!parent || !child) return;
	
	child->parent = parent;
	
	if (!parent->firstChild) {
		parent->firstChild = child;
		parent->lastChild = child;
	} else {
		parent->lastChild->next = child;
		parent->lastChild = child;
	}
}

void insertAfter(AST_NODE* node, AST_NODE* newNode) {
	if (!node || !newNode) return;
	
	newNode->next = node->next;
	node->next = newNode;
	newNode->parent = node->parent;
	
	if (node->parent && node->parent->lastChild == node) {
		node->parent->lastChild = newNode;
	}
}

void removeNode(AST_NODE* node) {
	if (!node || !node->parent) return;
	
	AST_NODE* parent = node->parent;
	AST_NODE* prev = NULL;
	AST_NODE* current = parent->firstChild;
	
	while (current && current != node) {
		prev = current;
		current = current->next;
	}
	
	if (!current) return; // Node not found
	
	if (prev) {
		prev->next = node->next;
	} else {
		parent->firstChild = node->next;
	}
	
	if (parent->lastChild == node) {
		parent->lastChild = prev;
	}
	
	node->parent = NULL;
	node->next = NULL;
}

AST_NODE* cloneNode(const AST_NODE* node) {
	if (!node) return NULL;
	
	AST_NODE* clone = createNode(node->type);
	if (!clone) return NULL;
	
	// Copy basic properties
	clone->endingIndex = node->endingIndex;
	clone->valueSize = node->valueSize;
	clone->isConst = node->isConst;
	
	// Clone strings
	if (node->value) clone->value = strdup(node->value);
	if (node->symbolName) clone->symbolName = strdup(node->symbolName);
	if (node->symbolType) clone->symbolType = strdup(node->symbolType);
	
	// Clone children
	AST_NODE* child = node->firstChild;
	while (child) {
		AST_NODE* childClone = cloneNode(child);
		if (childClone) {
			addChild(clone, childClone);
		}
		child = child->next;
	}
	
	return clone;
}

AST_NODE* findFirstChild(AST_NODE* parent, AST_NODE_TYPE type) {
	if (!parent) return NULL;
	
	AST_NODE* child = parent->firstChild;
	while (child) {
		if (child->type == type) return child;
		child = child->next;
	}
	return NULL;
}

AST_NODE* findNextSibling(AST_NODE* node, AST_NODE_TYPE type) {
	if (!node) return NULL;
	
	AST_NODE* sibling = node->next;
	while (sibling) {
		if (sibling->type == type) return sibling;
		sibling = sibling->next;
	}
	return NULL;
}

bool hasChildOfType(AST_NODE* parent, AST_NODE_TYPE type) {
	return findFirstChild(parent, type) != NULL;
}

const char* getNodeTypeName(AST_NODE_TYPE type) {
	static const char* typeNames[] = {
		"ROOT", "FUNCTION_DECLARATION", "ASM_FUNCTION_DECLARATION",
		"VARIABLE_DECLARATION", "FUNCTION_INVOKE", "USE_DECLARATION",
		"FUNCTION_ROOT", "ARGUMENT_LIST", "IDENTIFIER", "STRING_LITERAL",
		"NUMBER_LITERAL", "BOOLEAN_LITERAL", "NULL_LITERAL",
		"BINARY_EXPRESSION", "RETURN_STATEMENT", "BLOCK", "ERROR"
	};
	return typeNames[type];
}

void printAST(AST_NODE* node, int depth) {
	if (!node) return;
	
	// Print indentation
	for (int i = 0; i < depth; i++) printf("  ");
	
	// Print node info
	printf("%s", getNodeTypeName(node->type));
	if (node->value) printf(": %s", node->value);
	if (node->symbolName) printf(" (%s)", node->symbolName);
	if (node->symbolType) printf(" : %s", node->symbolType);
	printf("\n");
	
	// Print children
	AST_NODE* child = node->firstChild;
	while (child) {
		printAST(child, depth + 1);
		child = child->next;
	}
}
