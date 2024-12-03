/**
 * Quickfall Logging Utilities.
 */

#include <stdio.h>

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

void showLogginContext() {
	for(int i = 0; i < loggingCtx->errorCount; ++i) {
		printf("%sError: %s%s%s\n", TEXT_HGRAY, TEXT_HRED, loggingCtx->errors[i], RESET);
	}

	for(int i = 0; i < loggingCtx->warningCount; ++i) {
		printf("%sWarning: %s%s%s\n", TEXT_HGRAY, TEXT_YELLOW, loggingCtx->warnings[i], RESET);
	}
}
