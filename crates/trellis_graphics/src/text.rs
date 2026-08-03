use trellis_core::terminal::Cell;

use crate::primitives::{Region, Renderable};

pub struct Text {
    lines: Vec<String>,
    z_order: u32,
}

impl Text {
    pub fn new<S: AsRef<str>>(text: S, z_order: u32) -> Self {
        Self {
            lines: text.as_ref().lines().map(|x| x.to_string()).collect(),
            z_order,
        }
    }
    pub fn get_lines(&self) -> &[String] {
        self.lines.as_ref()
    }
    pub fn set_text<S: AsRef<str>>(&mut self, text: S) {
        self.lines = text.as_ref().lines().map(|x| x.to_string()).collect();
    }
    pub fn width(&self) -> u32 {
        self.lines.iter().map(|x| x.len() as u32).max().unwrap_or(0)
    }
    pub fn height(&self) -> u32 {
        self.lines.len() as u32
    }
}

impl Renderable for Text {
    fn render(&self) -> crate::primitives::Primitive {
        let mut region = Region::new(self.z_order);

        for (y, line) in self.get_lines().iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                region.set_cell(x as u32, y as u32, Cell::new(ch));
            }
        }

        region.into()
    }
}
