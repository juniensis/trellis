use std::hash::Hash;

use crate::{buffer::Buffer, cell::Cell, event::UiEvent, rand::rand_u128, style::Style};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct WidgetId(u128);
impl Default for WidgetId {
    fn default() -> Self {
        Self(rand_u128())
    }
}

pub trait Widget<Message> {
    fn collides(&self, x: usize, y: usize) -> bool;
    fn update(&mut self, message: Message) -> Option<Message>;
    fn draw(&self, buffer: &mut Buffer);
    fn identify(&self) -> WidgetId;
}

#[derive(Debug, Clone, Default, Hash)]
pub struct Label {
    id: WidgetId,
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
    pub fn width(&self) -> usize {
        self.text.lines().map(|x| x.len()).max().unwrap_or(0)
    }
    pub fn height(&self) -> usize {
        self.text.lines().count()
    }
    pub fn x(&self) -> usize {
        self.position.0
    }
    pub fn y(&self) -> usize {
        self.position.1
    }
}

impl Widget<UiEvent> for Label {
    fn collides(&self, x: usize, y: usize) -> bool {
        (self.x()..self.x() + self.width()).contains(&x)
            && (self.y()..self.y() + self.height()).contains(&y)
    }
    fn update(&mut self, message: UiEvent) -> Option<UiEvent> {
        match message {
            UiEvent::MoveTo(x, y) => {
                self.position = (x, y);
            }
            UiEvent::MoveBy(dx, dy) => {
                self.position = (
                    self.position.0.saturating_add_signed(dx),
                    self.position.1.saturating_add_signed(dy),
                );
            }
            UiEvent::String(txt) => self.text = txt,
            UiEvent::SetStyle(sty) => self.set_style(sty),
            _ => {}
        }
        None
    }
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
    fn identify(&self) -> WidgetId {
        self.id
    }
}
