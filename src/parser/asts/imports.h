/**
 * Import/Use-related AST parsing header.
 */

#ifndef IMPORTS_H
#define IMPORTS_H

#include "../../lexer/lexer.h"
#include "../ast.h"

/**
 * Parses a use/import declaration.
 * @param result the lexer result.
 * @param index the starting index.
 */
AST_NODE* parseUseDeclaration(LEXER_RESULT result, int index);

#endif 