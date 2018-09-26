extern crate piston_window;
use piston_window::{
    glutin_window::GlutinWindow,
    WindowSettings
};

fn main() {
    let mut window: GlutinWindow = WindowSettings::new("Tetris!", (200, 400))
        .build()
        .expect("Fatal: Failed to build window!");
    while let Some(e) = window.next() {
        window.draw_2d(&e, |_c, g| {
            clear([0.5, 1.0, 0.5, 1.0], g);
        });
    }
}
