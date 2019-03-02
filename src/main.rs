extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};

mod modules;

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    let viewport = modules::stage::ViewPort {
        width: 500,
        height: 500,
    };

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
        "SPO-WUK",
        [viewport.width, viewport.height],
    )
    .opengl(opengl)
    .fullscreen(true)
    .exit_on_esc(true)
    .build()
    .unwrap();

    // Create a new game and run it.
    let mut stage = modules::stage::Stage {
        gl: GlGraphics::new(opengl)
    };

    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            stage.render(&r);
        }

        if let Some(u) = e.press_args() {
            stage.update(&u);
        }
    }
}