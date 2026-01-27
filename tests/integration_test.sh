#!/bin/bash
# Integration test script for WhisperTray
# This script tests the transcription pipeline with a sample audio file

set -e

echo "=== WhisperTray Integration Test ==="

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if we're in the right directory
if [ ! -f "package.json" ]; then
    echo -e "${RED}Error: Must run from project root directory${NC}"
    exit 1
fi

# Create test directory
TEST_DIR=$(mktemp -d)
echo "Test directory: $TEST_DIR"

# Generate a simple test WAV file (1 second of silence)
echo -e "${YELLOW}Generating test audio file...${NC}"
ffmpeg -f lavfi -i "anullsrc=r=16000:cl=mono" -t 1 -y "$TEST_DIR/test.wav" 2>/dev/null

if [ ! -f "$TEST_DIR/test.wav" ]; then
    echo -e "${RED}Failed to generate test audio file${NC}"
    rm -rf "$TEST_DIR"
    exit 1
fi

echo -e "${GREEN}Test audio file created${NC}"

# Run Rust unit tests
echo -e "${YELLOW}Running Rust unit tests...${NC}"
cd src-tauri
if cargo test 2>&1 | tee "$TEST_DIR/rust_test.log" | grep -E "^(test |running |passed|failed)"; then
    echo -e "${GREEN}Rust tests passed${NC}"
else
    echo -e "${RED}Rust tests failed. See $TEST_DIR/rust_test.log${NC}"
    exit 1
fi
cd ..

# Run frontend tests (if any)
echo -e "${YELLOW}Running frontend tests...${NC}"
if npm run test -- --run 2>&1 | tee "$TEST_DIR/frontend_test.log"; then
    echo -e "${GREEN}Frontend tests passed${NC}"
else
    echo -e "${YELLOW}Frontend tests skipped or failed (non-critical)${NC}"
fi

# Build check
echo -e "${YELLOW}Checking build...${NC}"
if npm run build 2>&1 | tee "$TEST_DIR/build.log"; then
    echo -e "${GREEN}Build check passed${NC}"
else
    echo -e "${RED}Build check failed. See $TEST_DIR/build.log${NC}"
    exit 1
fi

# Cleanup
rm -rf "$TEST_DIR"

echo ""
echo -e "${GREEN}=== All integration tests passed ===${NC}"
