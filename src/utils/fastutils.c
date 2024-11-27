/**
 * Collection of fast functions.
 */

/**
 * Compares two strings. If they are equal return 1, if not, return 0.
 */
int faststrcmp(char* string1, char* string2) {
	return ((*(long*)string1) == ((*(long*)string2))) ? 1 : 0;
}

/**
 * Gets the string "hash" of the provided string.
 */
long strhash(char* string) {
	return *((long*)string);
}
