#!/bin/bash
# Build WASM module and copy to web/src/wasm

set -e

echo "Building WASM module..."

# Navigate to wasm_frontend and build
cd ../wasm_frontend

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "Error: wasm-pack not found. Please install it:"
    echo "  cargo install wasm-pack"
    exit 1
fi

# Build WASM
wasm-pack build --target web

echo "Copying WASM files to web/src/wasm..."

# Navigate back to web directory
cd ../web

# Remove old wasm directory (if it's a symlink or directory)
rm -rf src/wasm

# Copy WASM package
cp -r ../wasm_frontend/pkg src/wasm

echo "✓ WASM build complete!"
echo "  Files copied to: src/wasm/"
ls -lh src/wasm/*.wasm
