use trellis_core::{
    primitives::Pos,
    terminal::{Buffer, Cell},
};
use trellis_terminal::backend::Backend;

use crate::primitives::Primitive;

pub struct Viewport {
    front: Buffer,
    back: Buffer,
    cursor: Pos,
    pub backend: Box<dyn Backend>,
}

#[derive(Clone, Copy)]
pub struct DifferingCell {
    x: u32,
    y: u32,
    cell: Cell,
}

impl Viewport {
    pub fn new(mut backend: Box<dyn Backend>) -> crate::Result<Self> {
        let (w, h) = backend.size()?;
        let back = Buffer::new(w, h);
        Ok(Self {
            front: back.clone(),
            back,
            cursor: Pos::new(0, 0),
            backend,
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
    pub fn resize(&mut self, nw: u32, nh: u32) {
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
        let (w, h) = self.backend.size().unwrap();
        if self.front.width() != w || self.front.height() != h {
            self.resize(w, h);
        }
    }
    pub fn width(&self) -> u32 {
        self.back.width()
    }
    pub fn height(&self) -> u32 {
        self.back.height()
    }
    pub fn dimensions(&self) -> (u32, u32) {
        (self.width(), self.height())
    }
}

// Cursor controls
impl Viewport {
    #[inline]
    pub fn move_cursor_to(&mut self, pos: impl Into<Pos>) {
        let pos = pos.into();
        if self
            .backend
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
            .backend
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
            .backend
            .move_cursor(self.cursor.x as u16, self.cursor.y as u16)
            .is_err()
        {
            eprintln!("Warning: Failed to position cursor.")
        }
    }
}

// Drawing
impl Viewport {
    pub fn composite(&mut self, pos: Pos, prim: impl Into<Primitive>) {
        let prim: Primitive = prim.into();
        match prim {
            Primitive::Region(r) => {
                r.composite(pos, &mut self.back);
            }
            Primitive::Scatter(s) => {
                s.composite(pos, &mut self.back);
            }
        }
    }
    pub fn flush(&mut self) {
        let difference = self.write_difference();
        let (w, h) = self.front.dimensions();
        if difference.len() as u32 >= (w * h) / 2 {
            self.backend
                .write_bytes(self.front.string().as_bytes())
                .unwrap();
        } else {
            for cell in difference {
                self.backend
                    .move_cursor(cell.x as u16, cell.y as u16)
                    .unwrap();
                self.backend.write_str(&cell.cell.to_string()).unwrap();
            }
        }
        self.backend
            .move_cursor(self.cursor.x as u16, self.cursor.y as u16)
            .unwrap();
    }
}
