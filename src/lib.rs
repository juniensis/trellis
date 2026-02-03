#![allow(dead_code, unused_imports)]
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::collections::HashMap;

pub mod cell;
pub mod components;
pub mod context;
pub mod viewport;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    KeyPressed {
        code: KeyCode,
        modifier: KeyModifiers,
    },
    MoveTo(u16, u16),
    MoveBy(i16, i16),
}
