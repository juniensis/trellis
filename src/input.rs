//! Accepts events and converts them into top-level commands.

use std::{collections::VecDeque, time::Instant};

use trellis_terminal::event::{Event, KeyCode, Modifiers};

use crate::commands::Command;

#[derive(Debug, Clone, Copy)]
pub enum State {
    Normal,
    Insert,
}

#[derive(Debug, Clone)]
pub struct KeyStroke {
    code: KeyCode,
    modifiers: Modifiers,
    time: Instant,
}

pub struct InputStateMachine {
    state: State,
    buffer: VecDeque<KeyStroke>,
}

impl InputStateMachine {
    pub fn new() -> Self {
        Self {
            state: State::Normal,
            buffer: VecDeque::new(),
        }
    }
    pub fn handle(&mut self, event: &Event) -> Option<Command> {
        let keystroke = if let Event::Key { code, modifiers } = event {
            KeyStroke {
                code: *code,
                modifiers: *modifiers,
                time: Instant::now(),
            }
        } else {
            return None;
        };

        self.buffer.push_front(keystroke.clone());
        while self.buffer.len() > 16 {
            self.buffer.pop_back();
        }

        match self.state {
            State::Normal => self.handle_normal(keystroke),
            State::Insert => None,
            _ => None,
        }
    }
    fn handle_normal(&self, stroke: KeyStroke) -> Option<Command> {
        match stroke.code {
            KeyCode::Char('h') => Some(Command::NormalCursorLeft(1)),
            KeyCode::Char('j') => Some(Command::NormalCursorDown(1)),
            KeyCode::Char('k') => Some(Command::NormalCursorUp(1)),
            KeyCode::Char('l') => Some(Command::NormalCursorRight(1)),
            KeyCode::Char('H') => Some(Command::NormalCursorLeft(10)),
            KeyCode::Char('J') => Some(Command::NormalCursorDown(10)),
            KeyCode::Char('K') => Some(Command::NormalCursorUp(10)),
            KeyCode::Char('L') => Some(Command::NormalCursorRight(10)),
            KeyCode::Char('i') => Some(Command::NormalInsert),
            KeyCode::Enter => Some(Command::NormalEnter),
            _ => None,
        }
    }
}

impl Default for InputStateMachine {
    fn default() -> Self {
        Self::new()
    }
}
