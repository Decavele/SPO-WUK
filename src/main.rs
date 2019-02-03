extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};

pub struct App {
    gl: GlGraphics,
    // OpenGL drawing backend.
    position: Position,
}

pub struct Position {
    x: f64,
    y: f64,
}

pub struct ViewPort {
    height: u64,
    width: u64,
}


impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);

        let (x, y) = (self.position.x, self.position.y);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            let transform = c.transform.trans(x, y);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
        });
    }

    fn update(&mut self, button: &Button) {
        if button.eq(&Button::from(Key::Up)) {
            self.position.y -= 10.0;
        }

        if button.eq(&Button::from(Key::Down)) {
            self.position.y += 10.0;
        }

        if button.eq(&Button::from(Key::Left)) {
            self.position.x -= 10.0;
        }
        if button.eq(&Button::from(Key::Right)) {
            self.position.x += 10.0;
        }
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    let viewport = ViewPort {
        height: 500,
        width: 500,
    };

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
        "SPO-WUK",
        [500, 500],
    )
        .opengl(opengl)
        .fullscreen(true)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        position: Position {
            x: (viewport.width / 2) as f64,
            y: (viewport.height / 2) as f64,
        },
    };

    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.press_args() {
            app.update(&u);
        }
    }
}