use trellis_core::terminal::Cell;

use crate::primitives::{Region, RenderCtx, Renderable};

pub struct Rectangle {
    pub width: u32,
    pub height: u32,
    corners: [Cell; 4],
    vertical: Cell,
    horizontal: Cell,
    fill: Cell,
    z_order: u32,
}

impl Rectangle {
    pub fn new(width: u32, height: u32, z_order: u32) -> Self {
        Self {
            width,
            height,
            corners: [Cell::new('+'); 4],
            vertical: Cell::new('|'),
            horizontal: Cell::new('-'),
            fill: Cell::new(' '),
            z_order,
        }
    }
    pub fn set_corners(&mut self, corners: [Cell; 4]) {
        self.corners = corners;
    }
    pub fn set_fill(&mut self, fill: Cell) {
        self.fill = fill;
    }
    pub fn set_verticals(&mut self, vertical: Cell) {
        self.vertical = vertical;
    }
    pub fn set_horizontals(&mut self, horizontal: Cell) {
        self.horizontal = horizontal;
    }
}

impl Renderable for Rectangle {
    fn render(&self, _ctx: RenderCtx) -> crate::primitives::Primitive {
        let mut region = Region::new(self.z_order);
        region.set_cell(0, 0, self.corners[0]);
        region.set_cell(self.width - 1, 0, self.corners[1]);
        region.set_cell(self.width - 1, self.height - 1, self.corners[2]);
        region.set_cell(0, self.height - 1, self.corners[3]);

        for x in 1..self.width - 1 {
            region.set_cell(x, 0, self.horizontal);
            region.set_cell(x, self.height - 1, self.horizontal);
        }

        for y in 1..self.height - 1 {
            region.set_cell(0, y, self.vertical);
            region.set_cell(self.width - 1, y, self.vertical);
        }

        for inner_y in 1..self.height - 1 {
            for inner_x in 1..self.width - 1 {
                region.set_cell(inner_x, inner_y, self.fill);
            }
        }

        region.into()
    }
}
