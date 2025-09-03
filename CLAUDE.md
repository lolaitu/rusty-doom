# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Doom-like raycasting game engine written in Rust called "rusty-doom". The project implements a basic 3D rendering system using terminal-based graphics with crossterm for input/output handling.

## Architecture

The codebase follows a modular structure with these key components:

- **main.rs**: Entry point that initializes terminal, creates game instance, and handles cleanup
- **game.rs**: Core game loop (`Game` struct) managing timing, level, player, and main game state
- **level.rs**: Level representation (`Level` struct) with layout data and rendering methods
- **player.rs**: Player entity (`Joueur` struct) handling movement, rotation, and input processing  
- **graphics.rs**: 3D raycasting renderer that projects the 2D level into pseudo-3D terminal output

## Key Dependencies

- `crossterm 0.28.1`: Terminal manipulation, input handling, and cursor control
- `rand 0.8.5`: Random number generation (likely for game elements)

## Development Commands

```bash
# Build the project
cargo build

# Run the game
cargo run

# Run with optimizations
cargo run --release

# Check for compilation errors
cargo check

# Run tests (if any exist)
cargo test
```

## Code Conventions

- Uses French variable names and comments throughout (e.g., `Joueur` for Player, `mouvement` for movement)
- Player coordinates use floating-point (`f64`) for smooth movement
- Level layout uses `Vec<Vec<u8>>` with different numeric values representing different wall types
- Terminal coordinates are handled as `u16` for crossterm compatibility
- Error handling uses `std::io::Result<()>` pattern consistently

## Game Controls

- Arrow keys: Player movement (forward/back/strafe)
- W/X keys: Rotate player left/right  
- Space: Shoot weapon
- R: Reload weapon
- Ctrl+C: Exit game

## Terminal Optimization

For best 3D raycasting experience:
- **Reduce font size** to 8-10px for higher resolution
- **Use monospace font** (Consolas, Monaco, or terminal default)
- **Maximize terminal window** for wider FOV
- **Disable cursor blinking** if possible to reduce flicker

## Level System

Levels are represented as 2D grids where:
- `0`: Empty space
- `1,2,3,4,5`: Different wall types
- Level size is typically 24x24 tiles
- `debug_1()` method provides a test level with various wall configurations