use crate::{Trellis, viewport::Viewport};
use crossterm::event::{Event, KeyEvent};
use std::io::Write;

pub mod primitives;
pub mod text;

pub trait Component {
    fn update(&mut self, event: crate::Event) -> Option<crate::Event>;
    fn is_inside(&self, x: u16, y: u16) -> bool;
    fn draw(&self, viewport: &mut Viewport);
    fn erase(&self, viewport: &mut Viewport);
}
