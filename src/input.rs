use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

use trellis_terminal::event::{Event, KeyCode, Modifiers};

use crate::command::Command;

#[derive(Debug, Clone)]
pub struct Keystroke {
    pub code: KeyCode,
    pub modifiers: Modifiers,
    pub since_last: Option<Duration>,
}

#[derive(Debug, Default)]
pub struct InputStateMachine {
    keystroke_buffer: VecDeque<Keystroke>,
    last_keystroke: Option<Instant>,
}

impl InputStateMachine {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn handle_normal(&mut self, event: &Event) -> Option<Command> {
        let keystroke = if let Event::Key { code, modifiers } = event {
            Keystroke {
                code: *code,
                modifiers: *modifiers,
                since_last: self.last_keystroke.map(|x| x.elapsed()),
            }
        } else {
            return None;
        };

        self.last_keystroke = Some(Instant::now());
        self.keystroke_buffer.push_front(keystroke.clone());
        while self.keystroke_buffer.len() > 16 {
            self.keystroke_buffer.pop_back();
        }

        if keystroke.modifiers.is_none() {
            match keystroke.code {
                KeyCode::Char(ch) => match ch {
                    'h' => return Some(Command::NormalCursorLeft(1)),
                    'j' => return Some(Command::NormalCursorDown(1)),
                    'k' => return Some(Command::NormalCursorUp(1)),
                    'l' => return Some(Command::NormalCursorRight(1)),
                    'H' => return Some(Command::NormalCursorLeft(10)),
                    'J' => return Some(Command::NormalCursorDown(10)),
                    'K' => return Some(Command::NormalCursorUp(10)),
                    'L' => return Some(Command::NormalCursorRight(10)),
                    'i' => return Some(Command::NormalInsert),
                    _ => {}
                },
                KeyCode::Enter => return Some(Command::NormalEnter),
                _ => {}
            }
        }

        None
    }
}
