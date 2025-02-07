#
#
# The Quickfall makefile. Allows to build the Quickfall toolchain.
# 
#

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

#
# Compiling configuration.
#

# Which compiler to use (must be equal to the command used to compile)
COMPILER = gcc

# Flags that are passed to the compiler
FLAGS = -Wall -Wextra -Wimplicit-function-declaration -O0 -g
#
# Path configuration.
#

# The directory containing the code of Quickfall.
SRC_DIR = src

# The directory containing the benchmarks of Quickfall.
BENCH_SRC_DIR = benchmarks

# The directory containg the tests
TESTS_SRC_DIR = tests

#
# Output definition
#

# The target of a normal mode compiling.
TARGET = quickfall$(TARGET_EXTENSION)

# The target of a bench mode compiling.
BENCH_TARGET = bench$(TARGET_EXTENSION)

# The target of a testing mode compiling.
TEST_TARGET = test$(TARGET_EXTENSION)


#
# Building variables
#

# The sources that are going to be compiled in normal mode.
SRCS = $(wildcard ${SRC_DIR}/**/*.c) $(wildcard ${SRC_DIR}/**/**/*.c)
SOURCES = $(subst src/cli/main.c,,${SRCS})

BENCH_SOURCES = ${SOURCES} $(wildcard ${BENCH_SRC_DIR}/**/*.c)

TSRCS = ${SOURCES} $(wildcard ${TESTS_SRC_DIR}/*.c)
TEST_SOURCES = $(subst tests/main.c,,${TSRCS})

#
# Building logic
#

all: prepare_build $(TARGET)
bench: prepare_build $(BENCH_TARGET)
test: prepare_build $(TEST_TARGET)

prepare_build:
	@echo [INFO] Using "${COMPILER}" as a compiler!
	@echo [INFO] Detected current operating system as ${DETECTED_OS}
	$(CHECK_COMMANDS)
	@echo [INFO] Starting building logic

clean:
	$(RM) build
	$(RM) $(TARGET)
	$(RM) $(TEST_TARGET)
	$(RM) $(BENCH_TARGET)

$(TARGET):
	$(COMPILER) $(FLAGS) $(SOURCES) src/cli/main.c -o $(TARGET)

$(BENCH_TARGET):
	$(COMPILER) $(FLAGS) $(SOURCES) benchmarks/main.c -o $(BENCH_TARGET)

$(TEST_TARGET):
	$(COMPILER) $(FLAGS) $(TEST_SOURCES) tests/main.c -o $(TEST_TARGET)

