/**
 * Module resolution utilities for Quickfall.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#ifdef _WIN32
#include <windows.h>
#else
#include <unistd.h>
#endif

#include "./logging.c"

#ifdef _WIN32
#define PATH_SEPARATOR "\\"
#define STD_LIB_PATH "lib\\winx86-64\\"
#define MODULES_PATH "quickfall_modules\\"
#else
#define PATH_SEPARATOR "/"
#define STD_LIB_PATH "lib/unix64/"
#define MODULES_PATH "quickfall_modules/"
#endif

/**
 * Checks if a file exists and is readable
 * @param path the file path to check
 * @return 1 if file exists and is readable, 0 otherwise
 */
static int fileExists(const char* path) {
    FILE* f = fopen(path, "r");
    if (f != NULL) {
        fclose(f);
        return 1;
    }
    return 0;
}

/**
 * Try to find a module in a specific directory
 * @param dir the directory to search in
 * @param name the module name
 * @return resolved path or NULL if not found
 */
static char* tryFindModule(const char* dir, const char* name) {
    char* path = malloc(256);
    snprintf(path, 256, "%s%s.qf", dir, name);
    printf("  Trying to find module in %s%s.qf\n", dir, name);
    
    if (fileExists(path)) {
        return path;
    }
    
    free(path);
    return NULL;
}

/**
 * Resolves a module path to an actual file path.
 * @param modulePath the path from the use statement
 * @return the resolved file path or NULL if not found
 */
char* resolveModulePath(const char* modulePath) {
    if (modulePath == NULL) return NULL;

    // Remove quotes from the path
    char* cleanPath = strdup(modulePath);
    printf("  Original module path: '%s'\n", modulePath);
    size_t len = strlen(cleanPath);
    if (len >= 2 && cleanPath[0] == '"' && cleanPath[len-1] == '"') {
        cleanPath[len-1] = '\0';
        cleanPath++;
    }
    printf("  Cleaned module path: '%s'\n", cleanPath);

    char* resolvedPath = NULL;
    
    // First try as direct file path
    if (cleanPath[0] == '.' || cleanPath[0] == '/' || 
        (len > 2 && cleanPath[1] == ':')) {  // Handles ./file, ../file, /file, and C:/file
        resolvedPath = malloc(256);
        snprintf(resolvedPath, 256, "%s", cleanPath);
        if (fileExists(resolvedPath)) {
            return resolvedPath;
        }
        // Try with .qf extension
        snprintf(resolvedPath, 256, "%s.qf", cleanPath);
        if (fileExists(resolvedPath)) {
            return resolvedPath;
        }
        free(resolvedPath);
    }
    // Then try as module
    else if (cleanPath[0] == '@') {
        // Get executable path
        char exePath[256];
        #ifdef _WIN32
        GetModuleFileName(NULL, exePath, sizeof(exePath));
        // Remove executable name to get directory
        char* lastSlash = strrchr(exePath, '\\');
        #else
        ssize_t len = readlink("/proc/self/exe", exePath, sizeof(exePath)-1);
        if (len != -1) {
            exePath[len] = '\0';
            // Remove executable name to get directory
            char* lastSlash = strrchr(exePath, '/');
        }
        #endif
        if (lastSlash) {
            *lastSlash = '\0';  // Truncate to directory path
        }
        printf("  Quickfall directory: %s\n", exePath);

        // Standard library module
        const char* modName = cleanPath + 1;  // Skip the @ symbol
        
        // Convert forward slashes to system path separator
        char* convertedPath = strdup(modName);
        for (char* p = convertedPath; *p; p++) {
            if (*p == '/') *p = PATH_SEPARATOR[0];
        }
        
        printf("  Looking for standard library module: %s\n", convertedPath);
        // Try relative to executable directory
        resolvedPath = malloc(256);
        snprintf(resolvedPath, 256, "%s%s%s%s.qf", exePath, PATH_SEPARATOR, STD_LIB_PATH, convertedPath);
        if (fileExists(resolvedPath)) {
            printf("  Found in standard library: %s\n", resolvedPath);
            free(convertedPath);
            return resolvedPath;
        }
        free(resolvedPath);

        // Then try from executable directory
        resolvedPath = tryFindModule(STD_LIB_PATH, convertedPath);
        if (resolvedPath != NULL) {
            printf("  Found in standard library: %s\n", resolvedPath);
            free(convertedPath);
            return resolvedPath;
        }
        free(convertedPath);
    } else {
        // External module from quickfall_modules
        resolvedPath = malloc(256);
        snprintf(resolvedPath, 256, "%s%s.qf", MODULES_PATH, cleanPath);
    }

    // Check if file exists
    printf("  Attempting to resolve path: '%s'\n", resolvedPath ? resolvedPath : "NULL");
    if (resolvedPath == NULL || !fileExists(resolvedPath)) {
        printf("%sError: Could not find module '%s' at '%s'%s\n", 
               TEXT_RED, cleanPath, resolvedPath, RESET);
        if (resolvedPath) free(resolvedPath);
        return NULL;
    }

    return resolvedPath;
} 