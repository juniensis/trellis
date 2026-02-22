use crossterm::event::{KeyCode, KeyModifiers};

use crate::{
    Event,
    cell::Cell,
    components::{Component, primitives::Rectangle, text::line::Line},
    viewport::Viewport,
};
pub mod line;

pub enum EditMode {
    Exiting,
    Normal,
    Insert,
}

pub struct TextBox {
    container: Rectangle,
    text: Vec<Line>,
    x: u16,
    y: u16,
    line: usize,
    column: usize,
    pub mode: EditMode,
    pub should_redraw: bool,
}

impl TextBox {
    pub fn new(x: u16, y: u16, w: u16, h: u16, initial: &str) -> Self {
        let mut max_len = 0;
        let text = initial
            .lines()
            .map(|x| {
                max_len = max_len.max(x.len());
                Line::from_string(x)
            })
            .collect::<Vec<_>>();
        let w = w.max(max_len as u16);
        let h = text.len() as u16;
        Self {
            container: Rectangle::new(x, y, w, h),
            text,
            x,
            y,
            line: 0,
            column: 0,
            mode: EditMode::Normal,
            should_redraw: false,
        }
    }
    pub fn on_enter(&self, viewport: &mut Viewport) {
        viewport.move_cursor(self.x, self.y);
    }
    pub fn handle_key_normal(&mut self, code: KeyCode, modifier: KeyModifiers) {
        match (code, modifier) {
            (KeyCode::Char(ch), _) => match ch {
                'h' => self.column = self.column.saturating_sub(1),
                'H' => self.column = self.column.saturating_sub(5),
                'j' => {
                    self.line = (self.line + 1).min(self.text.len());
                }
                'J' => {
                    self.line = (self.line + 1).min(self.text.len());
                    self.column = self.text[self.line].len();
                }
                'k' => self.line = self.line.saturating_sub(1),
                'K' => {
                    self.line = self.line.saturating_sub(1);
                    self.column = self.text[self.line].len();
                }
                'l' => self.column = (self.column + 1).min(self.text[self.line].len()),
                'L' => self.column = (self.column + 5).min(self.text[self.line].len()),
                _ => {}
            },
            (KeyCode::Esc, _) => self.mode = EditMode::Exiting,
            _ => {}
        }
    }
    pub fn render_text(&self) -> Vec<Cell> {
        let mut out = Vec::new();
        for (j, line) in self.text.iter().enumerate() {
            for (i, byte) in line.bytes().enumerate() {
                let (x, y) = (self.x + i as u16, self.y + j as u16);
                out.push(Cell::new(x, y, byte as char));
            }
        }
        out
    }
}

impl Component for TextBox {
    fn update(&mut self, event: crate::Event) -> Option<Event> {
        match self.mode {
            EditMode::Exiting => return Some(Event::Leave),
            EditMode::Normal => match event {
                Event::KeyPressed { code, modifiers } => self.handle_key_normal(code, modifiers),
                Event::MoveTo(x, y) => {
                    self.x = x;
                    self.y = y;
                    self.should_redraw = true;
                }
                Event::MoveBy(dx, dy) => {
                    self.x = self.x.saturating_add_signed(dx);
                    self.y = self.y.saturating_add_signed(dy);
                    self.should_redraw = true;
                }
                _ => {}
            },
            EditMode::Insert => {}
        }
        Some(Event::MoveCursor(
            self.x + self.column as u16,
            self.y + self.line as u16,
        ))
    }
    fn is_inside(&self, x: u16, y: u16) -> bool {
        self.container.is_inside(x, y)
    }
    fn draw(&self, viewport: &mut Viewport) {
        if self.should_redraw {
            self.erase(viewport);
        }
        self.container.draw(viewport);
        viewport.write_cells(&self.render_text());
    }
    fn erase(&self, viewport: &mut Viewport) {
        self.container.erase(viewport);
        viewport.clear_cells(&self.render_text());
    }
}

#[cfg(test)]
mod tests {
    use crate::Trellis;

    use super::*;

    #[test]
    fn textbox() {
        let mut trellis = Trellis::new().with_component(TextBox::new(5, 5, 10, 10, "Hello"));
        trellis.run();
    }
}
