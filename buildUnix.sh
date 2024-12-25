#!/bin/bash

# Check if terminal supports colors
if [ -t 1 ]; then
    # Check if NO_COLOR environment variable is set
    if [ -z "$NO_COLOR" ] && [ -z "$TERM" ] || [ "$TERM" = "dumb" ]; then
        # No color support
        GREEN=""
        RED=""
        NC=""
    else
        # Color support available
        GREEN='\033[92m'
        RED='\033[91m'
        NC='\033[0m'
    fi
else
    # Not a terminal, no colors
    GREEN=""
    RED=""
    NC=""
fi

echo -e "${GREEN}Building Quickfall...${NC}"

# Run make and capture the exit code
make
if [ $? -eq 0 ]; then
    echo -e "${GREEN}Build successful!${NC}"
    exit 0
else
    echo -e "${RED}Build failed!${NC}"
    exit 1
fi 