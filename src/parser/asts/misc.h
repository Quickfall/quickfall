#include "../structs/misc.h"
#include "../../lexer/lexer.h"

#ifndef PARSER_ASTS_MISC_H
#define PARSER_ASTS_MISC_H

AST_USE_STD* parseASTUseSTDStatement(LEXER_RESULT result, int index);
AST_IMPORT_FILE* parseASTUseImportStatement(LEXER_RESULT result, int index);

#endif