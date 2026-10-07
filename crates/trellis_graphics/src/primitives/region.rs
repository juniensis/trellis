use std::fmt::Display;

use trellis_core::{
    primitives::Pos,
    terminal::{buffer::Buffer, cell::Cell},
};

use crate::primitives::{Primitive, Renderable, Scatter};

/// A dynamically resizing rectangular cell region.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Region {
    cells: Vec<Vec<Cell>>,
    width: u32,
    height: u32,
    z_order: u32,
}

impl Region {
    #[inline]
    pub fn build(cells: Vec<&[Cell]>, z_order: u32) -> Self {
        let (width, height) = (cells[0].len() as u32, cells.len() as u32);
        Self {
            cells: cells.iter().map(|x| x.to_vec()).collect(),
            width,
            height,
            z_order,
        }
    }
    #[inline]
    pub fn new(z_order: u32) -> Self {
        Self {
            cells: vec![vec![Cell::null(); 4]; 4],
            width: 4,
            height: 4,
            z_order,
        }
    }
    #[inline]
    pub fn with_capacity(width: u32, height: u32, z_order: u32) -> Self {
        Self {
            cells: vec![vec![Cell::null(); width as usize]; height as usize],
            width,
            height,
            z_order,
        }
    }
    #[inline]
    pub fn set_cell(&mut self, x: u32, y: u32, cell: Cell) {
        while y as usize >= self.cells.len() {
            self.cells.push(vec![Cell::null(); self.width as usize]);
        }

        if x >= self.width {
            let new_width = x + 1;
            for row in self.cells.iter_mut() {
                row.resize(new_width as usize, Cell::null());
            }
            self.width = new_width;
        }

        self.cells[y as usize][x as usize] = cell;
    }
    #[inline]
    pub fn get_cell(&self, x: u32, y: u32) -> Option<&Cell> {
        self.cells
            .get(y as usize)
            .and_then(|inner| inner.get(x as usize))
    }
    #[inline]
    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }
    #[inline]
    pub fn composite(&self, pos: impl Into<Pos>, dst: &mut Buffer) {
        let pos = pos.into();
        let x_offset = if pos.x < 0 { pos.x.abs() } else { 0 };
        let y_offset = if pos.y < 0 { pos.y.abs() } else { 0 };

        for (ldx, line) in self.cells.iter().skip(y_offset as usize).enumerate() {
            let y = pos.y.max(0) as usize + ldx;
            if y as u32 >= dst.height() {
                continue;
            }
            for (idx, cell) in line.iter().skip(x_offset as usize).enumerate() {
                let x = pos.x.max(0) as usize + idx;
                if x as u32 >= dst.width() {
                    continue;
                }
                if cell.is_null() {
                    continue;
                }
                dst.set_cell(x as u32, y as u32, *cell);
            }
        }
    }
    #[inline]
    pub fn positioned_iter(&self) -> impl Iterator<Item = (Pos, Cell)> {
        (0..self.height).flat_map(move |y| {
            (0..self.width).flat_map(move |x| self.get_cell(x, y).map(|&c| ((x, y).into(), c)))
        })
    }
    #[inline]
    pub fn merge(&mut self, other: &Self) {
        for (p, c) in other.positioned_iter() {
            if self.z_order < other.z_order {
                if self
                    .get_cell(p.x as u32, p.y as u32)
                    .is_none_or(|cx| cx != &c)
                {
                    self.set_cell(p.x as u32, p.y as u32, c);
                }
            } else {
                if self.get_cell(p.x as u32, p.y as u32).is_none() {
                    self.set_cell(p.x as u32, p.y as u32, c);
                }
            }
        }
    }
    pub fn z_order(&self) -> u32 {
        self.z_order
    }
    pub fn set_z_order(&mut self, z_order: u32) {
        self.z_order = z_order;
    }
    #[inline]
    pub fn merge_scatter(&mut self, other: &Scatter) {
        for (p, c) in other.iter() {
            if self
                .get_cell(p.x as u32, p.y as u32)
                .is_none_or(|cx| cx != &c)
            {
                self.set_cell(p.x as u32, p.y as u32, c);
            }
        }
    }
    pub fn into_primitive(self) -> Primitive {
        Primitive::Region(self)
    }
}

impl Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in &self.cells {
            for c in r {
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Default for Region {
    fn default() -> Self {
        Self::new(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composite_onto_buffer() {
        let mut region = Region::new(0);
        region.set_cell(1, 1, Cell::new('x'));
        region.set_cell(2, 1, Cell::new('x'));
        region.set_cell(3, 1, Cell::new('x'));
        region.set_cell(4, 1, Cell::new('x'));
        region.set_cell(1, 2, Cell::new('x'));
        region.set_cell(2, 2, Cell::new('x'));
        region.set_cell(3, 2, Cell::new('x'));
        region.set_cell(4, 2, Cell::new('x'));

        print!("{region}");
        println!("{}", region.cells.len());

        let mut buffer = Buffer::new(10, 10);
        region.composite((0, 0), &mut buffer);
        println!("{buffer}");
    }
    #[test]
    fn clipping_blit() {
        let mut region = Region::new(0);
        region.set_cell(0, 0, Cell::new('x'));
        region.set_cell(1, 0, Cell::new('x'));
        region.set_cell(2, 0, Cell::new('x'));

        let mut buffer = Buffer::new(3, 3);
        region.composite((-2, 0), &mut buffer);
        println!("{buffer}");
    }
}
