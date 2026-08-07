use std::collections::VecDeque;

use trellis_core::terminal::Cell;
use trellis_graphics::primitives::{Primitive, RenderCtx, Renderable};
use trellis_terminal::event::{Event, KeyCode};

use crate::{State, input::Keystroke};

pub struct StatusBar {
    state: State,
    input_buffer: VecDeque<Keystroke>,
    message_buffer: VecDeque<String>,
    max_width: u32,
}

impl StatusBar {
    pub fn new() -> Self {
        Self {
            state: State::Normal,
            input_buffer: VecDeque::new(),
            message_buffer: VecDeque::new(),
            max_width: 32,
        }
    }
    pub fn set_state(&mut self, state: State) {
        self.state = state;
    }
    pub fn set_max_width(&mut self, width: u32) {
        self.max_width = width;
    }
    pub fn handle_key(&mut self, event: &Event) {
        let keystroke = if let Event::Key { code, modifiers } = event {
            Keystroke {
                code: *code,
                modifiers: *modifiers,
                since_last: None,
            }
        } else {
            return;
        };

        self.input_buffer.push_front(keystroke);
        while self.input_buffer.len() > 16 {
            self.input_buffer.pop_back();
        }
    }
    pub fn send_message<S: ToString>(&mut self, string: S) {
        let mut message = string.to_string();
        message.truncate(16);
        self.message_buffer.push_front(message);
        while self.message_buffer.len() > 2 {
            self.message_buffer.pop_back();
        }
    }
}

impl Default for StatusBar {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderable for StatusBar {
    fn render(&self, _ctx: RenderCtx) -> Primitive {
        let mut region = Primitive::new_region(255);

        match self.state {
            State::Normal => {
                region.write_str((1, 0), "NORMAL");
            }
            State::Insert => {
                region.write_str((1, 0), "INSERT");
            }
        }

        let mut x = 8;

        for input in self.input_buffer.iter() {
            match input.code {
                KeyCode::Char(ch) => {
                    region.set_cell((x, 0), Cell::new(ch));
                    x += 1;
                }
                KeyCode::Enter => {
                    region.set_cell((x, 0), Cell::new('R'));
                    region.set_cell((x + 1, 0), Cell::new('E'));
                    region.set_cell((x + 2, 0), Cell::new('T'));
                    x += 3;
                }
                _ => {}
            }
        }

        for (x, message) in self.message_buffer.iter().enumerate() {
            region.write_str(((self.max_width as i64 - 32) + (x as i64 * 16), 0), message);
        }

        region
    }
}
