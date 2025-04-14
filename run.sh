#!/bin/bash
set -e

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${GREEN}Starting Orion...${NC}"

# Determine the appropriate binary directory
if [ -d "target/release" ] && [ -f "target/release/background" ]; then
    BIN_DIR="target/release"
elif [ -d "target/debug" ] && [ -f "target/debug/background" ]; then
    BIN_DIR="target/debug"
elif [ -d "dist/bin" ] && [ -f "dist/bin/background" ]; then
    BIN_DIR="dist/bin"
else
    echo -e "${RED}Could not find Orion binaries.${NC}"
    echo -e "${RED}Please run ./build.sh first.${NC}"
    exit 1
fi

echo -e "${GREEN}Using binaries from: ${BIN_DIR}${NC}"

# Check if background service is already running
if pgrep -f "${BIN_DIR}/background" > /dev/null; then
    echo -e "${GREEN}Orion background service is already running.${NC}"
else
    echo -e "${GREEN}Starting background service...${NC}"
    ${BIN_DIR}/background &
    BACKGROUND_PID=$!
    echo -e "${GREEN}Background service started with PID: ${BACKGROUND_PID}${NC}"
    # Wait a moment for the background service to initialize
    sleep 1
fi

# Start the UI
echo -e "${GREEN}Starting Orion UI...${NC}"
${BIN_DIR}/popup_ui

echo -e "${GREEN}Orion UI closed.${NC}"
