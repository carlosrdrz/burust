# Burust Game Engine

A Rust-based game engine with a sample game implementation.

## Installation Guide

This guide will walk you through the process of setting up the Burust game engine and sample game on your system.

### 1. Install Rust Toolchain

First, you need to install Rust using rustup, the Rust toolchain installer:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the on-screen instructions to complete the installation. Once installed, make sure to add Rust to your PATH:

```bash
source $HOME/.cargo/env
```

Verify the installation:

```bash
rustc --version
cargo --version
```

### 2. Install SDL2 Dependencies (macOS)

The Burust engine uses SDL2 for rendering. On macOS, you can install the required dependencies using Homebrew:

```bash
# Install Homebrew if you don't have it
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install SDL2 and SDL2_image
brew install sdl2 sdl2_image
```

### 3. Build the Project

Clone the repository (if you haven't already):

```bash
git clone <repository-url>
cd burust
```

Build the project using Cargo:

```bash
# Build the engine library
cd burengine
cargo build

# Build the game
cd ../burgame
cargo build
```

For a release build (optimized):

```bash
cargo build --release
```

### 4. Run the Game

From the project root:

```bash
cd burgame
cargo run
```

Or for a release build:

```bash
cargo run --release
```

## Project Structure

- `burengine/`: The game engine library
  - Provides rendering, game loop, and UI management functionality
  - Uses SDL2 for graphics rendering

- `burgame/`: A sample game implementation
  - Uses the burengine library
  - Contains example game code and resources

## Development

To work on the project, you can use any Rust-compatible IDE or editor. Visual Studio Code with the rust-analyzer extension is recommended.

### Updating Dependencies

If you need to update the SDL2 dependencies:

```bash
brew update
brew upgrade sdl2 sdl2_image
```

## Troubleshooting

### SDL2 Library Not Found

If you encounter errors related to SDL2 libraries not being found, you may need to set the library path:

```bash
export LIBRARY_PATH="$LIBRARY_PATH:$(brew --prefix)/lib"
```

### Linker Errors

For linker errors on macOS, you might need to specify the SDL2 path explicitly:

```bash
export LIBRARY_PATH="$LIBRARY_PATH:$(brew --prefix sdl2)/lib:$(brew --prefix sdl2_image)/lib"
export CPATH="$CPATH:$(brew --prefix sdl2)/include:$(brew --prefix sdl2_image)/include"
```
