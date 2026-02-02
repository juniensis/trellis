use std::io::{Write, stdout};

use crossterm::{cursor, execute};

pub struct Screen {
    current: Vec<u8>,
    buffer: Vec<u8>,
    pub width: u16,
    pub height: u16,
}

impl Screen {
    #[inline]
    pub fn new() -> Self {
        let dimensions = crossterm::terminal::window_size().unwrap();
        let (width, height) = (dimensions.columns, dimensions.rows);
        Self {
            buffer: vec![b' '; width as usize * height as usize],
            current: vec![b' '; width as usize * height as usize],
            width,
            height,
        }
    }
    #[inline]
    pub fn display(&mut self) {
        if self.buffer != self.current {
            let mut out = stdout();
            execute!(
                out,
                cursor::Hide,
                cursor::SavePosition,
                cursor::MoveTo(0, 0)
            )
            .unwrap();
            out.write_all(&self.buffer).unwrap();
            execute!(out, cursor::RestorePosition, cursor::Show).unwrap();
            out.flush().unwrap();
            self.current.copy_from_slice(self.buffer.as_slice());
        }
    }
    #[inline]
    pub fn update(&mut self) {
        let dimensions = crossterm::terminal::window_size().unwrap();
        if (dimensions.columns * dimensions.rows) > (self.width * self.height) {
            self.buffer.extend_from_slice(&vec![
                0u8;
                ((dimensions.columns * dimensions.rows) - (self.width * self.height))
                    as usize
            ]);
        }
        (self.width, self.height) = (dimensions.columns, dimensions.rows);
    }
    #[inline]
    pub fn clear(&mut self) {
        self.buffer.fill(b' ');
    }
    #[inline]
    pub fn flush(&mut self) {
        self.current.copy_from_slice(&self.buffer);
    }
    #[inline]
    pub fn write_to_cell(&mut self, pos: (usize, usize), ch: u8) {
        self.buffer[(self.width as usize * pos.1) + pos.0] = ch;
    }
    #[inline]
    pub fn write_slice(&mut self, pos: (usize, usize), buf: &[u8]) {
        let idx = (self.width as usize * pos.1) + pos.0;
        self.buffer[idx..idx + buf.len()].copy_from_slice(buf);
    }
    pub fn normalize_x(&self, x: u16) -> u16 {
        x.min(self.width)
    }
    pub fn normalize_y(&self, y: u16) -> u16 {
        y.min(self.height)
    }
}

impl Default for Screen {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod screen_t {
    use std::{thread::sleep, time::Duration};

    use super::*;

    #[test]
    fn init_t() {
        let mut sc = Screen::new();
        let (w, h) = (sc.width, sc.height);

        for i in 0..w {
            for j in 0..h {
                sc.write_to_cell((i as usize, j as usize), b'@');
                sleep(Duration::from_secs_f64(1.0 / 240.0));
                sc.display();
            }
        }
        sc.display();
    }
}
