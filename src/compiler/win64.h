/**
 * The Win-x64 support for the Quickfall compiler.
 */

#include <stdint.h>

#include "../ir/structs.h"
#include "../ir/instructions.h"

#include "./structs.h"

#ifndef COMPILER_WIN_64_H
#define COMPILER_WIN_64_H

/**
 * Compiles an QAsm / IR instruction to the Win-x64 bytecode.
 * @param buff the bytecode buffer.
 * @param ctx the compiler context.
 * @param instruction the instruction to convert.
 */
void compileInstruction(BYTECODE_BUFFER* buff, COMPILER_CONTEXT* ctx, IR_INSTRUCTION* instruction);

#endif