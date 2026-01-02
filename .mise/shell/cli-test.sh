#!/bin/bash
# Test CLI functionality
# This script tests the basic CLI commands to ensure they work correctly

set -e

echo "Testing Roro Kube CLI..."
echo ""

# Test 1: Help command
echo "Test 1: Help command"
cargo run -p roro_cli -- --help > /dev/null 2>&1 && echo "✓ Help command works" || (echo "✗ Help command failed" && exit 1)
echo ""

# Test 2: Status command
echo "Test 2: Status command"
cargo run -p roro_cli -- status && echo "✓ Status command works" || (echo "✗ Status command failed" && exit 1)
echo ""

# Test 3: Workspace status command (may fail if no workspace, that's ok)
echo "Test 3: Workspace status command"
if cargo run -p roro_cli -- workspace status 2>&1; then
    echo "✓ Workspace status command works"
else
    echo "⚠ Workspace status command failed (expected if no workspace exists)"
fi
echo ""

# Test 4: Sync command help
echo "Test 4: Sync command help"
cargo run -p roro_cli -- sync --help > /dev/null 2>&1 && echo "✓ Sync command help works" || (echo "✗ Sync command help failed" && exit 1)
echo ""

# Test 5: Build check
echo "Test 5: Build check"
cargo check -p roro_cli > /dev/null 2>&1 && echo "✓ CLI crate compiles" || (echo "✗ CLI crate compilation failed" && exit 1)
echo ""

echo "All CLI tests passed!"
