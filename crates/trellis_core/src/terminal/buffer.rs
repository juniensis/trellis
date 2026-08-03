use std::fmt;

use crate::terminal::cell::Cell;

/// Top-left is (0, 0), bottom right is ('width', 'height').
#[derive(Debug, Clone)]
pub struct Buffer {
    width: u32,
    height: u32,
    inner: Vec<Cell>,
}

impl Buffer {
    #[inline]
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            inner: vec![Cell::new(' '); (width * height) as usize],
        }
    }
    #[inline]
    pub fn with_fill(width: u32, height: u32, cell: Cell) -> Self {
        Self {
            width,
            height,
            inner: vec![cell; (width * height) as usize],
        }
    }
    #[inline]
    pub fn get(&self, x: u32, y: u32) -> Option<&Cell> {
        self.inner.get((y * self.width + x) as usize)
    }
    /// # Safety
    #[inline]
    pub unsafe fn get_unchecked(&self, x: u32, y: u32) -> &Cell {
        &self.inner[(y * self.width + x) as usize]
    }
    #[inline]
    pub fn get_mut(&mut self, x: u32, y: u32) -> Option<&mut Cell> {
        self.inner.get_mut((y * self.width + x) as usize)
    }
    /// # Safety
    #[inline]
    pub unsafe fn get_mut_unchecked(&mut self, x: u32, y: u32) -> &mut Cell {
        &mut self.inner[(y * self.width + x) as usize]
    }
    #[inline]
    pub fn set_cell(&mut self, x: u32, y: u32, cell: Cell) {
        if let Some(c) = self.get_mut(x, y) {
            *c = cell;
        }
    }
    #[inline]
    pub fn set_sequence(&mut self, x: u32, y: u32, cells: &[Cell]) {
        let idx = (y * self.width + x) as usize;
        if let Some(slice) = self.inner.get_mut(idx..idx + cells.len()) {
            slice.copy_from_slice(cells);
        }
    }
    #[inline]
    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }
    #[inline]
    pub fn width(&self) -> u32 {
        self.dimensions().0
    }
    #[inline]
    pub fn height(&self) -> u32 {
        self.dimensions().1
    }
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &Cell> {
        self.inner.iter()
    }
    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Cell> {
        self.inner.iter_mut()
    }
    #[inline]
    pub fn positioned_iter(&self) -> impl Iterator<Item = ((u32, u32), &Cell)> {
        self.iter()
            .enumerate()
            .map(|(idx, cell)| ((idx as u32 % self.width, idx as u32 / self.width), cell))
    }
    #[inline]
    pub fn positioned_iter_mut(&mut self) -> impl Iterator<Item = ((u32, u32), &mut Cell)> {
        let width = self.width;
        self.iter_mut()
            .enumerate()
            .map(move |(idx, cell)| ((idx as u32 % width, idx as u32 / width), cell))
    }
    #[inline]
    pub fn string(&self) -> String {
        self.inner
            .chunks(self.width as usize)
            .map(|x| {
                x.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect::<Vec<_>>()
            .join("\r\n")
    }
}

impl fmt::Display for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in self.inner.chunks(self.width as usize) {
            for cell in line {
                if cell.char() == '\0' {
                    write!(f, "{}", Cell::new(' '))?;
                } else {
                    write!(f, "{cell}")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
