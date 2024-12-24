/**
 * Standart types in Quickfall.
 */

#ifndef STD_TYPES_H
#define STD_TYPES_H

/**
 * The standart Quickfall types.
 */
typedef enum {
	VOID,
	STRING,
	NUMBER,
	BOOL
} STD_TYPE;

/**
 * Conversion of the STD_TYPE to thier bytes.
 * Used for internal and quickfall compiled.
 */
unsigned char* STD_TYPES_BYTES[4] = {
	0x01,
	0x02,
	0x03,
	0x04
}

#endif
