use piston::input::{ RenderArgs, UpdateArgs };
use opengl_graphics::{GlGraphics};
use super::snake::*;
use super::position::*;
use super::game::*;

pub struct Food {
    pub pos: Position
}

impl Food {
    pub fn new(pos: Position) -> Self {
        Food{
            pos: pos
        }
    }

    pub fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        let square = graphics::rectangle::square(self.pos.x as f64 * TILE_SIZE , self.pos.y as f64  * TILE_SIZE, TILE_SIZE);
        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            graphics::rectangle(GREEN, square, transform, gl);
        })
    }

    pub fn update(&mut self, args: &UpdateArgs) {
    }
}
