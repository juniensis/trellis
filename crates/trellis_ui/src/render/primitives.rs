use trellis_core::terminal::{cell::Cell, point::Point};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    width: u16,
    height: u16,
}

impl Rect {
    pub fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }
    pub fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
    }
    pub fn resize_by(&mut self, dw: i16, dh: i16) {
        self.width = self.width.saturating_add_signed(dw);
        self.height = self.height.saturating_add_signed(dh);
    }
}

pub struct Circle {
    radius: u16,
}

impl Circle {
    pub fn new(radius: u16) -> Self {
        Self { radius }
    }
    pub fn resize(&mut self, radius: u16) {
        self.radius = radius;
    }
    pub fn resize_by(&mut self, dr: i16) {
        self.radius = self.radius.saturating_add_signed(dr)
    }
}

pub struct Line {
    len: f32,
    angle: f32,
}

impl Line {
    pub fn new(len: f32, angle: f32) -> Self {
        Self { len, angle }
    }
    pub fn resize(&mut self, len: f32) {
        self.len = len;
    }
    pub fn resize_by(&mut self, len: f32) {
        self.len += len;
    }
    pub fn rotate_to(&mut self, angle: f32) {
        self.angle = angle;
    }
    pub fn rotate_by(&mut self, da: f32) {
        self.angle += da;
    }
}

pub struct Bezier {}

/// Vertices are given in Cartesian coordinates where 0,0 is the point the
/// polygon will be positioned around.
pub struct Polygon {
    vertices: Vec<Point>,
}

pub struct Text {
    text: String,
}

impl Text {
    pub fn new<S: ToString>(text: S) -> Self {
        Self {
            text: text.to_string(),
        }
    }
    pub fn change_text<S: ToString>(&mut self, text: S) {
        self.text = text.to_string();
    }
}

pub struct Particle {
    bounds: Rect,
    cells: Vec<Cell>,
}
