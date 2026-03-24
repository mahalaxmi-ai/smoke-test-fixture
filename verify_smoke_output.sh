#!/bin/bash

set -o pipefail

SMOKE_OUTPUT_FILE="smoke_output.txt"

if [ ! -f "$SMOKE_OUTPUT_FILE" ]; then
    echo "FAIL: $SMOKE_OUTPUT_FILE does not exist"
    exit 1
fi

CONTENT=$(cat "$SMOKE_OUTPUT_FILE" 2>/dev/null)
if [ $? -ne 0 ]; then
    echo "FAIL: Could not read $SMOKE_OUTPUT_FILE"
    exit 1
fi

LINE_COUNT=$(echo -n "$CONTENT" | wc -l)

if [ "$LINE_COUNT" -ne 0 ]; then
    echo "FAIL: File contains trailing newline (line count: $LINE_COUNT)"
    exit 1
fi

if [ "$CONTENT" != "SMOKE_TEST_PASS" ]; then
    echo "FAIL: File content does not match expected value"
    echo "Expected: SMOKE_TEST_PASS"
    echo "Got: $CONTENT"
    exit 1
fi

echo "PASS: smoke_output.txt verified successfully"
exit 0
