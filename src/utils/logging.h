/**
 * Quickfall Error & Warning Logging.
 */

#ifndef LOGGING_H
#define LOGGING_H

struct LoggingContext {
	char** errors;
	char** warnings;

	int errorCount;
	int warningCount;
};

void logError(char* error);
void logWarning(char* warning);

#endif
