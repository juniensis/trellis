use std::{
    collections::HashSet,
    io::{Stdout, stdout},
};

use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, window_size},
};

use crate::cell::Cell;

pub struct Viewport {
    out: Stdout,
    width: u16,
    height: u16,
    x: u16,
    y: u16,
}

impl Viewport {
    pub fn new() -> Self {
        let sz = window_size().unwrap();
        let mut out = stdout();
        execute!(out, EnterAlternateScreen).unwrap();
        Self {
            out,
            width: sz.columns,
            height: sz.rows,
            x: 0,
            y: 0,
        }
    }
    pub fn write_cells(&mut self, c: &[Cell]) {
        for cell in c {
            cell.display(&mut self.out).unwrap();
        }
    }
    pub fn clear_cells(&mut self, c: &[Cell]) {
        for cell in c {
            cell.clear(&mut self.out).unwrap();
        }
    }
    pub fn write_cell(&mut self, c: Cell) {
        c.display(&mut self.out).unwrap()
    }
    pub fn clear_cell(&mut self, c: Cell) {
        c.clear(&mut self.out).unwrap();
    }
    pub fn redraw_cell(&mut self, old: Cell, new: Cell) {
        self.clear_cell(old);
        self.write_cell(new);
    }
    pub fn resize(&mut self) {
        let sz = window_size().unwrap();
        self.width = sz.columns;
        self.height = sz.rows;
    }
    pub fn move_cursor(&mut self, x: u16, y: u16) {
        self.x = x;
        self.y = y;
        execute!(self.out, MoveTo(x, y)).unwrap();
    }
    pub fn displace_cursor(&mut self, dx: i16, dy: i16) {
        self.x = self.x.saturating_add_signed(dx);
        self.y = self.y.saturating_add_signed(dy);
        execute!(self.out, MoveTo(self.x, self.y)).unwrap();
    }
    pub fn pos(&self) -> (u16, u16) {
        (self.x, self.y)
    }
    pub fn dimensions(&self) -> (u16, u16) {
        (self.width, self.height)
    }
    pub fn leave(&mut self) {
        execute!(self.out, LeaveAlternateScreen).unwrap();
    }
}

impl Default for Viewport {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Viewport {
    fn drop(&mut self) {
        self.leave();
    }
}
