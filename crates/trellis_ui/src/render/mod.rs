use trellis_core::terminal::{cell::Cell, point::Point};

pub mod frame;
pub mod primitives;

pub trait Renderer {
    fn begin_layer(&mut self);
    fn end_layer(&mut self);
    fn draw_cell(&mut self, pos: Point, cell: Cell);
}
