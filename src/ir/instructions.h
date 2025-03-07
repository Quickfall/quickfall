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
     * Adds two 32 bit integers together.
     * @param output the output variable of the result.
     * @param p1 the first integer pointer.
     * @param p2 the second integer pointer.
     */
    IADD,

    /**
     * Subtracts two 32 bit integers together.
     * @param output the output variable of the result.
     * @param p1 the first integer pointer.
     * @param p2 the second integer pointer.
     */
    ISUB,

    /**
     * Multiplies two 32 bit integers together.
     * @param output the output variable of the result.
     * @param p1 the first integer pointer.
     * @param p2 the second integer pointer.
     */
    IMUL,

    /**
     * Divides two 32 bit integers together.
     * @param output the output variable of the result.
     * @param p1 the first integer pointer.
     * @param p2 the second integer pointer.
     */
    IDIV,

    /**
     * Compares two 32 bit integers to check if they are equal.
     * @param output the output variable of the result.
     * @param p1 the first integer pointer.
     * @param p2 the second integer pointer.
     */
    ICMP,

    /**
     * Compares two 32 bit integers to check if the first one is higher than the second one.
     * @param output the output variable of the result.
     * @param p1 the first integer pointer.
     * @param p2 the second integer pointer.
     */
    ICMP_H,

    /**
     * Compares two 32 bit integers to check if the first one is higher or equal to the second one.
     * @param output the output variable of the result.
     * @param p1 the first integer pointer.
     * @param p2 the second integer pointer.
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
    PTR_DEC,

    /**
     * Declares a pointer that is an offset of another pointer.
     * @param ptr the new pointer.
     * @param p the old pointer.
     * @param off the offset.
     */
    PTR_DEC_OFF

} IR_INSTRUCTION_CODE;

/**
 * The types for parameters in an IR instruction.
 */
typedef enum IR_PARAMETER_TYPE {

    VARIABLE,
    INT,
    STRING

} IR_PARAMETER_TYPE;

/**
 * Holds all of the parameter types of the instructions.
 */
const unsigned char INSTRUCTION_PARAMETER_TYPES[24][3] = {
    {INT},
    {INT, VARIABLE},
    {INT, INT, VARIABLE},
    {INT, VARIABLE},

    {VARIABLE, INT, INT},
    {VARIABLE, INT, INT},
    {VARIABLE, INT, INT},
    {VARIABLE, INT, INT},

    {VARIABLE, INT, INT},
    {VARIABLE, INT, INT},
    {VARIABLE, INT, INT},

    {VARIABLE, INT},
    {VARIABLE},
    {STRING},
    
    {0},
    {0},
    {0},
    {0},

    {VARIABLE, VARIABLE},
    {VARIABLE, VARIABLE},
    {VARIABLE, VARIABLE},
    {VARIABLE, VARIABLE},

    {VARIABLE, INT},
    {VARIABLE, VARIABLE, VARIABLE}
};

#endif