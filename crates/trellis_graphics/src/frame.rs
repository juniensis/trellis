use trellis_core::primitives::Pos;

use crate::{
    primitives::{Primitive, RenderCtx, Renderable},
    viewport::Viewport,
};

pub enum Command {
    Draw(Primitive),
    Transform(Box<dyn FnOnce(Primitive) -> Primitive>),
}

pub struct Frame<'a> {
    viewport: &'a mut Viewport,
    buffer: Vec<(Pos, Primitive)>,
    ctx: RenderCtx,
}

impl<'a> Frame<'a> {
    pub fn new(viewport: &'a mut Viewport, ctx: RenderCtx) -> Self {
        Self {
            viewport,
            buffer: Vec::new(),
            ctx,
        }
    }
    pub fn with_draw<R: Renderable>(mut self, position: impl Into<Pos>, object: &R) -> Self {
        self.buffer.push((position.into(), object.render(self.ctx)));
        self
    }
    pub fn draw<R: Renderable>(&mut self, position: impl Into<Pos>, object: &R) {
        self.buffer.push((position.into(), object.render(self.ctx)));
    }
    pub fn submit(mut self) {
        let viewport = self.viewport;
        let mut buffer = self.buffer;
        buffer.sort_by_key(|(_, x)| x.z_order());
        for (pos, primitive) in buffer {
            viewport.composite(pos, primitive);
        }
        viewport.flush();
    }
    pub fn submit_with_hidden_cursor(mut self) {
        let viewport = self.viewport;
        viewport.backend.hide_cursor();
        let mut buffer = self.buffer;
        buffer.sort_by_key(|(_, x)| x.z_order());
        for (pos, primitive) in buffer {
            viewport.composite(pos, primitive);
        }
        viewport.flush();
        viewport.backend.show_cursor();
    }
}
