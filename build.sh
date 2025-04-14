#!/bin/bash
set -e

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${GREEN}Starting Orion build process...${NC}"

# Create build directory
mkdir -p dist

# Build optimized binaries
echo -e "${GREEN}Building optimized binaries...${NC}"
cargo build --release --all

# Copy binaries to distribution directory
echo -e "${GREEN}Bundling components into distribution package...${NC}"
mkdir -p dist/bin

# Copy the main binaries
cp target/release/background dist/bin/
cp target/release/popup_ui dist/bin/
cp target/release/settings_app dist/bin/

# Create a distribution archive
echo -e "${GREEN}Creating distribution archive...${NC}"
VERSION=$(grep '^version' Cargo.toml | head -1 | cut -d '"' -f2)
tar -czf "dist/orion-v${VERSION}.tar.gz" -C dist bin

# Report success and locations
echo -e "${GREEN}Build completed successfully!${NC}"
echo -e "Optimized binaries are available in: ${GREEN}dist/bin/${NC}"
echo -e "Distribution archive: ${GREEN}dist/orion-v${VERSION}.tar.gz${NC}"
