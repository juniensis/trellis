use crate::components::primitives::Rectangle;

pub enum EditMode {
    Normal,
    Insert,
}

pub struct TextBox {
    container: Rectangle,
    text: Vec<String>,
    x: u16,
    y: u16,
    mode: EditMode,
}

impl TextBox {
    pub fn new(x: u16, y: u16, w: u16, h: u16, initial: &str) -> Self {
        Self {
            container: Rectangle::new(x, y, w, h),
            text: vec![initial.to_string()],
            x: u16,
            y: u16,
            mode: Normal,
        }
    }
}
