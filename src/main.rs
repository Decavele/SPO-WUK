// External crate imports for the Piston game engine ecosystem
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

// Import necessary components from Piston
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};

/// Main application struct that holds the game state.
///
/// This structure manages the OpenGL graphics backend and the current
/// position of the player character on the screen.
pub struct App {
    /// OpenGL drawing backend for rendering graphics
    gl: GlGraphics,
    /// Current position of the player character
    position: Position,
}

/// Represents a 2D position on the screen using floating-point coordinates.
///
/// Uses f64 for sub-pixel precision, allowing smooth movement animations.
pub struct Position {
    /// Horizontal coordinate (X-axis)
    x: f64,
    /// Vertical coordinate (Y-axis)
    y: f64,
}

/// Defines the dimensions of the game viewport/window.
///
/// Currently used only for initial player positioning at the center of the screen.
pub struct ViewPort {
    /// Height of the viewport in pixels
    height: u64,
    /// Width of the viewport in pixels
    width: u64,
}


impl App {
    /// Renders the current game state to the screen.
    ///
    /// This method is called every frame and performs the following:
    /// 1. Clears the screen with a green background
    /// 2. Creates a 50x50 pixel square
    /// 3. Positions the square at the player's current location
    /// 4. Draws the square in red
    ///
    /// # Arguments
    ///
    /// * `args` - Render arguments provided by the Piston event loop,
    ///            containing viewport and frame timing information
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        // Color constants in RGBA format (values 0.0 to 1.0)
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0]; // Background color
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];   // Player color

        // Create a square shape (50x50 pixels) at origin (0,0)
        let square = rectangle::square(0.0, 0.0, 50.0);

        // Get current player position
        let (x, y) = (self.position.x, self.position.y);

        // Draw to the screen
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen with green background
            clear(GREEN, gl);

            // Create transformation matrix to translate square to player position
            let transform = c.transform.trans(x, y);

            // Draw the red square at the player's position
            rectangle(RED, square, transform, gl);
        });
    }

    /// Updates the game state based on keyboard input.
    ///
    /// Handles arrow key presses to move the player character:
    /// - Up/Down: Moves vertically by 10 pixels
    /// - Left/Right: Moves horizontally by 10 pixels
    ///
    /// Note: Currently has no boundary checking, so the player can move
    /// off-screen. Future versions should add collision detection.
    ///
    /// # Arguments
    ///
    /// * `button` - The button/key that was pressed
    fn update(&mut self, button: &Button) {
        // Move up (decrease Y coordinate)
        if button.eq(&Button::from(Key::Up)) {
            self.position.y -= 10.0;
        }

        // Move down (increase Y coordinate)
        if button.eq(&Button::from(Key::Down)) {
            self.position.y += 10.0;
        }

        // Move left (decrease X coordinate)
        if button.eq(&Button::from(Key::Left)) {
            self.position.x -= 10.0;
        }

        // Move right (increase X coordinate)
        if button.eq(&Button::from(Key::Right)) {
            self.position.x += 10.0;
        }
    }
}

/// Main entry point for the SPO-WUK game.
///
/// Initializes the game window, graphics backend, and runs the main game loop.
///
/// # Game Loop
///
/// The game loop processes two types of events:
/// 1. Render events - triggers rendering of the current frame
/// 2. Press events - triggers input handling for keyboard presses
///
/// # Window Configuration
///
/// - Size: 500x500 pixels (fullscreen)
/// - OpenGL Version: 3.2 (change to V2_1 if compatibility issues occur)
/// - Exit: ESC key closes the application
fn main() {
    // Set OpenGL version. Change to OpenGL::V2_1 if V3_2 is not supported.
    let opengl = OpenGL::V3_2;

    // Define viewport dimensions
    let viewport = ViewPort {
        height: 500,
        width: 500,
    };

    // Create a Glutin window with specified settings
    let mut window: Window = WindowSettings::new(
        "SPO-WUK",        // Window title
        [500, 500],       // Window dimensions [width, height]
    )
        .opengl(opengl)   // Set OpenGL version
        .fullscreen(true) // Run in fullscreen mode
        .exit_on_esc(true) // Exit when ESC key is pressed
        .build()
        .unwrap();

    // Initialize the game application
    let mut app = App {
        // Create OpenGL graphics backend
        gl: GlGraphics::new(opengl),
        // Start player at center of viewport
        position: Position {
            x: (viewport.width / 2) as f64,  // Center X (250 pixels)
            y: (viewport.height / 2) as f64, // Center Y (250 pixels)
        },
    };

    // Create event loop with default settings
    let mut events = Events::new(EventSettings::new());

    // Main game loop - runs until window is closed
    while let Some(e) = events.next(&mut window) {
        // Handle render events - called every frame
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        // Handle button press events - called when keys are pressed
        if let Some(u) = e.press_args() {
            app.update(&u);
        }
    }
}