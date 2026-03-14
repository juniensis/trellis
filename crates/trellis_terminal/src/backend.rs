use std::{thread, time::Duration};

use crate::{
    ansi::{ALT_BUFFER_DISABLE, ALT_BUFFER_ENABLE, CURSOR_INVISIBLE, CURSOR_VISIBLE, EscapeCode},
    error::TerminalError,
    event::Event,
};

use trellis_core::collections::Queue;

pub trait Backend {
    fn read_event() -> Option<Event>
    where
        Self: Sized;
    fn write_bytes(&mut self, bytes: &[u8]) -> Result<(), TerminalError>;
    fn size(&mut self) -> Result<(usize, usize), TerminalError>;
    fn enable_raw_mode(&mut self) -> Result<(), TerminalError>;
    fn disable_raw_mode(&mut self) -> Result<(), TerminalError>;
    fn flush(&mut self) -> Result<(), TerminalError>;

    //
    // Provided.
    //

    fn write_str(&mut self, string: &str) -> Result<(), TerminalError> {
        self.write_bytes(string.as_bytes())
    }

    fn write_char(&mut self, ch: char) -> Result<(), TerminalError> {
        let mut buf = [0; 4];
        self.write_str(ch.encode_utf8(&mut buf))
    }

    /// Link a Backend's event stream to 'dst'.
    ///
    /// This spawns a thread which polls at 'polling_hz' for new events and
    /// pushes them to the queue as they come in.
    fn link(&self, dst: Queue<Event>, polling_hz: u16)
    where
        Self: Sized,
    {
        thread::spawn(move || read_events::<Self>(dst, polling_hz));
    }

    /// Execute an ANSI escape code, immediately flushing after.
    fn execute_ansi(&mut self, command: EscapeCode) -> Result<(), TerminalError> {
        self.write_bytes(command.to_string().as_bytes())?;
        self.flush()
    }

    fn queue_ansi(&mut self, command: EscapeCode) -> Result<(), TerminalError> {
        self.write_bytes(command.to_string().as_bytes())
    }

    /// Queue entering the alternate screen.
    fn enter_alternate_screen(&mut self) -> Result<(), TerminalError> {
        self.write_str(ALT_BUFFER_ENABLE)
    }

    /// Queue leaving the alternate screen.
    fn leave_alternate_screen(&mut self) -> Result<(), TerminalError> {
        self.write_str(ALT_BUFFER_DISABLE)
    }

    fn move_cursor(&mut self, x: u16, y: u16) -> Result<(), TerminalError> {
        self.execute_ansi(EscapeCode::CursorMove(x, y))
    }

    fn displace_cursor(&mut self, dx: i16, dy: i16) -> Result<(), TerminalError> {
        if dx.is_negative() {
            self.queue_ansi(EscapeCode::CursorLeft(dx.unsigned_abs()))?;
        } else {
            self.queue_ansi(EscapeCode::CursorRight(dx.unsigned_abs()))?;
        }

        if dy.is_negative() {
            self.queue_ansi(EscapeCode::CursorUp(dy.unsigned_abs()))
        } else {
            self.queue_ansi(EscapeCode::CursorDown(dy.unsigned_abs()))
        }
    }

    fn show_cursor(&mut self) -> Result<(), TerminalError> {
        self.write_str(CURSOR_VISIBLE)
    }

    fn hide_cursor(&mut self) -> Result<(), TerminalError> {
        self.write_str(CURSOR_INVISIBLE)
    }
}

fn read_events<B: Backend>(dst: Queue<Event>, polling_hz: u16) {
    let dur = Duration::from_secs_f32(1.0 / polling_hz as f32);
    loop {
        if let Some(event) = B::read_event() {
            dst.push(event);
        }
        thread::sleep(dur);
    }
}

#[cfg(test)]
mod tests {
    use std::io::{BufWriter, Stdout, Write, stdout};

    use super::*;

    struct TestBackend {
        buffer: BufWriter<Stdout>,
    }

    impl Default for TestBackend {
        fn default() -> Self {
            Self {
                buffer: BufWriter::new(stdout()),
            }
        }
    }

    impl Backend for TestBackend {
        fn read_event() -> Option<Event> {
            None
        }
        fn write_bytes(&mut self, bytes: &[u8]) -> Result<(), TerminalError> {
            let _ = self.buffer.write(bytes)?;
            Ok(())
        }
        fn size(&mut self) -> Result<(usize, usize), TerminalError> {
            Ok((0, 0))
        }
        fn enable_raw_mode(&mut self) -> Result<(), TerminalError> {
            Ok(())
        }
        fn disable_raw_mode(&mut self) -> Result<(), TerminalError> {
            Ok(())
        }
        fn flush(&mut self) -> Result<(), TerminalError> {
            self.buffer.flush()?;
            Ok(())
        }
    }

    #[ignore]
    #[test]
    fn write_varieties() {
        let mut test = TestBackend {
            buffer: BufWriter::new(std::io::stdout()),
        };

        test.write_bytes(b"Hello, world!\n").unwrap();
        test.flush().unwrap();
        test.write_str("Hello, world!\n").unwrap();
        test.write_char('Ⲙ').unwrap();
    }
}
