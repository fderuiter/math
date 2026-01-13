#!/bin/bash
set -e

# Function to check if a command exists
check_command() {
    if ! command -v "$1" &> /dev/null; then
        echo "Error: $1 could not be found."
        return 1
    else
        echo "Found $1: $(command -v "$1")"
        return 0
    fi
}

echo "=== Math Explorer Setup Script ==="

# 1. Check for Rust toolchain
echo ""
echo "--- Checking Prerequisites ---"
if ! check_command cargo; then
    echo "Please install Rust and Cargo from https://rustup.rs/"
    exit 1
fi

if ! check_command rustc; then
    echo "Please install Rust and Cargo from https://rustup.rs/"
    exit 1
fi

# Check for pdflatex (Optional)
if check_command pdflatex; then
    HAS_LATEX=true
    echo "LaTeX found. Paper generation will be available."
else
    HAS_LATEX=false
    echo "Warning: pdflatex not found. You will not be able to compile the papers."
    echo "This does not affect the Rust code."
fi

# 2. Build the Rust project
echo ""
echo "--- Building Rust Project ---"
cd math_explorer
echo "Running 'cargo build'..."
cargo build
echo "Build successful."

# 3. Run Tests
echo ""
echo "--- Running Tests ---"
echo "Running 'cargo test'..."
cargo test
echo "Tests passed."

# Return to root
cd ..

echo ""
echo "=== Setup Complete ==="
echo "You are ready to explore math!"
if [ "$HAS_LATEX" = false ]; then
    echo "Note: Install a LaTeX distribution (like TeX Live) to compile papers in the 'papers/' directory."
fi
