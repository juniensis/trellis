use crate::render::{primitives::Primitive, region::Region};

pub mod primitives;
pub mod region;
pub mod styles;

pub trait Renderer {
    fn begin_layer(&self, initial_width: u16, initial_height: u16) -> Region;
    fn commit_layer(&mut self, x_offset: u16, y_offset: u16);

    /*
    pub fn draw_primitive(&self, mut region: Region, primitive: Primitive) -> Region {
        primitive
    }*/
}
