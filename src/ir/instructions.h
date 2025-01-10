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
     * @param size the size of the pointer.
     * @param ptr the pointer that is going to be allocated.
     */
    S_ALLOC,

    /**
     * Loads the values of a specific address into a variable.
     * @param var the output variable.
     * @param ptr the pointer containing the target address.
     */
    PTR_LOAD,


    /**
     * Adds two 32 bit integers together.
     * @param output the output variable of the result.
     * @param i1 the first integer.
     * @param i2 the second integer.
     */
    IADD,

    /**
     * Subtracts two 32 bit integers together.
     * @param output the output variable of the result.
     * @param i1 the first integer.
     * @param i2 the second integer.
     */
    ISUB,

    /**
     * Multiplies two 32 bit integers together.
     * @param output the output variable of the result.
     * @param i1 the first integer.
     * @param i2 the second integer.
     */
    IMUL,

    /**
     * Divides two 32 bit integers together.
     * @param output the output variable of the result.
     * @param i1 the first integer.
     * @param i2 the second integer.
     */
    IDIV,

    /**
     * Compares two 32 bit integers to check if they are equal.
     * @param out the output variable containing the result.
     * @param i1 the first integer.
     * @param i2 the second integer.
     */
    ICMP,

    /**
     * Compares two 32 bit integers to check if the first one is higher than the second one.
     * @param out the output variable containing the result.
     * @param i1 the first integer.
     * @param i2 the second integer.
     */
    ICMP_H,

    /**
     * Compares two 32 bit integers to check if the first one is higher or equal to the second one.
     * @param out the output variable containing the result.
     * @param i1 the first integer.
     * @param i2 the second integer.
    */
    ICMP_L,

    /**
     * Moves a variable value into the parameter registers.
     * @param var the variable holding the value.
     * @param index the index of the parameter to register to.
     */
    PRM_PUSH,

    /**
     * Moves a variable value into the return value register.
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
     * Sets the byte located at the pointer.
     * @param ptr the pointer containing the target address.
     * @param val the new value (an integer for now).
     */
    PTR_SET,

    /**
     * Sets 32 bits located at the pointer.
     * @param ptr the pointer containing the target address.
     * @param val the new integer value.
     */
    QUAD_SET,

    /**
     * Sets 16 bits located at the pointer.
     * @param ptr the pointer containing the target address.
     * @param val the new integer value.
     */
    DUO_SET,

    /**
     * Sets 64 bits located at the pointer.
     * @param ptr the pointer containing the target address.
     * @param val the new integer value.
     */
    OCT_SET,


    /**
     * Declares a pointer at the specified address.
     * @param ptr the new pointer.
     * @param addr the address
     */
    PTR_DEC

} IR_INSTRUCTION_CODE;

#endif