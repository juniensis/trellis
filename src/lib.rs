#![allow(dead_code, unused_imports)]

pub mod nodes;

use std::{
    io::{self, Stdout, stdout},
    process::exit,
    time::Duration,
};

use crossterm::{
    cursor::MoveTo,
    event::{
        self, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers, poll, read,
    },
    execute,
    terminal::{
        self, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
    },
};

use crate::{nodes::Node, screen::Screen};
pub mod screen;

pub enum Mode {
    Normal,
    Insert,
    InNode(usize),
}

pub struct Trellis {
    out: Stdout,
    screen: Screen,
    x: u16,
    y: u16,
    mode: Mode,
    nodes: Vec<Box<dyn Node>>,
}

impl Trellis {
    pub fn new() -> Self {
        enable_raw_mode().unwrap();
        let mut out = stdout();
        execute!(out, EnterAlternateScreen).unwrap();
        let size = terminal::window_size().unwrap();
        Self {
            out,
            screen: Screen::new(),
            x: size.columns / 2,
            y: size.rows / 2,
            mode: Mode::Normal,
            nodes: Vec::new(),
        }
    }
    pub fn update(&mut self) -> io::Result<()> {
        if let Ok(true) = poll(Duration::from_secs_f64(1.0 / 240.0)) {
            match event::read()? {
                Event::Key(key) => self.handle_key(key),
                Event::Resize(_, _) => {
                    self.screen.update();
                    self.screen.clear();
                    self.screen.flush();
                }
                _ => {}
            }
        }

        self.screen.display();
        execute!(self.out, MoveTo(self.x, self.y))?;
        Ok(())
    }
    pub fn add_node(&mut self, node: Box<dyn Node>) {
        self.nodes.push(node)
    }
    pub fn handle_key(&mut self, event: KeyEvent) {
        match self.mode {
            Mode::Normal => self.normal_mode(event),
            Mode::Insert => self.insert_mode(event),
            Mode::InNode(x) => {
                let (newx, newy, ret) = self.nodes[x].update(event);
                self.x = newx;
                self.y = newy;
                if let Some(r) = ret {
                    self.mode = r;
                }
                self.nodes[x].draw(&mut self.screen);
            }
        }
    }
    pub fn normal_mode(&mut self, event: KeyEvent) {
        match event {
            KeyEvent {
                code: KeyCode::Char(ch),
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            } => match ch {
                'h' => self.x = self.x.saturating_sub(1),
                'j' => self.y = self.screen.normalize_y(self.y + 1),
                'k' => self.y = self.y.saturating_sub(1),
                'l' => self.x = self.screen.normalize_x(self.x + 1),
                'i' => self.mode = Mode::Insert,
                _ => {}
            },
            KeyEvent {
                code: KeyCode::Char(ch),
                modifiers: KeyModifiers::SHIFT,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            } => match ch {
                'H' => self.x = self.x.saturating_sub(5),
                'J' => self.y = self.screen.normalize_y(self.y + 5),
                'K' => self.y = self.y.saturating_sub(5),
                'L' => self.x = self.screen.normalize_x(self.x + 5),
                _ => {}
            },
            KeyEvent {
                code: KeyCode::Esc,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            } => {
                self.drop();
                exit(0);
            }

            _ => {}
        }
    }
    pub fn insert_mode(&mut self, event: KeyEvent) {
        match event {
            KeyEvent {
                code: KeyCode::Char(ch),
                modifiers: _,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            } => {
                if ch == 'j'
                    && let Ok(true) = poll(Duration::from_millis(100))
                    && let Ok(Event::Key(KeyEvent {
                        code: KeyCode::Char('k'),
                        modifiers: _,
                        kind: _,
                        state: _,
                    })) = read()
                {
                    self.mode = Mode::Normal;
                    return;
                }
                self.screen
                    .write_to_cell((self.x as usize, self.y as usize), ch as u8);
                self.x = self.screen.normalize_x(self.x + 1);
            }
            KeyEvent {
                code,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            } => match code {
                KeyCode::Esc => self.mode = Mode::Normal,
                KeyCode::Backspace => {
                    self.x = self.x.saturating_sub(1);
                    self.screen
                        .write_to_cell((self.x as usize, self.y as usize), b' ');
                }
                _ => {}
            },

            _ => {}
        }
    }
    fn drop(&mut self) {
        disable_raw_mode().unwrap();
        execute!(self.out, LeaveAlternateScreen).unwrap();
    }
}

impl Default for Trellis {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Trellis {
    fn drop(&mut self) {
        disable_raw_mode().unwrap();
        execute!(self.out, LeaveAlternateScreen).unwrap();
    }
}

#[cfg(test)]
mod lib_t {}
