#![deny(missing_docs)]

//! A simple Sudoku Game based on the piston library

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate serde;

use glutin_window::GlutinWindow;
use opengl_graphics::{Filter, GlGraphics, GlyphCache, OpenGL, TextureSettings};
use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::input::RenderEvent;
use piston::window::WindowSettings;

pub use crate::game_state::GameState;
pub use crate::gameboard::Gameboard;
pub use crate::gameboard_controller::GameboardController;
pub use crate::gameboard_view::{GameboardView, GameboardViewSettings};
pub use crate::winning_state::WinningState;

mod game_state;
mod gameboard;
mod gameboard_controller;
mod gameboard_view;
mod winning_state;

fn main() {
    println!("Hello, world!");
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Sudoku", [512; 2])
        .graphics_api(opengl)
        .fullscreen(false)
        .exit_on_esc(true);
    let mut window: GlutinWindow = settings.build().expect("Could not create Window");

    println!("{}", settings.get_exit_on_esc());

    let mut events = Events::new(EventSettings::new().lazy(true));
    let mut gl = GlGraphics::new(opengl);

    let game_state = GameState::new();
    let mut gameboard_controller = GameboardController::new(game_state);
    let gameboard_view_settings = GameboardViewSettings::new();
    let gameboard_view = GameboardView::new(gameboard_view_settings);

    let texture_settings = TextureSettings::new().filter(Filter::Nearest);
    let ref mut glyphs = GlyphCache::new("assets/FiraSans-Regular.ttf", (), texture_settings)
        .expect("Could not load font");

    gameboard_controller.load_default_save();

    while let Some(e) = events.next(&mut window) {
        gameboard_controller.event(
            gameboard_view.settings.position,
            gameboard_view.settings.size,
            &e,
        );
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                use graphics::clear;

                clear([1.0; 4], g);
                gameboard_view.draw(&gameboard_controller, glyphs, &c, g);
            });
        }
    }
}
