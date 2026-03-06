use crate::render::{primitives::Primitive, region::Region};

pub mod primitives;
pub mod region;
pub mod styles;

pub trait Renderable {
    fn render(&self) -> Region;
}

pub trait Renderer {
    fn composite(&mut self, x: u16, y: u16, region: Region);
    fn flush(&mut self);

    fn draw<R: Renderable>(&mut self, x: u16, y: u16, renderable: R) {
        let region = renderable.render();
        self.composite(x, y, region);
    }
}
