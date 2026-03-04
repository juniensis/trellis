use trellis_core::terminal::{cell::Cell, point::Point};

use crate::render::{
    region::Region,
    styles::{RectangleBorder, RectangleStyle},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub width: u16,
    pub height: u16,
    pub style: RectangleStyle,
}

impl Rect {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            style: RectangleStyle::new(),
        }
    }
    pub fn with_fill(mut self, fill: Cell) -> Self {
        self.style = self.style.with_fill(fill);
        self
    }
    pub fn with_border(mut self, border: RectangleBorder) -> Self {
        self.style = self.style.with_border(border);
        self
    }
    pub fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
    }
    pub fn resize_by(&mut self, dw: i16, dh: i16) {
        self.width = self.width.saturating_add_signed(dw);
        self.height = self.height.saturating_add_signed(dh);
    }
    pub fn render(&self, mut region: Region) -> Region {
        let mut region = if let Some(fill) = self.style.fill {
            for row in 0..self.height {
                for col in 0..self.width {
                    region.set_cell(col, row, fill);
                }
            }
            region
        } else {
            self.clear(region)
        };

        if let Some(border) = self.style.border {
            let (corners, hori, vert) = border.into_components();
            for col in 1..self.width.saturating_sub(1) {
                region.set_cell(col, 0, hori);
                region.set_cell(col, self.height - 1, hori);
            }

            for row in 1..self.height.saturating_sub(1) {
                region.set_cell(0, row, vert);
                region.set_cell(self.width - 1, row, vert);
            }

            region.set_cell(0, 0, corners[0]);
            region.set_cell(self.width - 1, 0, corners[1]);
            region.set_cell(self.width - 1, self.height - 1, corners[2]);
            region.set_cell(0, self.height - 1, corners[3]);
        }
        region
    }
    pub fn clear(&self, mut region: Region) -> Region {
        for row in 0..self.height {
            for col in 0..self.width {
                region.set_cell(col, row, Cell::null());
            }
        }
        region
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Circle {
    pub radius: u16,
}

impl Circle {
    pub fn new(radius: u16) -> Self {
        Self { radius }
    }
    pub fn resize(&mut self, radius: u16) {
        self.radius = radius;
    }
    pub fn resize_by(&mut self, dr: i16) {
        self.radius = self.radius.saturating_add_signed(dr)
    }
    pub fn render(&self, mut region: Region) -> Region {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line {
    pub len: f32,
    pub angle: f32,
}

impl Line {
    pub fn new(len: f32, angle: f32) -> Self {
        Self { len, angle }
    }
    pub fn resize(&mut self, len: f32) {
        self.len = len;
    }
    pub fn resize_by(&mut self, len: f32) {
        self.len += len;
    }
    pub fn rotate_to(&mut self, angle: f32) {
        self.angle = angle;
    }
    pub fn rotate_by(&mut self, da: f32) {
        self.angle += da;
    }
    pub fn render(&self, mut region: Region) -> Region {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bezier {}

/// Vertices are given in Cartesian coordinates where 0,0 is the point the
/// polygon will be positioned around.
#[derive(Debug, Clone, PartialEq)]
pub struct Polygon {
    pub vertices: Vec<Point>,
}

impl Polygon {
    pub fn new(vertices: &[Point]) -> Self {
        Self {
            vertices: vertices.to_vec(),
        }
    }
    pub fn render(&self, mut region: Region) -> Region {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Text {
    text: String,
}

impl Text {
    pub fn new<S: ToString>(text: S) -> Self {
        Self {
            text: text.to_string(),
        }
    }
    pub fn change_text<S: ToString>(&mut self, text: S) {
        self.text = text.to_string();
    }
    pub fn render(&self, mut region: Region) -> Region {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Particle {
    pub bounds: Rect,
    pub cells: Vec<Cell>,
}

impl Particle {
    pub fn new(bounds: Rect, cells: &[Cell]) -> Self {
        Self {
            bounds,
            cells: cells.to_vec(),
        }
    }
    pub fn render(&self, mut region: Region) -> Region {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Primitive {
    Rect(Rect),
    Circle(Circle),
    Line(Line),
    Bezier(Bezier),
    Polygon(Polygon),
    Text(Text),
    Particle(Particle),
}

impl Primitive {
    pub fn render(&self, mut region: Region) -> Region {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use trellis_core::terminal::style::Style;

    use super::*;

    #[test]
    fn rectangle_t() {
        let region = Region::new(10, 20);
        let rect = Rect::new(5, 5)
            .with_fill(Cell::new('x'))
            .with_border(RectangleBorder::Ascii(Style::default()));

        let written_to = rect.render(region);
        println!("{written_to}");
    }
}
