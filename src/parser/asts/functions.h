/**
 * Parsing for function related ASTs.
 */

#include "../structs/functions.h"

#include "../../lexer/lexer.h"

#ifndef FUNCTIONS_AST_H
#define FUNCTIONS_AST_H

/**
 * Parses a function declaration into AST.
 * @param result the Lexer result.
 * @param index the index of the start of the parsing.
 */
AST_FUNCTION_DEC* parseFunctionDeclaration(LEXER_RESULT result, int index);

/**
 * Parses an ASM function declaration into AST.
 * @param result the Lexer result.
 * @param index the index of the start of the parsing.
 */
AST_ASM_FUNCTION_DEC* parseASMFunctionDeclaration(LEXER_RESULT result, int index);

/**
 * Parses the parameters of a function into AST.
 * @param result the Lexer result.
 * @param index the index of the start of the parsing.
 */
void parseFunctionParameters(AST_FUNCTION_DEC* func, LEXER_RESULT result, int index);

#endif