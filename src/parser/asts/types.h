/**
 * Type-related AST parsing.
 */

#ifndef TYPE_AST_H
#define TYPE_AST_H

#include "../ast.h"

#include "../../lexer/lexer.h"
#include "../../lexer/tokens.h"

/**
 * Parses a type definition.
 * @param result the Lexer result.
 * @param index the starting index of the parsing.
 */
AST_NODE* parseTypeDefinition(LEXER_RESULT result, int index);

#endif
