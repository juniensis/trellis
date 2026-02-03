use std::fmt::Display;

use crossterm::style::Colors;

use crate::{
    cell::{Cell, Nuclei},
    components::Component,
};

#[derive(Debug, Clone, Copy)]
pub enum RectangleStyle {
    Fill(Nuclei),
    CustomBorder {
        corners: [Nuclei; 4],
        horizontal: Nuclei,
        vertical: Nuclei,
    },
    PlainBorder,
    ColoredBorder(Colors),
}

#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    style: RectangleStyle,
}

impl Rectangle {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self {
            x,
            y,
            width,
            height,
            style: RectangleStyle::Fill(Nuclei::new('x')),
        }
    }
    pub fn with_style(self, style: RectangleStyle) -> Self {
        let mut ret = self;
        ret.style = style;
        ret
    }
    pub fn move_to(&mut self, x: u16, y: u16) {
        (self.x, self.y) = (x, y);
    }
    pub fn move_by(&mut self, dx: i16, dy: i16) {
        self.x = self.x.saturating_add_signed(dx);
        self.y = self.y.saturating_add_signed(dy);
    }
    pub fn resize(&mut self, nw: u16, nh: u16) {
        (self.width, self.height) = (nw, nh);
    }
    pub fn render_cells(&self) -> Vec<Cell> {
        match self.style {
            RectangleStyle::Fill(n) => {
                let mut cells = Vec::with_capacity((self.width * self.height) as usize);
                for j in self.y..self.y + self.height {
                    for i in self.x..self.x + self.width {
                        cells.push(n.into_cell(i, j));
                    }
                }
                cells
            }
            RectangleStyle::CustomBorder {
                corners,
                horizontal,
                vertical,
            } => {
                let mut cells = vec![
                    corners[0].into_cell(self.x, self.y),
                    corners[1].into_cell(self.x + self.width, self.y),
                    corners[2].into_cell(self.x + self.width, self.y + self.height),
                    corners[3].into_cell(self.x, self.y + self.height),
                ];

                for i in (self.x + 1)..self.x + self.width {
                    cells.push(horizontal.into_cell(i, self.y));
                    cells.push(horizontal.into_cell(i, self.y + self.height));
                }

                for j in (self.y + 1)..self.y + self.height {
                    cells.push(vertical.into_cell(self.x, j));
                    cells.push(vertical.into_cell(self.x + self.width, j));
                }
                cells
            }
            RectangleStyle::PlainBorder => {
                let corner = Nuclei::new('+');
                let vert = Nuclei::new('|');
                let hori = Nuclei::new('-');
                let mut cells = vec![
                    corner.into_cell(self.x, self.y),
                    corner.into_cell(self.x + self.width, self.y),
                    corner.into_cell(self.x + self.width, self.y + self.height),
                    corner.into_cell(self.x, self.y + self.height),
                ];

                for i in (self.x + 1)..self.x + self.width {
                    cells.push(hori.into_cell(i, self.y));
                    cells.push(hori.into_cell(i, self.y + self.height));
                }

                for j in (self.y + 1)..self.y + self.height {
                    cells.push(vert.into_cell(self.x, j));
                    cells.push(vert.into_cell(self.x + self.width, j));
                }
                cells
            }
            RectangleStyle::ColoredBorder(c) => {
                let corner = Nuclei::new('+').with_colors(c);
                let vert = Nuclei::new('|').with_colors(c);
                let hori = Nuclei::new('-').with_colors(c);
                let mut cells = vec![
                    corner.into_cell(self.x, self.y),
                    corner.into_cell(self.x + self.width, self.y),
                    corner.into_cell(self.x + self.width, self.y + self.height),
                    corner.into_cell(self.x, self.y + self.height),
                ];

                for i in (self.x + 1)..self.x + self.width {
                    cells.push(hori.into_cell(i, self.y));
                    cells.push(hori.into_cell(i, self.y + self.height));
                }

                for j in (self.y + 1)..self.y + self.height {
                    cells.push(vert.into_cell(self.x, j));
                    cells.push(vert.into_cell(self.x + self.width, j));
                }
                cells
            }
        }
    }
}

impl Component for Rectangle {
    fn update(&mut self, _event: crate::Event) {}
    fn draw(&self, viewport: &mut crate::viewport::Viewport) {
        viewport.write_cells(&self.render_cells());
    }
    fn erase(&self, viewport: &mut crate::viewport::Viewport) {
        viewport.clear_cells(&self.render_cells());
    }
}

#[cfg(test)]
mod primitives_t {
    use std::{thread::sleep, time::Duration};

    use crossterm::style::Color;

    use crate::viewport::Viewport;

    use super::*;

    #[ignore]
    #[test]
    fn rectangle_t() {
        let mut viewport = Viewport::new();
        let mut rect = Rectangle::new(2, 2, 5, 5).with_style(RectangleStyle::CustomBorder {
            corners: [Nuclei::new('@'); 4],
            horizontal: Nuclei::new('='),
            vertical: Nuclei::new('|'),
        });
        for _ in 0..60 {
            rect.draw(&mut viewport);
            sleep(Duration::from_millis(50));
            rect.erase(&mut viewport);
            rect.move_by(1, 1);
        }
    }
}
