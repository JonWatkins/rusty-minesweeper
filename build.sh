#!/bin/bash
set -e
echo "Creating macOS bundle..."
cargo bundle --release
BUNDLE_PATH="target/release/bundle/osx/Minesweeper.app"
echo "Application bundle created at: $BUNDLE_PATH"
echo "Build complete! You can now copy the .app bundle to your Applications folder."
