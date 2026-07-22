use trellis_core::{
    primitives::Pos,
    terminal::{Buffer, Cell},
};
use trellis_terminal::backend::Backend;

use crate::render::Renderer;
use crate::{primitives::*, render::Frame};

pub struct Viewport {
    front: Buffer,
    back: Buffer,
    cursor: Pos,
    term: Box<dyn Backend>,
}

#[derive(Clone, Copy)]
pub struct DifferingCell {
    x: usize,
    y: usize,
    cell: Cell,
}

impl Viewport {
    #[inline]
    pub fn new(mut backend: Box<dyn Backend>) -> crate::Result<Self> {
        let (w, h) = backend.size()?;
        let back = Buffer::new(w, h);
        Ok(Self {
            front: back.clone(),
            back,
            cursor: Pos::new(0, 0),
            term: backend,
        })
    }
    /// Write all differing cells to the front buffer, and clear the back
    /// buffer at the same time.
    pub fn write_difference(&mut self) -> Vec<DifferingCell> {
        let mut ret = Vec::new();
        for (((x, y), dst), src) in self.front.positioned_iter_mut().zip(self.back.iter_mut()) {
            if dst != src {
                *dst = *src;
                ret.push(DifferingCell { x, y, cell: *dst });
            }
            *src = Cell::new(' ');
        }
        ret
    }
    pub fn resize(&mut self, nw: usize, nh: usize) {
        let mut new_front = Buffer::new(nw, nh);
        let mut new_back = new_front.clone();

        for (f, b) in self
            .front
            .positioned_iter()
            .zip(self.back.positioned_iter())
        {
            new_front.set_cell(f.0.0, f.0.1, *f.1);
            new_back.set_cell(b.0.0, b.0.1, *b.1);
        }

        self.front = new_front;
        self.back = new_back;
    }
    pub fn update_size(&mut self) {
        let (w, h) = self.term.size().unwrap();
        if self.front.width() != w || self.front.height() != h {
            self.resize(w, h);
        }
    }
}

// Cursor controls
impl Viewport {
    #[inline]
    pub fn move_cursor_to(&mut self, pos: impl Into<Pos>) {
        let pos = pos.into();
        if self
            .term
            .move_cursor(pos.x.max(0) as u16, pos.y.max(0) as u16)
            .is_ok()
        {
            self.cursor = pos;
        } else {
            eprintln!("Warning: Failed to position cursor.")
        }
    }
    #[inline]
    pub fn move_cursor_by(&mut self, dx: impl Into<Pos>) {
        let pos = dx.into();
        let altered = self.cursor.unsigned_saturating_add(pos);
        if self
            .term
            .move_cursor(altered.x as u16, altered.y as u16)
            .is_ok()
        {
            self.cursor = altered;
        } else {
            eprintln!("Warning: Failed to position cursor.")
        }
    }
    #[inline]
    pub fn verify_cursor(&mut self) {
        if self
            .term
            .move_cursor(self.cursor.x as u16, self.cursor.y as u16)
            .is_err()
        {
            eprintln!("Warning: Failed to position cursor.")
        }
    }
}

impl Renderer for Viewport {
    fn composite(&mut self, pos: Pos, primitive: Primitive) {
        match primitive {
            Primitive::Region(r) => {
                r.composite(pos, &mut self.back);
            }
            Primitive::Scatter(s) => {
                s.composite(pos, &mut self.back);
            }
        }
    }
    fn flush(&mut self) {
        let difference = self.write_difference();
        for cell in difference {
            self.term.move_cursor(cell.x as u16, cell.y as u16).unwrap();
            self.term.write_str(&cell.cell.to_string()).unwrap();
        }
        self.term
            .move_cursor(self.cursor.x as u16, self.cursor.y as u16)
            .unwrap();
    }
    fn begin_pass(&mut self) -> crate::render::Frame<'_, Self> {
        Frame::new(self)
    }
    fn end_pass(&mut self) {
        self.flush();
    }
}

#[cfg(test)]
mod tests {
    use std::{
        thread::sleep,
        time::{Duration, Instant},
    };

    use trellis_terminal::Terminal;

    use crate::render::shapes::Rect;

    use super::*;

    #[test]
    fn viewport_draw() {
        let rect = Rect::new(5, 5).with_fill(Cell::new('x'));

        let backend = Terminal::default();
        let mut renderer = Viewport::new(Box::new(backend)).unwrap();
        renderer.begin_pass().draw(rect, (0, 0)).end_pass();
        sleep(Duration::from_millis(50));
        renderer.begin_pass().draw(rect, (6, 6)).end_pass();
        sleep(Duration::from_millis(50));
    }

    #[test]
    fn animate() {
        let mut initial_rect = Rect::new(5, 4).with_fill(Cell::new('x'));
        let mut backend = Terminal::default();
        backend.enter_alternate_screen();
        backend.enable_raw_mode();
        let (w, h) = backend.size().unwrap();
        let mut renderer = Viewport::new(Box::new(backend)).unwrap();

        let mut x = 0;
        let mut y = 0;
        let mut dx = 1;
        let mut dy = 1;

        let now = Instant::now();
        for i in 0..240 {
            renderer.begin_pass().draw(initial_rect, (x, y)).end_pass();
            if x + dx > (w - 5) as i32 || x + dx <= 0 {
                dx = -dx;
            }
            if y + dy > (h - 4) as i32 || y + dy <= 0 {
                dy = -dy;
            }
            x += dx;
            y += dy;
        }
        let elapsed = now.elapsed().as_secs_f32();

        let fps = 1.0 / (elapsed / 240.0);
        drop(renderer);
        println!("{fps}");
    }
    #[test]
    fn stress() {
        let mut backend = Terminal::default();
        backend.enter_alternate_screen();
        backend.enable_raw_mode();
        let (w, h) = backend.size().unwrap();
        let mut renderer = Viewport::new(Box::new(backend)).unwrap();

        let chars = ['@', '#', '%', '&', '+', '=', '-', ':', '.'];

        let scatters = chars
            .iter()
            .map(|x| Rect::new(w as u16, h as u16).with_fill(Cell::new(*x)))
            .collect::<Vec<_>>();

        let now = Instant::now();
        for i in 0..240 {
            renderer
                .begin_pass()
                .draw(scatters[i % scatters.len()], (0, 0))
                .end_pass();
        }
        let elapsed = now.elapsed().as_secs_f32();

        let fps = 1.0 / (elapsed / 240.0);
        drop(renderer);
        println!("{fps}");
    }
}
