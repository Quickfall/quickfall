/**
 * The instruction codes for the Quickfall IR.
 */

#ifndef IR_INSTRUCTIONS
#define IR_INSTRUCTIONS

/**
 * The instruction codes of IR.
 */
typedef enum IR_INSTRUCTION_CODE {

    /**
     * Swaps the current IR block.
     * @param block the new IR block index in the context (function for instance).
     */
    BLOCK_SWAP,

    /**
     * Swaps the current IR block if a condition is met.
     * @param block the new IR block index in the context (function for instance).
     * @param cond the condition to match.
     */
    COND_BLOCK_SWAP,

    /**
     * Swaps the current IR block depending on a condition result.
     * @param trueBlock the new IR block index in the context (function for instance) if the condition is met.
     * @param falseBlock the new IR block index in the context (function for instance) if the condition isn't met.
     * @param cond the condition.
     */
    LOGICAL_BLOCK_SWAP,

    /**
     * Allocates a set amount of bits in the stack.
     * @param size the size of the poINT_VALUEer.
     * @param ptr the poINT_VALUEer that is going to be allocated.
     */
    S_ALLOC,


    /**
     * Adds two 32 bit INT_VALUEegers together.
     * @param output the output variable of the result.
     * @param p1 the first INT_VALUEeger poINT_VALUEer.
     * @param p2 the second INT_VALUEeger poINT_VALUEer.
     */
    IADD,

    /**
     * Subtracts two 32 bit INT_VALUEegers together.
     * @param output the output variable of the result.
     * @param p1 the first INT_VALUEeger poINT_VALUEer.
     * @param p2 the second INT_VALUEeger poINT_VALUEer.
     */
    ISUB,

    /**
     * Multiplies two 32 bit INT_VALUEegers together.
     * @param output the output variable of the result.
     * @param p1 the first INT_VALUEeger poINT_VALUEer.
     * @param p2 the second INT_VALUEeger poINT_VALUEer.
     */
    IMUL,

    /**
     * Divides two 32 bit INT_VALUEegers together.
     * @param output the output variable of the result.
     * @param p1 the first INT_VALUEeger poINT_VALUEer.
     * @param p2 the second INT_VALUEeger poINT_VALUEer.
     */
    IDIV,

    /**
     * Compares two 32 bit INT_VALUEegers to check if they are equal.
     * @param output the output variable of the result.
     * @param p1 the first INT_VALUEeger poINT_VALUEer.
     * @param p2 the second INT_VALUEeger poINT_VALUEer.
     */
    ICMP,

    /**
     * Compares two 32 bit INT_VALUEegers to check if the first one is higher than the second one.
     * @param output the output variable of the result.
     * @param p1 the first INT_VALUEeger poINT_VALUEer.
     * @param p2 the second INT_VALUEeger poINT_VALUEer.
     */
    ICMP_H,

    /**
     * Compares two 32 bit INT_VALUEegers to check if the first one is higher or equal to the second one.
     * @param output the output variable of the result.
     * @param p1 the first INT_VALUEeger poINT_VALUEer.
     * @param p2 the second INT_VALUEeger poINT_VALUEer.
     */
    ICMP_L,

    /**
     * Moves a variable value INT_VALUEo the parameter registers.
     * @param var the variable holding the value.
     * @param index the index of the parameter to register to.
     */
    PRM_PUSH,

    /**
     * Moves a variable value INT_VALUEo the return value register.
     * @param var the variable holding the value.
     */
    RET_PUSH,

    /**
     * Calls a function.
     * @param funcName the functionName.
     */
    CALL,

    /**
     * Returns from a function.
     */
    RET,

    /**
     * Saves the stack.
     */
    STACK_SAVE,


    /**
     * Loads the stack back.
     */
    STACK_LOAD,

    /**
     * Frees all of the stack used by the function.
     */
    STACK_FREE_FUNC,


    /**
     * Sets the byte located at the poINT_VALUEer.
     * @param ptr the poINT_VALUEer containing the target address.
     * @param val the new value (an INT_VALUEeger for now).
     */
    PTR_SET,

    /**
     * Sets 32 bits located at the poINT_VALUEer.
     * @param ptr the poINT_VALUEer containing the target address.
     * @param val the new INT_VALUEeger value.
     */
    QUAD_SET,

    /**
     * Sets 16 bits located at the poINT_VALUEer.
     * @param ptr the poINT_VALUEer containing the target address.
     * @param val the new INT_VALUEeger value.
     */
    DUO_SET,

    /**
     * Sets 64 bits located at the poINT_VALUEer.
     * @param ptr the poINT_VALUEer containing the target address.
     * @param val the new INT_VALUEeger value.
     */
    OCT_SET,


    /**
     * Declares a poINT_VALUEer at the specified address.
     * @param ptr the new poINT_VALUEer.
     * @param addr the address
     */
    PTR_DEC,

    /**
     * Declares a poINT_VALUEer that is an offset of another poINT_VALUEer.
     * @param ptr the new poINT_VALUEer.
     * @param p the old poINT_VALUEer.
     * @param off the offset.
     */
    PTR_DEC_OFF,

    /**
     * Loads a parameter and pushes it into a register.
     * @param index the parameter index.
     * @param ptr the pointer.
     */
    PRM_LOAD

} IR_INSTRUCTION_CODE;

/**
 * The types for parameters in an IR instruction.
 */
typedef enum IR_PARAMETER_TYPE {

    VARIABLE,
    INT_VALUE,
    STRING_VALUE

} IR_PARAMETER_TYPE;

#endif