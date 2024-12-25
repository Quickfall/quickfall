/**
 * The instruction codes for the Quickfall IR.
 */

#ifndef IR_INSTRUCTIONS
#define IR_INSTRUCTIONS

/**
 * The instruction codes of IR.
 */
typedef enum {

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
     * @param ptr the pointer that is going to be allocated.
     * @param size the size of the pointer.
     */
    S_ALLOC,

    /**
     * Sets the value of the pointer's address to the provided element.
     * @param val the new value (an integer for now).
     * @param ptr the pointer containing the target address.
     */
    PTR_SET,

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


    PRM_PUSH,
    CALL,
    RET

} IR_INSTRUCTION_CODES;

#endif