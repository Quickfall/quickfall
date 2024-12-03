/**
 * Quickfall Logging Utilities.
 */

#include <stdlib.h>

#include "./logging.h"

struct LoggingContext* loggingCtx;

void logError(char* error) {
	loggingCtx->errors[loggingCtx->errorCount] = error;
	loggingCtx->errorCount++;
}

void logWarning(char* warning) {
	loggingCtx->warnings[loggingCtx->warningCount] = warning;
	loggingCtx->warningCount++;
}
