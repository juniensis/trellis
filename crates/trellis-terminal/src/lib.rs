use crate::{
    error::TerminalResult,
    event::{EventSender, KeyCode, TerminalEvent},
};
use crossterm::{
    cursor::{self, Hide, MoveTo, Show},
    execute, queue,
    terminal::{
        EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode, window_size,
    },
};
use std::{
    io::{self, BufWriter, Stdout, Write},
    thread,
};

pub mod error;
pub mod event;

pub struct Terminal {
    writer: BufWriter<Stdout>,
}

impl Terminal {
    pub fn new() -> TerminalResult<Self> {
        let mut term = Self {
            writer: BufWriter::new(io::stdout()),
        };
        enable_raw_mode()?;
        execute!(term.writer, EnterAlternateScreen, Hide)?;
        Ok(term)
    }
    pub fn link(&self, sender: EventSender<TerminalEvent>) {
        thread::spawn(|| read_events(sender));
    }
    pub fn size(&self) -> TerminalResult<(usize, usize)> {
        let sz = window_size()?;
        Ok((sz.columns as usize, sz.rows as usize))
    }
    pub fn move_cursor(&mut self, x: usize, y: usize) -> TerminalResult<()> {
        queue!(self.writer, MoveTo(x as u16, y as u16))?;
        Ok(())
    }
    pub fn write(&mut self, bytes: &[u8]) -> TerminalResult<()> {
        self.writer.write_all(bytes)?;
        Ok(())
    }
    pub fn reset_sgr(&mut self) -> TerminalResult<()> {
        self.writer.write_all(b"\x1b[0m")?;
        Ok(())
    }
    pub fn show_cursor(&mut self) -> TerminalResult<()> {
        queue!(self.writer, Show)?;
        Ok(())
    }
    pub fn hide_cursor(&mut self) -> TerminalResult<()> {
        queue!(self.writer, Hide)?;
        Ok(())
    }
    pub fn save_cursor(&mut self) -> TerminalResult<()> {
        queue!(self.writer, cursor::SavePosition)?;
        Ok(())
    }
    pub fn restore_cursor(&mut self) -> TerminalResult<()> {
        queue!(self.writer, cursor::RestorePosition)?;
        Ok(())
    }
    pub fn flush(&mut self) -> TerminalResult<()> {
        self.writer.flush()?;
        Ok(())
    }
    pub fn write_str<S: AsRef<str>>(&mut self, str: S) -> TerminalResult<()> {
        write!(self.writer, "{}", str.as_ref())?;
        Ok(())
    }
    pub fn write_char(&mut self, ch: char) -> TerminalResult<()> {
        write!(self.writer, "{ch}")?;
        Ok(())
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        let _ = execute!(self.writer, LeaveAlternateScreen, Show);
        let _ = disable_raw_mode();
        let _ = self.writer.flush();
    }
}

fn read_events(sender: EventSender<TerminalEvent>) {
    loop {
        if let Some(e) = crossterm::event::read()
            .ok()
            .and_then(TerminalEvent::from_crossterm_event)
        {
            match e {
                TerminalEvent::Key {
                    code: KeyCode::Char('c'),
                    modifiers,
                } if modifiers.is_ctrl() => {
                    sender.send(TerminalEvent::Quit);
                    return;
                }
                other => sender.send(other),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::event::EventQueue;

    use super::*;
    #[test]
    fn read_events() {
        let queue = EventQueue::new();
        let terminal = Terminal::new().unwrap();
        terminal.link(queue.sender());

        loop {
            let e = queue.recv();
            println!("{e:?}");
        }
    }
}
