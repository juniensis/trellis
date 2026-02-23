use std::fmt::Display;

use crate::{
    cell::Cell,
    error::{UiError, UiResult},
};

#[derive(Clone)]
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
    pub fn write_cell(&mut self, x: usize, y: usize, cell: Cell) {
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
    pub fn write_difference(&mut self, other: &Buffer) -> UiResult<()> {
        if self.dimensions() != other.dimensions() {
            return Err(UiError::BufferDimensionMismatch(
                self.dimensions(),
                other.dimensions(),
            ));
        }
        for (target, source) in self.inner.iter_mut().zip(other.inner.iter()) {
            if target != source {
                *target = *source;
            }
        }
        Ok(())
    }
}

impl Display for Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.inner.chunks(self.width) {
            for cell in line {
                write!(f, "{cell}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
