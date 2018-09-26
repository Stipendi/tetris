extern crate piston_window;
mod game;
use piston_window::{
    PistonWindow,
    WindowSettings,
    clear
};
use game::{
    Game
};

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Tetris!", (200, 400))
        .build()
        .expect("Fatal: Failed to build window!");
    let game = Game::new();
    while let Some(e) = window.next() {
        window.draw_2d(&e, |_c, g| {

            clear([0.5, 1.0, 0.5, 1.0], g);
        });
    }
}
