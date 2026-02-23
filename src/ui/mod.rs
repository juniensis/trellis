use std::{collections::HashMap, rc::Rc};

use trellis_terminal::{Terminal, error::TerminalResult};
use trellis_ui::{
    Buffer, Style,
    widgets::{Label, PrimitiveWidget},
};

pub struct Viewport {
    front: Buffer,
    back: Buffer,
    terminal: Terminal,
    labels: HashMap<Rc<str>, Label>,
    offset: (isize, isize),
}

impl Viewport {
    pub fn new() -> TerminalResult<Self> {
        let terminal = Terminal::new()?;
        let (width, height) = terminal.size()?;
        Ok(Self {
            front: Buffer::new(width, height),
            back: Buffer::new(width, height),
            terminal,
            labels: HashMap::new(),
            offset: (0, 0),
        })
    }
    pub fn set_offset(&mut self, x: isize, y: isize) {
        self.offset = (x, y)
    }
    pub fn get_buffer_mut(&mut self) -> &mut Buffer {
        &mut self.back
    }
    pub fn flush(&mut self) -> TerminalResult<()> {
        self.terminal.save_cursor()?;
        self.terminal.hide_cursor()?;

        let mut last_x = usize::MAX;
        let mut last_y = usize::MAX;
        let mut last_style = Style::default();

        for y in 0..self.back.height() {
            for x in 0..self.back.width() {
                let next = unsafe { self.back.get_unchecked(x, y) };
                let current = unsafe { self.front.get_unchecked(x, y) };

                if next == current {
                    continue;
                }

                if x != last_x.saturating_add(1) || y != last_y {
                    self.terminal.move_cursor(x, y)?;
                }

                if next.style() != last_style {
                    self.terminal.write_str(next.to_string())?;
                    last_style = next.style();
                } else {
                    self.terminal.write_char(next.char())?;
                }

                last_x = x;
                last_y = y;
                *self.front.get_mut(x, y).unwrap() = *next;
            }
        }

        self.terminal.reset_sgr()?;
        self.terminal.restore_cursor()?;
        self.terminal.show_cursor()?;
        self.terminal.flush()
    }
    pub fn add_label(&mut self, name: &str, text: &str, x: usize, y: usize, style: Option<Style>) {
        self.labels.insert(
            Rc::from(name),
            Label::new()
                .at_position(x, y)
                .with_text(text)
                .with_style(style.unwrap_or_default()),
        );
    }
    pub fn render(&mut self) {
        let mut back = self.back.clone();
        for label in self.labels.values() {
            label.draw(&mut back);
        }
        self.back = back;
    }
}

#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration};

    use super::*;
    #[test]
    fn labels() {
        let mut ui = Viewport::new().unwrap();
        ui.add_label("label", "Hello, world!", 0, 0, None);
        ui.render();
        ui.flush().unwrap();
        sleep(Duration::from_secs(5));
    }
}
