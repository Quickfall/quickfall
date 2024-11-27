# Detect operating system and set commands accordingly
ifeq ($(OS),Windows_NT)
    DETECTED_OS := Windows
    TARGET_EXTENSION := .exe
    RM := del /Q
    MKDIR := mkdir
    CP := copy
    PATHSEP := \\
else
    DETECTED_OS := $(shell uname -s)
    TARGET_EXTENSION :=
    RM := rm -rf
    MKDIR := mkdir -p
    CP := cp
    PATHSEP := /
endif

# Compiler settings
CFLAGS = -Wall -Wextra -std=c11 -Wimplicit-function-declaration

# Directories
SRC_DIR = src
CLI_DIR = $(SRC_DIR)/cli
LEXER_DIR = $(SRC_DIR)/lexer
PARSER_DIR = $(SRC_DIR)/parser
COMPILER_DIR = $(SRC_DIR)/compiler
COMPILER_PLATFORMS = $(COMPILER_DIR)/platforms
UTILS_DIR = $(SRC_DIR)/utils
BUILD_DIR = build

# Source files
SOURCES = $(LEXER_DIR)/lexer.c \
          $(LEXER_DIR)/tokens.c \
          $(PARSER_DIR)/parser.c \
          $(PARSER_DIR)/ast.c \
          $(COMPILER_DIR)/compiler.c \
	  $(COMPILER_DIR)/stdl.c \
          $(COMPILER_PLATFORMS)/linux.c \
          $(COMPILER_PLATFORMS)/windowsx86-64.c \
          $(UTILS_DIR)/hashes.c \
	  $(UTILS_DIR)/fastutils.c \

# Executable name
TARGET = quickfall$(TARGET_EXTENSION)
BENCH_TARGET = bench$(TARGET_EXTENSION)

# Default target
all: check_commands $(TARGET)
bench: check_commands $(BENCH_TARGET)

OBJECTS = $(SOURCES:%.c=$(BUILD_DIR)/%.o)

# Check commands target
check_commands:
	@echo "Using compiler: gcc"
	@echo "Operating System: $(DETECTED_OS)"
	$(CHECK_COMMANDS)
	@echo "Sources: $(SOURCES)"

# Create build directory structure
$(BUILD_DIR):
	$(MKDIR) $(BUILD_DIR)$(PATHSEP)benchmarks
	$(MKDIR) $(BUILD_DIR)$(PATHSEP)$(SRC_DIR)$(PATHSEP)cli
	$(MKDIR) $(BUILD_DIR)$(PATHSEP)$(SRC_DIR)$(PATHSEP)compiler
	$(MKDIR) $(BUILD_DIR)$(PATHSEP)$(SRC_DIR)$(PATHSEP)compiler$(PATHSEP)platforms
	$(MKDIR) $(BUILD_DIR)$(PATHSEP)$(SRC_DIR)$(PATHSEP)lexer
	$(MKDIR) $(BUILD_DIR)$(PATHSEP)$(SRC_DIR)$(PATHSEP)parser
	$(MKDIR) $(BUILD_DIR)$(PATHSEP)$(SRC_DIR)$(PATHSEP)utils

# Compile source files
$(BUILD_DIR)/%.o: %.c | $(BUILD_DIR)
	gcc $(CFLAGS) -c $< -o $@

# Link object files
$(TARGET): $(OBJECTS)
	gcc $(CFLAGS) -c src/cli/main.c -o build/src/cli/main.o
	gcc build/src/cli/main.o $(OBJECTS) -o $@

$(BENCH_TARGET): $(OBJECTS)
	gcc $(CFLAGS) -c benchmarks/main.c -o build/benchmarks/main.o
	gcc build/benchmarks/main.o $(OBJECTS) -o $@

# Clean build files
clean:
	$(RM) $(BUILD_DIR)
	$(RM) $(TARGET)

# Phony targets
.PHONY: all clean check_commands
