use std::rc::Rc;

use crate::{buffer::Buffer, cell::Cell, style::Style};

pub trait Widget<Message> {
    fn collides(&self, x: usize, y: usize) -> bool;
    fn update(&mut self, message: Message) -> Option<Message>;
    fn draw(&self, buffer: &mut Buffer);
}

pub trait PrimitiveWidget {
    fn draw(&self, buffer: &mut Buffer);
}

#[derive(Debug, Clone, Default)]
pub struct Label {
    label: Rc<str>,
    text: String,
    position: (usize, usize),
    style: Style,
}

impl Label {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
    #[inline]
    pub fn with_text<S: ToString>(mut self, text: S) -> Self {
        self.text = text.to_string();
        self
    }
    #[inline]
    pub fn at_position(mut self, x: usize, y: usize) -> Self {
        self.position = (x, y);
        self
    }
    #[inline]
    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
    #[inline]
    pub fn set_text<S: ToString>(&mut self, text: S) {
        self.text = text.to_string();
    }
    #[inline]
    pub fn set_position(&mut self, x: usize, y: usize) {
        self.position = (x, y);
    }
    #[inline]
    pub fn set_style(&mut self, style: Style) {
        self.style = style;
    }
    #[inline]
    pub fn get_style(&self) -> &Style {
        &self.style
    }
    #[inline]
    pub fn get_style_mut(&mut self) -> &mut Style {
        &mut self.style
    }
    #[inline]
    pub fn get_position(&self) -> (usize, usize) {
        self.position
    }
    #[inline]
    pub fn get_position_mut(&mut self) -> &mut (usize, usize) {
        &mut self.position
    }
    #[inline]
    pub fn get_text(&self) -> &str {
        &self.text
    }
    #[inline]
    pub fn get_text_mut(&mut self) -> &mut String {
        &mut self.text
    }
    #[inline]
    pub fn label(&self) -> &str {
        self.label.as_ref()
    }
}

impl PrimitiveWidget for Label {
    fn draw(&self, buffer: &mut Buffer) {
        let (mut x, mut y) = self.position;
        for cell in self
            .text
            .chars()
            .map(|x| Cell::new(x).with_style(self.style))
        {
            if cell.char() == '\n' {
                x = self.position.0;
                y += 1;
            } else {
                buffer.write_cell(x, y, cell);
                x += 1;
            }
        }
    }
}
