use std::marker::PhantomData;

use trellis_core::{collections::Queue, terminal::buffer::Buffer};
use trellis_terminal::{Terminal, backend::Backend};
use trellis_ui::render::region::Region;

pub struct Viewport<Message> {
    term: Terminal,
    pos: (i32, i32),
    size: (u16, u16),
    front: Buffer,
    back: Buffer,
    _marker: PhantomData<Message>,
}

impl<Message> Viewport<Message> {
    pub fn new() -> Self {
        let mut term = Terminal::new();
        let sz = term.size().expect("sz");

        let buf = Buffer::new(sz.0, sz.1);
        Self {
            term,
            pos: (0, 0),
            size: (sz.0 as u16, sz.1 as u16),
            front: buf.clone(),
            back: buf,
            _marker: PhantomData,
        }
    }
    pub fn composite_region(&mut self, region: Region, pos: (u16, u16)) {
        for y in 0..region.size().1 {
            for x in 0..region.size().0 {
                if let Some(cell) = region.get_cell(x, y) {
                    self.back
                        .set_cell((x + pos.0) as usize, (y + pos.1) as usize, *cell);
                }
            }
        }
    }
    pub fn diff(&mut self) {
        for (left, right) in self.back.iter().zip(self.front.iter_mut()) {
            if left != right {
                *right = *left;
            }
        }
    }
    pub fn flush(&mut self) {
        self.term
            .execute_ansi(trellis_terminal::ansi::EscapeCode::ClearAll)
            .unwrap();
        self.term.write_str(&self.front.to_string()).unwrap();
        self.term.flush().unwrap();
    }
}

#[cfg(test)]
mod tests {}
