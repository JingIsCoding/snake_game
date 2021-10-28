use opengl_graphics::{GlGraphics};
use piston::input::{ RenderArgs, UpdateArgs };
use super::position::*;
use super::food::*;
use super::game::*;
use std::collections::VecDeque;

pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

pub struct Snake {
    pub dir: Direction,
    pub body: VecDeque<Body>
}

impl Snake {
    pub fn new(init_pos: Option<Position>) -> Self {
        let mut body = VecDeque::new();
        if let Some(pos) = init_pos {
            body.push_front(Body::new(pos));
        } else {
            body.push_front(Body::new(Position::new(0, 0)));
        }
        Snake{
            body: body,
            dir: Direction::Right
        }
    }

    pub fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        for body in self.body.iter() {
            body.render(gl, args);
        }
    }

    pub fn update(&mut self, foods: &mut Vec<Food>, args: &UpdateArgs) {
        self.move_snake(foods);
    }

    fn move_snake(&mut self, foods: &mut Vec<Food>) {
        let head = self.body.front();
        if let Some(head) = head {
            let mut new_pos = match self.dir {
                Direction::Up => {
                    Position{
                        x: head.pos.x,
                        y: head.pos.y - 1,
                    }
                }
                Direction::Right => {
                    Position{
                        x: head.pos.x + 1,
                        y: head.pos.y,
                    }
                }
                Direction::Down => {
                    Position{
                        x: head.pos.x,
                        y: head.pos.y + 1,
                    }
                }
                Direction::Left => {
                    Position{
                        x: head.pos.x - 1,
                        y: head.pos.y,
                    }
                }
            };
            new_pos.x = if new_pos.x < 0 { WIDTH } else { new_pos.x };
            new_pos.x = if new_pos.x > WIDTH { 0 } else { new_pos.x };
            new_pos.y = if new_pos.y < 0 { HEIGHT } else { new_pos.y };
            new_pos.y = if new_pos.y > HEIGHT { 0 } else { new_pos.y };

            if let Some(index) = foods.iter().position(|food| food.pos.is_same(&new_pos)) {
                foods.remove(index);
            } else {
                self.body.pop_back();
            }
            self.body.push_front(Body::new(new_pos));
        }
    }

    pub fn change_dir(&mut self, dir: Direction) {
        self.dir = dir;
    }
}

pub struct Body {
    pub pos: Position,
    square: graphics::types::Rectangle
}

impl Body {
    fn new(pos: Position) -> Self {
        let square = graphics::rectangle::square(pos.x as f64 * TILE_SIZE , pos.y as f64  * TILE_SIZE, TILE_SIZE);
        Body {
            pos: pos,
            square: square,
        }
    }

    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            graphics::rectangle(RED, self.square, transform, gl);
        })
    }
}

