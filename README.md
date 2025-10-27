# SPO-WUK

SPO-WUK?! is a cooperative 2D arcade game written in Rust, inspired by classic Pac-Man gameplay. Built with the Piston game engine and OpenGL, this project reimagines arcade classics with custom characters and cooperative mechanics.

## Current Status

**Version:** 0.1.0 (Early Alpha)

Currently implemented:
- Basic 2D rendering with OpenGL 3.2
- Arrow key player movement
- Fullscreen game window
- Simple game loop architecture

## Quick Start

### Prerequisites
- Rust toolchain (install from https://rustup.rs/)
- OpenGL 3.2+ compatible graphics driver

### Build & Run
```bash
cargo run
```

### Controls
- **Arrow Keys** - Move player
- **ESC** - Exit game

## Project Structure

```
SPO-WUK/
├── src/main.rs          # Game logic (108 lines)
├── Cargo.toml           # Project configuration
├── README.md            # This file
├── DOCUMENTATION.md     # Comprehensive documentation
├── CONCEPT.md           # Game concept & characters
└── LICENSE              # BSD 3-Clause License
```

## Documentation

For detailed information, see:
- **[DOCUMENTATION.md](DOCUMENTATION.md)** - Complete technical documentation
  - Architecture & design
  - Implementation details
  - Development roadmap
  - API reference
- **[CONCEPT.md](CONCEPT.md)** - Game concept & character info

## Technology Stack

- **Language:** Rust (Edition 2018)
- **Game Engine:** Piston 0.40.0
- **Graphics:** OpenGL 3.2 via piston2d-opengl_graphics
- **Window Management:** Glutin Window

## Authors

- Jan Decavele
- Thomas Decavele

## License

BSD 3-Clause License - Copyright 2019, Decavele
 