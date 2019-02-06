extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};


mod {Actor};

pub struct ViewPort {
    pub   height: u64,
    pub  width: u64,
}

pub struct Stage {
    gl: GlGraphics,
    actor: Actor,
}

impl Stage {
    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];


        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);
            self.actor.render(gl);
        });
    }

    pub fn update(&mut self, button: &Button) {}
}
