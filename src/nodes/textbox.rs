use crossterm::event::{KeyCode, KeyEvent};

use crate::{Mode, nodes::Node, screen::Screen};

pub struct TextBox {
    lines: Vec<String>,
    // (line, column)
    cursor: (usize, usize),
    position: (u16, u16),
    max_width: usize,
}

impl TextBox {
    pub fn new(position: (u16, u16)) -> Self {
        Self {
            lines: vec![String::new()],
            cursor: (0, 0),
            position,
            max_width: 0,
        }
    }
    pub fn push_char(&mut self, ch: char) {
        self.lines[self.cursor.0].push(ch);
        self.cursor.1 += 1;
    }
    pub fn pop_char(&mut self) {
        self.cursor.1 = self.cursor.1.saturating_sub(1);
        self.lines[self.cursor.0].pop();
    }
    pub fn newline(&mut self) {
        self.cursor.0 += 1;
        self.cursor.1 = 0;
        if self.cursor.0 >= self.lines.len() {
            self.lines.push(String::new());
        }
    }
    pub fn update_max_width(&mut self) -> (usize, usize) {
        let ret = self.max_width;
        self.max_width = self.lines.iter().map(|x| x.len()).max().unwrap_or(ret);
        (ret, self.max_width)
    }
    pub fn pop_line(&mut self) {
        self.lines.pop();
    }
}

impl Node for TextBox {
    fn draw(&mut self, screen: &mut Screen) {
        let (x, y) = self.position;
        let (old, new) = self.update_max_width();
        for l in 0..self.lines.len() + 2 {
            if old > new {
                let dif = old - new;
                for i in 0..dif {
                    screen.write_to_cell((x as usize + 2 + new - i, (y as usize + l)), b' ');
                }
            }
            screen.write_slice(
                (x as usize, (y as usize + 1) + l),
                " ".repeat(new + 2).as_bytes(),
            );
        }
        let boxtop = format!("+{}+", "-".repeat(new));
        screen.write_slice((x as usize, y as usize), boxtop.as_bytes());
        for (idx, line) in self.lines.iter().enumerate() {
            screen.write_slice(
                (x as usize, (y as usize + 1) + idx),
                format!("|{line}").as_bytes(),
            );
            screen.write_to_cell((x as usize + new + 1, y as usize + 1 + idx), b'|');
        }
        screen.write_slice(
            (x as usize, y as usize + self.lines.len() + 1),
            boxtop.as_bytes(),
        );
        screen.update();
    }
    fn update(&mut self, event: crossterm::event::KeyEvent) -> (u16, u16, Option<Mode>) {
        let mut ret = None;
        if event.is_press() {
            match event.code {
                KeyCode::Char(ch) => self.push_char(ch),
                KeyCode::Backspace => self.pop_char(),
                KeyCode::Enter => self.newline(),
                KeyCode::Esc => ret = Some(Mode::Normal),
                _ => {}
            }
        }
        (
            self.position.0 + 1 + self.cursor.1 as u16,
            self.position.1 + 1 + self.cursor.0 as u16,
            ret,
        )
    }
}

#[cfg(test)]
mod textbox_t {
    use crate::Trellis;

    use super::*;
    #[test]
    fn create_box_t() {
        let mut trel = Trellis::new();
        let textbox = TextBox::new((5, 5));
        trel.add_node(Box::new(textbox));
        trel.mode = Mode::InNode(0);

        loop {
            trel.update().unwrap();
        }
    }
}
