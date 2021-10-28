use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::{Button, Key, RenderArgs, UpdateArgs };
use super::snake::*;
use super::food::*;
use super::position::*;
use rand::prelude::*;

pub const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
pub const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
pub const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
pub const TILE_SIZE: f64 = 20_f64;
pub const WIDTH: i32 = 32;
pub const HEIGHT: i32 = 24;

pub struct Game {
    gl: GlGraphics,
    snake: Snake,
    foods: Vec<Food>,
    timer: Timer
}

impl Game {
    pub fn new(opengl: OpenGL) -> Self {
        let snake = Snake::new(Some(Position::new(10,10)));
        Game{
            gl: GlGraphics::new(opengl),
            timer: Timer::new(),
            foods: vec![],
            snake: snake
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |_c, gl| {
            graphics::clear(WHITE, gl);
        });
        self.snake.render(&mut self.gl, args);
        for food in self.foods.iter() {
            food.render(&mut self.gl, args);
        }
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.timer.tick(args.dt);
        if self.timer.has_been(0.12_f64) {
            self.timer.reset();
            self.snake.update(&mut self.foods, args);
        }
        self.generate_food();
    }

    fn generate_food(&mut self) {
        if self.foods.len() >= 3 {
            return
        }
        let mut rng = rand::thread_rng();
        let y: f64 = rng.gen();
        if y < 0.005 {
            let rand_x = rng.gen_range(0, WIDTH);
            let rand_y = rng.gen_range(0, HEIGHT);
            let pos = Position::new(rand_x, rand_y);
            for food in self.foods.iter() {
                if food.pos.is_same(&pos) {
                    return
                }
            }
            for body in self.snake.body.iter() {
                if body.pos.is_same(&pos) {
                    return
                }
            }
            let food = Food::new(pos);
            self.foods.push(food);
        }
    }

    pub fn button_press(&mut self, button: &Button) {
        match button {
            &Button::Keyboard(Key::Up) => {
                self.snake.change_dir(Direction::Up);
            },
            &Button::Keyboard(Key::Right) => {
                self.snake.change_dir(Direction::Right);
            },
            &Button::Keyboard(Key::Down) => {
                self.snake.change_dir(Direction::Down);
            },
            &Button::Keyboard(Key::Left) => {
                self.snake.change_dir(Direction::Left);
            },
            _ => ()
        }
    }
}

pub struct Timer {
    pub time_elapsed: f64,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            time_elapsed: 0_f64,
        }
    }

    pub fn tick(&mut self, dt: f64) {
        self.time_elapsed += dt;
    }

    pub fn has_been(&self, duration: f64) -> bool {
        self.time_elapsed > duration
    }
    
    pub fn reset(&mut self) {
        self.time_elapsed = 0_f64;
    }
}
