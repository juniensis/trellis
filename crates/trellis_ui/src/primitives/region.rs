use std::fmt::Display;

use trellis_core::{
    primitives::Pos,
    terminal::{buffer::Buffer, cell::Cell},
};

/// A dynamically resizing rectangular cell region.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Region {
    cells: Vec<Vec<Cell>>,
    width: u16,
    height: u16,
}

impl Region {
    #[inline]
    pub fn build(cells: Vec<&[Cell]>) -> Self {
        let (width, height) = (cells[0].len() as u16, cells.len() as u16);
        Self {
            cells: cells.iter().map(|x| x.to_vec()).collect(),
            width,
            height,
        }
    }
    #[inline]
    pub fn new() -> Self {
        Self {
            cells: vec![vec![Cell::null(); 4]; 4],
            width: 4,
            height: 4,
        }
    }
    #[inline]
    pub fn with_capacity(width: u16, height: u16) -> Self {
        Self {
            cells: vec![vec![Cell::null(); width as usize]; height as usize],
            width,
            height,
        }
    }
    #[inline]
    pub fn set_cell(&mut self, x: u16, y: u16, cell: Cell) {
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
    pub fn get_cell(&self, x: u16, y: u16) -> Option<&Cell> {
        self.cells
            .get(y as usize)
            .and_then(|inner| inner.get(x as usize))
    }
    #[inline]
    pub fn size(&self) -> (u16, u16) {
        (self.width, self.height)
    }
    #[inline]
    pub fn composite(&self, pos: impl Into<Pos>, dst: &mut Buffer) {
        let pos = pos.into();
        let mut y = pos.y as usize;
        let len = dst.width() - pos.x as usize;
        for line in self.cells.iter() {
            let a = len.min(line.len());
            dst.set_sequence(pos.x as usize, y, &line[0..a]);
            y += 1;
        }
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
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composite_onto_buffer() {
        let mut region = Region::new();
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
}
