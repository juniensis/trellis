use std::{
    hash::Hash,
    io::{self, Write},
};

use crossterm::{
    cursor::{MoveTo, RestorePosition, SavePosition},
    execute,
    style::{Color, Colored, Colors, Print, ResetColor, SetColors},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Nuclei {
    ch: char,
    colors: Colors,
}

impl Nuclei {
    pub fn new(ch: char) -> Self {
        Self {
            ch,
            colors: Colors::new(Color::Reset, Color::Reset),
        }
    }
    pub fn with_fg(self, fg: Color) -> Self {
        let mut ret = self;
        ret.colors.foreground = Some(fg);
        ret
    }
    pub fn with_bg(self, bg: Color) -> Self {
        let mut ret = self;
        ret.colors.background = Some(bg);
        ret
    }
    pub fn with_char(self, ch: char) -> Self {
        let mut ret = self;
        ret.ch = ch;
        ret
    }
    pub fn set_char(&mut self, ch: char) {
        self.ch = ch;
    }
    pub fn set_fg(&mut self, color: Color) {
        self.colors.foreground = Some(color);
    }
    pub fn set_bg(&mut self, color: Color) {
        self.colors.background = Some(color);
    }
    pub fn with_colors(self, colors: Colors) -> Self {
        let mut ret = self;
        ret.colors = colors;
        ret
    }
    pub fn into_cell(self, x: u16, y: u16) -> Cell {
        Cell { nuclei: self, x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    nuclei: Nuclei,
    pub x: u16,
    pub y: u16,
}

impl Cell {
    pub fn new(x: u16, y: u16, ch: char) -> Self {
        Self {
            nuclei: Nuclei::new(ch),
            x,
            y,
        }
    }
    pub fn display<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        execute!(
            writer,
            SavePosition,
            MoveTo(self.x, self.y),
            SetColors(self.nuclei.colors),
            Print(self.nuclei.ch),
            ResetColor,
            RestorePosition
        )
    }
    pub fn with_fg(self, fg: Color) -> Self {
        let mut ret = self;
        ret.nuclei = ret.nuclei.with_fg(fg);
        ret
    }
    pub fn with_bg(self, bg: Color) -> Self {
        let mut ret = self;
        ret.nuclei = ret.nuclei.with_bg(bg);
        ret
    }
    pub fn with_colors(self, colors: Colors) -> Self {
        let mut ret = self;
        ret.nuclei.colors = colors;
        ret
    }
    pub fn clear<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        execute!(
            writer,
            SavePosition,
            MoveTo(self.x, self.y),
            ResetColor,
            Print(' '),
            RestorePosition
        )
    }
    pub fn x(&self) -> usize {
        self.x as usize
    }
    pub fn y(&self) -> usize {
        self.y as usize
    }
    pub fn redraw<W: Write>(&mut self, writer: &mut W, new: Cell) -> io::Result<()> {
        self.clear(writer)?;
        *self = new;
        self.display(writer)
    }
}

impl Hash for Cell {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

#[cfg(test)]
mod cell_t {
    use std::{io::stdout, thread::sleep, time::Duration};

    use super::*;
    #[test]
    fn display_t() {
        let cell = Cell::new(5, 5, '@').with_fg(Color::Red);
        cell.display(&mut stdout()).unwrap();
        cell.clear(&mut stdout()).unwrap();
    }
}
