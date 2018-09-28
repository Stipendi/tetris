use rgb::rgb;
use piston_window::{
    Context,
    G2d,
    line::Line,
    clear,
    Size,
    draw_state::DrawState
};

struct Grid {
    size: [u32; 2],
    width: f64,
    color: [f32; 4]
}

impl Grid {
    fn new(size: [u32; 2], width: f64, color: [f32; 4]) -> Grid {
        Grid {
            size,
            width,
            color,
        }
    }
}

impl Default for Grid {
    fn default() -> Self {
        Grid {
            size: [10, 20],
            width: 2.0,
            color: rgb(105, 106, 109, 0.5),
        }
    }
}

pub struct Game {
    grid: Option<Grid>,
    window_size: Size,
}

impl Game {
    pub fn new(window_size: Size) -> Game {
        Game {
            grid: Some(Grid::default()),
            window_size,
        }
    }
    pub fn render(&self, c: Context, g: &mut G2d) {
        clear([0.0, 0.0, 0.0, 1.0], g);
        if let Some(grid) = &self.grid {
            let line = Line::new(grid.color, grid.width);
            for i in 0..grid.size[0] + 1 {
                let draw_state = DrawState::new_alpha();
                let x: f64 = (self.window_size.width as f64 - grid.width * 2.0) / grid.size[0] as f64;
                line.draw([x * i as f64 + grid.width, 0.0, x * i as f64 + grid.width, self.window_size.height as f64], &draw_state, c.transform, g);
            }
            for i in 0..grid.size[1] + 1 {
                let draw_state = DrawState::new_alpha();
                let y: f64 = (self.window_size.height as f64 - grid.width * 2.0) / grid.size[1] as f64;
                line.draw([0.0, y * i as f64 + grid.width, self.window_size.width as f64, y * i as f64 + grid.width], &draw_state, c.transform, g);
            }
        }
    }
}