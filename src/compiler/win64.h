/**
 * The Win-x64 support for the Quickfall compiler.
 */

#include "../ir/structs.h"
#include "../ir/instructions.h"

#include "./compiler.h"

#ifndef COMPILER_WIN_64_H
#define COMPILER_WIN_64_H

/**
 * Compiles an QAsm / IR instruction to the Win-x64 bytecode.
 * @param buff the bytecode buffer.
 * @param instruction the instruction to convert.
 */
void compileInstruction(BYTECODE_BUFFER* buff, IR_INSTRUCTION* instruction);

#endif