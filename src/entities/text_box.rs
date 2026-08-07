use trellis_core::terminal::Cell;
use trellis_graphics::{
    primitives::{Primitive, RenderCtx, Renderable},
    shapes::Rectangle,
};

use crate::id::Id;

#[derive(Debug, Default)]
pub struct Text {
    lines: Vec<String>,
}

impl Text {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn insert(&mut self, line: usize, byte_column: usize, ch: char) {
        while self.lines.len() <= line {
            self.lines.push(String::new());
        }

        if let Some(l) = self.lines.get_mut(line) {
            l.insert(byte_column, ch);
        }
    }
    pub fn insert_str(&mut self, line: usize, byte_column: usize, str: &str) {
        while self.lines.len() <= line {
            self.lines.push(String::new());
        }

        if let Some(l) = self.lines.get_mut(line) {
            l.insert_str(byte_column, str);
        }
    }
    pub fn delete(&mut self, line: usize, byte_column: usize) {
        if let Some(l) = self.lines.get_mut(line) {
            l.remove(byte_column);
        }
    }
    pub fn split_line(&mut self, line: usize, byte_column: usize) {
        if let Some(l) = self.lines.get_mut(line) {
            let rest = l.split_off(byte_column);
            self.insert_str(line + 1, 0, &rest);
        }
    }
}

#[derive(Debug, Default)]
pub struct TextCursor {
    line: usize,
    column: usize,
}

impl TextCursor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn up(&mut self) {
        self.line = self.line.saturating_sub(1);
    }
    pub fn down(&mut self) {
        self.line += 1;
    }
    pub fn left(&mut self) {
        self.column = self.column.saturating_sub(1);
    }
    pub fn right(&mut self) {
        self.column += 1;
    }
}

#[derive(Debug, Default)]
pub struct TextBox {
    pub content: Text,
    pub cursor: TextCursor,
    pub z_order: u32,
}

impl TextBox {
    pub fn width(&self) -> u32 {
        self.content
            .lines
            .iter()
            .map(|x| x.len())
            .max()
            .unwrap_or(0) as u32
            + 1
    }
    pub fn height(&self) -> u32 {
        self.content.lines.len() as u32 + 1
    }
}

impl Renderable for TextBox {
    fn render(&self, _ctx: RenderCtx) -> Primitive {
        let mut rect = Rectangle::new(self.width(), self.height(), self.z_order).render(_ctx);

        for (y, line) in self.content.lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let y = y + 1;
                let x = x + 1;
                rect.set_cell((x, y), Cell::new(ch))
            }
        }

        rect
    }
}

#[cfg(test)]
mod tests {
    use trellis_graphics::shell::Shell;
    use trellis_terminal::event::{Event, KeyCode};

    use super::*;

    #[test]
    fn draw_textbox() {
        let mut shell = Shell::new_crossterm();
        let mut text_box = TextBox::default();
        text_box.content.insert_str(0, 0, "TextBox Testing");
        text_box.content.insert_str(1, 0, "Another Line");

        'running: while let Some(tick) = shell.tick() {
            for event in tick.events() {
                if let &Event::Key {
                    code: KeyCode::Escape,
                    modifiers: _,
                } = event
                {
                    break 'running;
                }
            }

            shell.start_frame().with_draw((5, 5), &text_box).submit();
        }
    }
}
