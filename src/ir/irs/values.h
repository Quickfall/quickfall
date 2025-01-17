/**
 * IR for value related.
 */

#ifndef IR_VALUES_H
#define IR_VALUES_H

/**
 * Parses the value into the buffer.
 * @param buff the byte buffer.
 * @param index the index of the buffer.
 * @param value the value to parse.
 */
void parseValue(void** buff, int index, void* value);

/**
 * Gets the byte equivalent.
 * @param value the value.
 */
unsigned char* getByteEquivalent(void* value);

/**
 * Gets the value size for a certain type for a  parameter.
 * @param type the type's byte indentifier.
 */
int getValueSize(unsigned char type);

#endif