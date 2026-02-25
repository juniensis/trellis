use std::fmt::Debug;

use crate::Style;

/// General events relevant to the UI. Primitive widgets accept these as a
/// message.
#[derive(Debug, Clone)]
pub enum UiEvent {
    MoveTo(usize, usize),
    MoveBy(isize, isize),
    Resize(usize, usize),
    String(String),
    Bytes(Vec<u8>),
    Integer(isize),
    UnsignedInteger(usize),
    Float(f64),
    SetStyle(Style),
    None,
}
