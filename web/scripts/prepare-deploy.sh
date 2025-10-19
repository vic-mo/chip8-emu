#!/bin/bash
# Prepare for deployment - ensure WASM files exist

set -e

WASM_DIR="src/wasm"

# Check if WASM files exist
if [ ! -f "$WASM_DIR/wasm_frontend.js" ] || [ ! -f "$WASM_DIR/wasm_frontend_bg.wasm" ]; then
    echo "⚠️  WASM files not found in $WASM_DIR"
    echo "Building WASM module..."
    ./scripts/build-wasm.sh
else
    echo "✓ WASM files found in $WASM_DIR"
fi

# Check if ROM files exist
if [ ! -f "public/roms/catalog.json" ]; then
    echo "⚠️  ROM catalog not found"
    echo "Note: ROMs are optional. Run ./setup-roms.sh to add them."
else
    ROM_COUNT=$(find public/roms -name "*.ch8" 2>/dev/null | wc -l)
    echo "✓ ROM catalog found ($ROM_COUNT ROMs)"
fi

echo ""
echo "✓ Ready for deployment!"
