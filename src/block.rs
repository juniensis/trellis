use serde::{Deserialize, Serialize};

use trellis_core::terminal::Cell;
use trellis_graphics::{
    primitives::{Primitive, RenderCtx, Renderable},
    shapes::Rectangle,
};

use crate::world::{
    bounds::{BoundingBox, Bounds},
    id::Id,
};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Text {
    lines: Vec<String>,
}

impl Text {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn insert(&mut self, line: usize, byte_column: usize, ch: char) {
        while self.lines.len() <= line {
            self.lines.push(String::new());
        }

        if let Some(l) = self.lines.get_mut(line) {
            l.insert(byte_column, ch);
        }
    }
    pub fn insert_str(&mut self, line: usize, byte_column: usize, str: &str) {
        while self.lines.len() <= line {
            self.lines.push(String::new());
        }

        if let Some(l) = self.lines.get_mut(line) {
            l.insert_str(byte_column, str);
        }
    }
    pub fn delete(&mut self, line: usize, byte_column: usize) {
        if let Some(l) = self.lines.get_mut(line) {
            l.remove(byte_column);
        }
    }
    pub fn split_line(&mut self, line: usize, byte_column: usize) {
        if let Some(l) = self.lines.get_mut(line) {
            let rest = l.split_off(byte_column);
            self.insert_str(line + 1, 0, &rest);
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Block {
    pub id: Id,
    pub world_x: i64,
    pub world_y: i64,
    pub width: i64,
    pub height: i64,
    pub corners: [char; 4],
    pub verticals: char,
    pub horizontals: char,
    pub contents: Text,
}

impl Block {
    pub fn bounds(&self) -> Bounds {
        Bounds::Box(BoundingBox::new(
            self.world_x,
            self.world_y,
            self.world_x + self.width,
            self.world_y + self.height,
        ))
    }
    pub fn update_width(&mut self) {
        self.width = self
            .contents
            .lines
            .iter()
            .map(|x| x.len())
            .max()
            .unwrap_or(0) as i64
            + 1;
    }
    pub fn update_height(&mut self) {
        self.height = self.contents.lines.len() as i64 + 1;
    }
}

impl Renderable for Block {
    fn render(&self, _ctx: RenderCtx) -> Primitive {
        let mut rect = Rectangle::new(self.width as u32, self.height as u32, 1).render(_ctx);

        for (y, line) in self.contents.lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let y = y + 1;
                let x = x + 1;
                rect.set_cell((x, y), Cell::new(ch))
            }
        }

        rect
    }
}
