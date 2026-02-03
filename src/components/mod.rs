use crate::{context::Context, viewport::Viewport};
use crossterm::event::{Event, KeyEvent};
use std::io::Write;

pub mod primitives;
pub mod text;

pub trait Component {
    fn update(&mut self, event: crate::Event);
    fn draw(&self, viewport: &mut Viewport);
    fn erase(&self, viewport: &mut Viewport);
}
