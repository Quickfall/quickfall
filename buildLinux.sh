#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m' # No Color

echo -e "${GREEN}Building Quickfall...${NC}"

# Run make and capture the exit code
if make; then
    echo -e "${GREEN}Build successful!${NC}"
    exit 0
else
    echo -e "${RED}Build failed!${NC}"
    exit 1
fi