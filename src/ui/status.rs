use trellis_graphics::primitives::{Primitive, RenderCtx, Renderable};

use crate::input::State;

pub struct StatusBar {
    state: State,
    coords: (i64, i64),
    max_width: u32,
}

impl StatusBar {
    pub fn new(x: i64, y: i64, width: u32) -> Self {
        Self {
            state: State::Normal,
            coords: (x, y),
            max_width: width,
        }
    }
    pub fn set_state(&mut self, state: State) {
        self.state = state;
    }
    pub fn set_max_width(&mut self, width: u32) {
        self.max_width = width;
    }
    pub fn set_coords(&mut self, x: i64, y: i64) {
        self.coords = (x, y);
    }
}

impl Renderable for StatusBar {
    fn render(&self, _ctx: RenderCtx) -> Primitive {
        let mut region = Primitive::new_region(255);

        match self.state {
            State::Normal => {
                region.write_str((1, 0), "NORMAL");
            }
            State::Insert => {
                region.write_str((1, 0), "INSERT");
            }
        }

        let coords = format!("({}, {})", self.coords.0, self.coords.1);
        region.write_str((self.max_width - coords.len() as u32, 0), coords);

        region
    }
}
