/**
 * Module resolution utilities for Quickfall.
 */

#ifndef MODULE_RESOLVER_H
#define MODULE_RESOLVER_H

#include <stdio.h>

/**
 * Resolves a module path to an actual file path.
 * @param modulePath the path from the use statement
 * @return the resolved file path or NULL if not found
 */
char* resolveModulePath(const char* modulePath);

#endif 