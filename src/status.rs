use std::collections::VecDeque;

use trellis_core::terminal::Cell;
use trellis_graphics::primitives::{Primitive, Renderable};
use trellis_terminal::event::{Event, KeyCode};

use crate::{State, input::Keystroke};

pub struct StatusBar {
    state: State,
    input_buffer: VecDeque<Keystroke>,
    max_width: u32,
}

impl StatusBar {
    pub fn new() -> Self {
        Self {
            state: State::Normal,
            input_buffer: VecDeque::new(),
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
}

impl Default for StatusBar {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderable for StatusBar {
    fn render(&self) -> Primitive {
        let mut region = Primitive::new_region(255);

        match self.state {
            State::Normal => {
                region.set_cell((1, 0), Cell::new('N'));
                region.set_cell((2, 0), Cell::new('O'));
                region.set_cell((3, 0), Cell::new('R'));
                region.set_cell((4, 0), Cell::new('M'));
                region.set_cell((5, 0), Cell::new('A'));
                region.set_cell((6, 0), Cell::new('L'));
            }
            State::Insert => {
                region.set_cell((1, 0), Cell::new('I'));
                region.set_cell((2, 0), Cell::new('N'));
                region.set_cell((3, 0), Cell::new('S'));
                region.set_cell((4, 0), Cell::new('E'));
                region.set_cell((5, 0), Cell::new('R'));
                region.set_cell((6, 0), Cell::new('T'));
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

        region
    }
}
