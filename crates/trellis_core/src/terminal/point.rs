#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

impl Point {
    #[inline]
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
    #[inline]
    pub fn move_by(&mut self, dx: i16, dy: i16) {
        self.x = self.x.saturating_add_signed(dx);
        self.y = self.y.saturating_add_signed(dy);
    }
    #[inline]
    pub fn move_to(&mut self, x: u16, y: u16) {
        self.x = x;
        self.y = y;
    }
}
