#!/bin/bash
# Check that SpanKind values match between Rust and Swift
#
# This script runs the Rust test that validates SpanKind enum values.
# If values change in Rust, both this test and Swift must be updated.
#
# Usage: ./scripts/check_spankind_sync.sh

set -e

RUST_FILE="rmde-core/src/parser.rs"
SWIFT_FILE="RMDE/Sources/App/EditorState.swift"

echo "Checking SpanKind sync between Rust and Swift..."
echo ""

# Check files exist
if [ ! -f "$RUST_FILE" ]; then
    echo "ERROR: Rust file not found: $RUST_FILE"
    exit 1
fi

if [ ! -f "$SWIFT_FILE" ]; then
    echo "ERROR: Swift file not found: $SWIFT_FILE"
    exit 1
fi

# Check Rust enum exists
if ! grep -q "pub enum SpanKind" "$RUST_FILE"; then
    echo "ERROR: SpanKind enum not found in $RUST_FILE"
    exit 1
fi

# Check Swift struct exists
if ! grep -q "struct HighlightSpan" "$SWIFT_FILE"; then
    echo "ERROR: HighlightSpan struct not found in $SWIFT_FILE"
    exit 1
fi

echo "Found SpanKind in Rust and HighlightSpan in Swift"
echo ""

# Run the Rust test which validates all values
echo "Running Rust sync test..."
cargo test test_spankind_values_for_swift_sync --quiet 2>&1

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ SpanKind values are in sync!"
    echo ""
    echo "Rust:  $RUST_FILE"
    echo "Swift: $SWIFT_FILE"
else
    echo ""
    echo "❌ SpanKind values are OUT OF SYNC!"
    echo ""
    echo "To fix:"
    echo "1. Check rmde-core/src/parser.rs SpanKind enum values"
    echo "2. Update RMDE/Sources/App/EditorState.swift HighlightSpan constants"
    echo "3. Update RMDE/Sources/Views/EditorView.swift attributesForKind()"
    exit 1
fi
