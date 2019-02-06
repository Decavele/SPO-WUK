extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};

struct Position {
    x: f64,
    y: f64,
}

pub struct Actor {
    position: Position
}

impl Actor {
    pub fn render(&mut self, GlGraphics gl, args: &RenderArgs) {
        use graphics::*;

        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);

        let (x, y) = (self.position.x, self.position.y);

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform.trans(x, y);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
        });
    }


    pub fn move_left(&mut self) {
        self.position.x -= 10.0;
    }

    pub fn move_right(&mut self) {
        self.position.x += 10.0;
    }

    pub fn move_up(&mut self) {
        self.position.y -= 10.0;
    }

    pub fn move_down(&mut self) {
        self.position.y += 10.0;
    }
}