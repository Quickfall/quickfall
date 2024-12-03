/**
 * Quickfall Logging Utilities.
 */

#include <stdlib.h>

struct LoggingContext loggingCtx;

loggingCtx.errorCount = 0;
loggingCtx.warningCount = 0;

loggingCtx.errors = malloc(sizeof(char*) * 100);
loggingCtx.warnings = malloc(sizeof(char*) * 100);

void logError(char* error) {
	loggingCtx.errors[loggingCtx.errorCount] = error;
	loggingCtx.errorCount++;
}

void logWarning(char* warning) {
	loggingCtx.warning[loggingCtx.warningCount] = warning;
	loggingCtx.warningCount++;
}
