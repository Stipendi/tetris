extern crate piston_window;
mod rgb;
mod game;
use piston_window::{
    PistonWindow,
    WindowSettings,
    clear,
    Window
};
use game::{
    Game
};

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Tetris!", (200, 400))
        .build()
        .expect("Fatal: Failed to build window!");
    let game = Game::new(window.window.size());
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            game.render(c, g);
        });
    }
}
