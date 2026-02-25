use std::fmt;

use crate::terminal::Cell;

/// Top-left is (0, 0), bottom right is ('width', 'height').
#[derive(Debug, Clone)]
pub struct Buffer {
    width: usize,
    height: usize,
    inner: Vec<Cell>,
}

impl Buffer {
    #[inline]
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            inner: vec![Cell::new(' '); width * height],
        }
    }
    #[inline]
    pub fn with_fill(width: usize, height: usize, cell: Cell) -> Self {
        Self {
            width,
            height,
            inner: vec![cell; width * height],
        }
    }
    #[inline]
    pub fn get(&self, x: usize, y: usize) -> Option<&Cell> {
        self.inner.get(y * self.width + x)
    }
    /// # Safety
    #[inline]
    pub unsafe fn get_unchecked(&self, x: usize, y: usize) -> &Cell {
        &self.inner[y * self.width + x]
    }
    #[inline]
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        self.inner.get_mut(y * self.width + x)
    }
    /// # Safety
    #[inline]
    pub unsafe fn get_mut_unchecked(&mut self, x: usize, y: usize) -> &mut Cell {
        &mut self.inner[y * self.width + x]
    }
    #[inline]
    pub fn set_cell(&mut self, x: usize, y: usize, cell: Cell) {
        if let Some(c) = self.get_mut(x, y) {
            *c = cell;
        }
    }
    #[inline]
    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }
    #[inline]
    pub fn width(&self) -> usize {
        self.dimensions().0
    }
    #[inline]
    pub fn height(&self) -> usize {
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
    pub fn positioned_iter(&self) -> impl Iterator<Item = ((usize, usize), &Cell)> {
        self.iter()
            .enumerate()
            .map(|(idx, cell)| ((idx % self.width, idx / self.width), cell))
    }
    #[inline]
    pub fn positioned_iter_mut(&mut self) -> impl Iterator<Item = ((usize, usize), &mut Cell)> {
        let width = self.width;
        self.iter_mut()
            .enumerate()
            .map(move |(idx, cell)| ((idx % width, idx / width), cell))
    }
}

impl fmt::Display for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in self.inner.chunks(self.width) {
            for cell in line {
                write!(f, "{cell}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
