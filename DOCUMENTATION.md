# SPO-WUK - Complete Documentation

## Table of Contents
1. [Project Overview](#project-overview)
2. [Architecture](#architecture)
3. [Setup & Installation](#setup--installation)
4. [Code Structure](#code-structure)
5. [Implementation Details](#implementation-details)
6. [Game Controls](#game-controls)
7. [Technical Specifications](#technical-specifications)
8. [Development Roadmap](#development-roadmap)
9. [Contributing](#contributing)

---

## Project Overview

**SPO-WUK** is a cooperative 2D arcade game written in Rust, inspired by classic Pac-Man gameplay. The project is currently in early alpha (v0.1.0) and serves as a playful reimagining/parody featuring custom characters.

### Key Features (Current)
- Basic 2D rendering using OpenGL
- Arrow key-based player movement
- Fullscreen game window
- Simple game loop architecture

### Project Goals
Create a cooperative arcade experience with:
- Multiple playable characters (Spoweken)
- Classic arcade-style gameplay
- Modern Rust game engine foundation

---

## Architecture

### Technology Stack

| Component | Technology | Version |
|-----------|-----------|---------|
| Language | Rust | Edition 2018 |
| Game Engine | Piston | 0.40.0 |
| Window Management | Piston Window | 0.87.0 |
| Graphics Backend | OpenGL Graphics | 0.58.0 |
| Graphics API | OpenGL | 3.2 |
| Window Context | Glutin Window | 0.52.0 |
| 2D Graphics | Piston 2D Graphics | 0.29.0 |

### System Architecture

```
┌─────────────────────────────────────────┐
│            Main Game Loop               │
│         (Events::new())                 │
└────────────┬────────────────────────────┘
             │
             ├──► Render Events ──► App::render()
             │                      └──► GlGraphics draw
             │
             └──► Input Events ───► App::update()
                                    └──► Position update
```

### Core Components

1. **App** - Main game application struct
   - Manages OpenGL graphics backend
   - Tracks game state (player position)
   - Handles rendering and updates

2. **Position** - Player coordinate tracking
   - X/Y coordinate system
   - F64 precision for smooth movement

3. **ViewPort** - Screen dimension management
   - Fixed 500x500 pixel display
   - Used for initial player positioning

---

## Setup & Installation

### Prerequisites

1. **Rust Toolchain** (required)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
   Or visit: https://rustup.rs/

2. **OpenGL Support** (required)
   - OpenGL 3.2 or later
   - If not available, modify `main.rs:71` to use `OpenGL::V2_1`

### Building the Project

```bash
# Clone the repository
git clone http://local_proxy@127.0.0.1:60196/git/Decavele/SPO-WUK
cd SPO-WUK

# Build and run in one command
cargo run

# Or build separately
cargo build --release
./target/release/SPO-WUK
```

### Dependencies

All dependencies are managed by Cargo and specified in `Cargo.toml`. The project uses:
- **piston** - Core game engine framework
- **piston_window** - Window and event handling
- **piston2d-graphics** - 2D rendering primitives
- **pistoncore-glutin_window** - Cross-platform window creation
- **piston2d-opengl_graphics** - OpenGL rendering backend

Dependencies are locked via `Cargo.lock` for reproducible builds.

---

## Code Structure

### Directory Layout

```
SPO-WUK/
├── src/
│   └── main.rs          # All game logic (108 lines)
├── Cargo.toml           # Project manifest & dependencies
├── Cargo.lock           # Locked dependency versions
├── README.md            # Basic project info
├── CONCEPT.md           # Game concept & character names
├── DOCUMENTATION.md     # This file
├── LICENSE              # BSD 3-Clause License
└── .gitignore          # Git ignore rules
```

### Source Code Organization

Currently monolithic - all code resides in `/home/user/SPO-WUK/src/main.rs`

**Suggested future structure:**
```
src/
├── main.rs              # Entry point & game loop
├── app.rs               # App struct & core logic
├── renderer.rs          # Rendering logic
├── input.rs             # Input handling
├── entities/
│   ├── player.rs        # Player entity
│   └── enemy.rs         # Enemy entities (Spoweken)
└── utils/
    └── position.rs      # Position & viewport utilities
```

---

## Implementation Details

### Data Structures

#### App Struct
**Location:** `/home/user/SPO-WUK/src/main.rs:12-16`

```rust
pub struct App {
    gl: GlGraphics,      // OpenGL drawing backend
    position: Position,  // Current player position
}
```

**Responsibilities:**
- Manages OpenGL graphics context
- Stores game state (player position)
- Provides render and update methods

#### Position Struct
**Location:** `/home/user/SPO-WUK/src/main.rs:18-21`

```rust
pub struct Position {
    x: f64,  // Horizontal coordinate
    y: f64,  // Vertical coordinate
}
```

**Usage:**
- Tracks player location on screen
- Uses f64 for smooth sub-pixel movement
- Initialized to center of viewport (250, 250)

#### ViewPort Struct
**Location:** `/home/user/SPO-WUK/src/main.rs:23-26`

```rust
pub struct ViewPort {
    height: u64,  // Window height in pixels
    width: u64,   // Window width in pixels
}
```

**Current Configuration:**
- Fixed 500x500 pixel window
- Used only for initial player centering

### Core Methods

#### App::render()
**Location:** `/home/user/SPO-WUK/src/main.rs:30-49`

```rust
fn render(&mut self, args: &RenderArgs)
```

**Purpose:** Renders the game scene each frame

**Implementation:**
1. Clears screen with green background (`[0.0, 1.0, 0.0, 1.0]`)
2. Creates a 50x50 pixel square shape
3. Translates square to current player position
4. Draws red square (`[1.0, 0.0, 0.0, 1.0]`) at position

**Color Constants:**
- `GREEN`: Background color - `[0.0, 1.0, 0.0, 1.0]` (RGBA)
- `RED`: Player color - `[1.0, 0.0, 0.0, 1.0]` (RGBA)

#### App::update()
**Location:** `/home/user/SPO-WUK/src/main.rs:51-67`

```rust
fn update(&mut self, button: &Button)
```

**Purpose:** Handles keyboard input and updates game state

**Movement Logic:**
- **Up Arrow:** Decreases Y by 10 pixels (`y -= 10.0`)
- **Down Arrow:** Increases Y by 10 pixels (`y += 10.0`)
- **Left Arrow:** Decreases X by 10 pixels (`x -= 10.0`)
- **Right Arrow:** Increases X by 10 pixels (`x += 10.0`)

**Note:** No bounds checking - player can move off-screen

### Main Game Loop
**Location:** `/home/user/SPO-WUK/src/main.rs:69-109`

**Initialization Sequence:**
1. Set OpenGL version (3.2)
2. Create viewport specification (500x500)
3. Configure window:
   - Title: "SPO-WUK"
   - Dimensions: 500x500 pixels
   - Fullscreen mode enabled
   - ESC key exits application
4. Initialize App with:
   - OpenGL graphics backend
   - Player centered at (250, 250)
5. Create event loop

**Event Loop:**
```rust
while let Some(e) = events.next(&mut window) {
    if let Some(r) = e.render_args() {
        app.render(&r);  // Render frame
    }

    if let Some(u) = e.press_args() {
        app.update(&u);  // Handle input
    }
}
```

**Event Types Handled:**
- **Render events:** Trigger frame rendering
- **Press events:** Trigger input updates (arrow keys)

---

## Game Controls

### Keyboard Controls

| Key | Action |
|-----|--------|
| **↑** (Up Arrow) | Move player up (Y - 10) |
| **↓** (Down Arrow) | Move player down (Y + 10) |
| **←** (Left Arrow) | Move player left (X - 10) |
| **→** (Right Arrow) | Move player right (X + 10) |
| **ESC** | Exit game |

### Current Limitations

1. **No boundary detection** - Player can move off-screen
2. **No diagonal movement smoothing** - Can move in 4 directions only
3. **Fixed movement speed** - Always 10 pixels per keypress
4. **No animation** - Instant position updates
5. **Press-based movement** - No continuous movement while holding keys

---

## Technical Specifications

### Graphics Rendering

**Render Pipeline:**
1. Clear viewport with solid green fill
2. Create square primitive (50x50 pixels)
3. Apply transformation matrix (translation to player position)
4. Draw square with red fill color
5. Present frame buffer

**Performance Characteristics:**
- **Target FPS:** Unlimited (determined by Piston event loop)
- **VSync:** Controlled by window manager
- **Draw Calls:** 2 per frame (clear + rectangle)

### Memory Management

**Ownership Model:**
- `App` owns `GlGraphics` and `Position`
- `Window` owned by main function
- `Events` iterator owns event state
- All stack-allocated (no heap allocations in game loop)

**Lifetimes:**
- Graphics context lives for entire program duration
- Events consumed from iterator each frame

### Platform Support

**Supported Platforms:**
- Linux (tested)
- macOS (via Glutin)
- Windows (via Glutin)

**Requirements:**
- OpenGL 3.2+ compatible graphics driver
- Rust 1.31+ (Edition 2018)
- X11 or Wayland (Linux)

---

## Development Roadmap

### Completed (v0.1.0)
- [x] Basic Piston engine setup
- [x] OpenGL rendering pipeline
- [x] Window creation and management
- [x] Arrow key input handling
- [x] Simple player movement
- [x] Project documentation

### Planned Features

#### v0.2.0 - Core Gameplay
- [ ] Boundary collision detection
- [ ] Game map/maze generation
- [ ] Collectible items (pellets)
- [ ] Score tracking system
- [ ] Game state management (start, playing, game over)

#### v0.3.0 - Enemies & AI
- [ ] Enemy entity system (Spoweken)
  - [ ] claude
  - [ ] po-pol
  - [ ] bicky
  - [ ] rosé
  - [ ] vref
- [ ] Basic AI pathfinding
- [ ] Collision detection (player vs enemy)
- [ ] Lives system

#### v0.4.0 - Graphics & Audio
- [ ] Sprite rendering (replace colored squares)
- [ ] Animation system
- [ ] Texture loading
- [ ] Sound effects
- [ ] Background music
- [ ] Particle effects

#### v0.5.0 - Cooperative Gameplay
- [ ] Multi-player support (local)
- [ ] Player 2 controls (WASD)
- [ ] Shared score system
- [ ] Power-ups for cooperation mechanics

#### v1.0.0 - Polish & Release
- [ ] Menu system
- [ ] High score persistence
- [ ] Level progression
- [ ] Settings/options
- [ ] Gamepad support
- [ ] Performance optimization
- [ ] Cross-platform testing

### Potential Refactoring

**Code Organization:**
- Split `main.rs` into multiple modules
- Separate rendering, input, and game logic
- Create entity component system (ECS)
- Add configuration file support

**Architecture Improvements:**
- Implement state machine for game states
- Add resource management system
- Create event bus for component communication
- Delta-time based movement (frame-rate independent)

**Testing:**
- Unit tests for game logic
- Integration tests for rendering
- Benchmark performance critical paths

---

## Contributing

### Project Authors
- Jan Decavele
- Thomas Decavele

### License
BSD 3-Clause License - Copyright 2019, Decavele

### Development Guidelines

**Code Style:**
- Follow Rust standard conventions (`rustfmt`)
- Use `cargo clippy` for linting
- Document public APIs with doc comments
- Keep functions focused and small

**Git Workflow:**
1. Create feature branch from main
2. Make atomic commits with clear messages
3. Test changes locally with `cargo test`
4. Ensure `cargo build --release` succeeds
5. Submit pull request with description

**Commit Message Format:**
```
<type>: <description>

Examples:
feat: Add enemy AI pathfinding
fix: Correct player boundary detection
docs: Update installation instructions
refactor: Extract rendering into separate module
```

### Getting Help

**Repository:** http://local_proxy@127.0.0.1:60196/git/Decavele/SPO-WUK

**Building & Running Issues:**
- Ensure Rust toolchain is up to date: `rustup update`
- Check OpenGL driver support
- Verify all dependencies installed: `cargo check`

**OpenGL Compatibility:**
If the game fails to start, try OpenGL 2.1:
```rust
// In main.rs line 71
let opengl = OpenGL::V2_1;  // Change from V3_2
```

---

## Appendix: Character Reference

From `CONCEPT.md`:

**Main Character:**
- **Pac'm-Dan** - The player character (Pac-Man equivalent)

**Enemies (Spoweken):**
- **claude** - Enemy ghost
- **po-pol** - Enemy ghost
- **bicky** - Enemy ghost
- **rosé** - Enemy ghost
- **vref** - Enemy ghost

---

**Last Updated:** 2025-10-27
**Version:** 0.1.0
**Documentation Version:** 1.0
