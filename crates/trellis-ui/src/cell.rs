use std::fmt::Display;

use crate::style::{Color, Modes, Style};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cell {
    ch: char,
    style: Style,
}

impl Cell {
    #[inline]
    pub fn new(ch: char) -> Self {
        Self {
            ch,
            style: Style::new(),
        }
    }
    #[inline]
    pub fn build(ch: char, fg: Color, bg: Color, modes: Modes) -> Self {
        Self {
            ch,
            style: Style::new().with_fg(fg).with_bg(bg).with_mode(modes),
        }
    }
    #[inline]
    pub fn with_fg(mut self, fg: Color) -> Self {
        self.style.set_fg(fg);
        self
    }
    #[inline]
    pub fn with_bg(mut self, bg: Color) -> Self {
        self.style.set_bg(bg);
        self
    }
    #[inline]
    pub fn with_modes(mut self, mode: Modes) -> Self {
        self.style = self.style.with_mode(mode);
        self
    }
    #[inline]
    pub fn set_char(&mut self, ch: char) {
        self.ch = ch;
    }
    #[inline]
    pub fn set_modes(&mut self, modes: Modes) {
        self.style = self.style.with_mode(modes);
    }
    #[inline]
    pub fn add_modes(&mut self, modes: Modes) {
        self.style.mode_mut().0 |= modes.as_u8();
    }
    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
    pub fn set_style(&mut self, style: Style) {
        self.style = style;
    }
    pub fn char(&self) -> char {
        self.ch
    }
    pub fn style(&self) -> Style {
        self.style
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}\x1b[0m", self.style.as_ansi(), self.ch)
    }
}
