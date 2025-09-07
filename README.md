# Minesweeper

A Minesweeper clone built with Rust.

## Features

- **Complete Minesweeper gameplay** with proper mine placement and number calculation
- **Multiple difficulty levels**: Beginner (9x9, 10 mines), Intermediate (16x16, 40 mines), Expert (30x16, 99 mines)
- **Timer system** to track game duration

## Game Controls

- **Left click**: Reveal a cell
- **Right click**: Flag/unflag a cell
- **ESC**: Pause/Un-Pause the game

## How to Play

1. Click on cells to reveal them
2. Numbers show how many mines are adjacent to that cell
3. Right-click to flag cells you think contain mines
4. Avoid clicking on mines!
5. Reveal all non-mine cells to win

## Building and Running

```bash
# Build the project
cargo build

# Run the game
cargo run

# Build for release (optimized)
cargo build --release
```

## Creating a macOS App Bundle

```bash
# Install cargo-bundle (if not already installed)
cargo install cargo-bundle

# Create the macOS app bundle with custom icon
./build.sh

# The app bundle will be created at:
# target/release/bundle/osx/Minesweeper.app

# Run the app bundle
open target/release/bundle/osx/Minesweeper.app
```

## Debug mode

```bash
cargo run -- --debug --show-mines
```
