use rgb::rgb;
use piston_window::{
    Context,
    G2d,
    line::Line,
    clear,
    Size,
    draw_state::DrawState
};

pub struct Game {
    grid_width: f64,
    grid_size: [u32; 2],
    window_size: Size,
}

impl Game {
    pub fn new(window_size: Size) -> Game {
        Game {
            grid_width: 2.0, // note to self: make it relative! relative = good
            grid_size: [10, 20],
            window_size,
        }
    }
    pub fn render(&self, c: Context, g: &mut G2d) {
        clear([0.0, 0.0, 0.0, 1.0], g);
        let grid = Line::new(rgb(105, 106, 109, 0.5), self.grid_width);
        for i in 0..self.grid_size[0] + 1 {
            let draw_state = DrawState::new_alpha();
            let x = (self.window_size.width as f64 - self.grid_width) / 10.0;
            grid.draw([x * i as f64, 0.0, x * i as f64, self.window_size.height as f64], &draw_state, c.transform, g); //unexpected behavior: grid doesn't get made correctly
        }
    }
}
