use std::io::{BufWriter, Stdout, Write, stdout};

use crossterm::{
    event::{KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};

use crate::{
    backend::Backend,
    error::TerminalError,
    event::{Event, KeyCode, Modifiers},
};

pub struct Terminal {
    buffer: BufWriter<Stdout>,
}

impl Terminal {
    pub fn new() -> Self {
        Self {
            buffer: BufWriter::new(stdout()),
        }
    }
}

impl Default for Terminal {
    fn default() -> Self {
        Self::new()
    }
}

impl Backend for Terminal {
    fn read_event() -> Option<Event> {
        if let Ok(e) = crossterm::event::read() {
            match e {
                crossterm::event::Event::Key(KeyEvent {
                    code,
                    modifiers,
                    kind: _,
                    state: _,
                }) => {
                    let kc = match code {
                        crossterm::event::KeyCode::Esc => KeyCode::Escape,
                        crossterm::event::KeyCode::Enter => KeyCode::Enter,
                        crossterm::event::KeyCode::Backspace => KeyCode::Enter,
                        crossterm::event::KeyCode::Tab => KeyCode::Tab,
                        crossterm::event::KeyCode::Left => KeyCode::Left,
                        crossterm::event::KeyCode::Right => KeyCode::Right,
                        crossterm::event::KeyCode::Up => KeyCode::Up,
                        crossterm::event::KeyCode::Down => KeyCode::Down,
                        crossterm::event::KeyCode::Char(ch) => KeyCode::Char(ch),
                        _ => return None,
                    };
                    let mut mods = Modifiers::new();
                    for modi in modifiers.iter() {
                        match modi {
                            KeyModifiers::ALT => mods = mods.with_alt(),
                            KeyModifiers::SUPER => mods = mods.with_super(),
                            KeyModifiers::CONTROL => mods = mods.with_ctrl(),
                            _ => {}
                        }
                    }

                    Some(Event::Key {
                        code: kc,
                        modifiers: mods,
                    })
                }
                crossterm::event::Event::Resize(x, y) => {
                    Some(Event::Resized(x as usize, y as usize))
                }
                _ => None,
            }
        } else {
            None
        }
    }
    fn write_bytes(&mut self, bytes: &[u8]) -> Result<(), TerminalError> {
        let _ = self.buffer.write(bytes)?;
        Ok(())
    }
    fn size(&mut self) -> Result<(u32, u32), TerminalError> {
        let sz = crossterm::terminal::window_size()?;
        Ok((sz.columns as u32, sz.rows as u32))
    }
    fn enable_raw_mode(&mut self) -> Result<(), TerminalError> {
        enable_raw_mode()?;
        Ok(())
    }
    fn disable_raw_mode(&mut self) -> Result<(), TerminalError> {
        disable_raw_mode()?;
        Ok(())
    }
    fn flush(&mut self) -> Result<(), TerminalError> {
        self.buffer.flush()?;
        Ok(())
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        self.disable_raw_mode().unwrap();
        self.leave_alternate_screen().unwrap();
        self.flush().unwrap();
    }
}
