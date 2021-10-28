extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod game;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{OpenGL};
use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::window::WindowSettings;
use piston::input::{ ButtonState, RenderEvent, UpdateEvent, ButtonEvent };
use game::game::*;

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: Window = WindowSettings::new("shapes", [640, 480])
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();
    let mut game = Game::new(opengl);
    let mut events = Events::new(EventSettings::new()).ups(60);
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            game.render(&args);
        }
        if let Some(args) = e.update_args() {
            game.update(&args);
        }
        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                game.button_press(&k.button);
            }
        }
    }
}
