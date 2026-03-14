use crate::primitives::Primitive;
use trellis_core::primitives::Pos;

pub mod shapes;
pub mod styles;

pub trait Renderable {
    fn render(&self) -> Primitive;
}

pub trait Renderer: Sized {
    fn composite(&mut self, pos: Pos, primitive: Primitive);
    fn flush(&mut self);

    fn begin_pass(&mut self) -> Frame<'_, Self>;
    fn end_pass(&mut self);
}

pub struct Frame<'a, Renderer: crate::render::Renderer + Sized> {
    inner: &'a mut Renderer,
}

impl<'a, Renderer: crate::render::Renderer> Frame<'a, Renderer> {
    pub fn new(r: &'a mut Renderer) -> Self {
        Self { inner: r }
    }
    pub fn draw<R: Renderable, P: Into<Pos>>(mut self, obj: R, pos: P) -> Self {
        let prim = obj.render();
        self.inner.composite(pos.into(), prim);
        self
    }
    pub fn end_pass(mut self) {
        self.inner.end_pass();
    }
}
