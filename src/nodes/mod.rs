use crossterm::{cursor::MoveTo, event::KeyEvent};

use crate::{Mode, Trellis, nodes::textbox::TextBox, screen::Screen};

pub mod textbox;

pub trait Node {
    fn draw(&mut self, screen: &mut Screen);
    fn update(&mut self, event: KeyEvent) -> (u16, u16, Option<Mode>);
}
