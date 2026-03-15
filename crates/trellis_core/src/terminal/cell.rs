use std::fmt;

use crate::terminal::style::{Color, Modes, Stylable, Style};

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
            style: Style::new().with_fg(fg).with_bg(bg).with_modes(modes),
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
        self.style = self.style.with_modes(mode);
        self
    }
    #[inline]
    pub fn set_char(&mut self, ch: char) {
        self.ch = ch;
    }
    #[inline]
    pub fn set_modes(&mut self, modes: Modes) {
        self.style = self.style.with_modes(modes);
    }
    #[inline]
    pub fn add_modes(&mut self, modes: Modes) {
        self.style.modes_mut().0 |= modes.as_u8();
    }
    #[inline]
    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
    #[inline]
    pub fn set_style(&mut self, style: Style) {
        self.style = style;
    }
    #[inline]
    pub fn char(&self) -> char {
        self.ch
    }
    #[inline]
    pub fn style(&self) -> Style {
        self.style
    }
    #[inline]
    pub fn null() -> Self {
        Self::new('\0')
    }
    #[inline]
    pub fn is_null(&self) -> bool {
        self.ch == '\0'
    }
    #[inline]
    pub fn bytes(&self) -> Vec<u8> {
        self.stylize(self.style).as_bytes().to_vec()
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.stylize(self.style))
    }
}

impl Stylable<String> for Cell {
    fn stylize(&self, style: Style) -> String {
        format!("{}{}\x1b[0m", style.as_ansi(), self.ch)
    }
}
