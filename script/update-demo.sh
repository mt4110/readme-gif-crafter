#!/bin/bash
set -e

# Resolve project root (directory where this script resides/..)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_ROOT"

if [ ! -f "assets/demo.mp4" ]; then
    echo "assets/demo.mp4 not found in $PROJECT_ROOT."
    echo "Please place 'demo.mp4' in the 'assets' directory."
    exit 1
fi

echo "Updating demo..."
# Build release for speed
cargo build --release

# Run conversion and update Japanese README
./target/release/rgc assets/demo.mp4 --output assets/demo.gif --update-readme README.md --preset github

# Update English README using markdown-only mode (skips re-conversion)
./target/release/rgc assets/demo.gif --markdown-only --update-readme README_EN.md

echo "Done! check git status."
