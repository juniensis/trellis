use std::{
    fmt::Display,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: i16,
    pub y: i16,
}

impl Pos {
    #[inline]
    pub fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }
    #[inline]
    pub fn with_x(mut self, x: i16) -> Self {
        self.x = x;
        self
    }
    #[inline]
    pub fn with_y(mut self, y: i16) -> Self {
        self.y = y;
        self
    }
    #[inline]
    pub fn set_x(&mut self, x: i16) {
        self.x = x;
    }
    #[inline]
    pub fn set_y(&mut self, y: i16) {
        self.y = y;
    }
    #[inline]
    pub fn displace(&mut self, dx: i16, dy: i16) {
        self.x += dx;
        self.y += dy;
    }
    #[inline]
    pub fn displaced(mut self, dx: i16, dy: i16) -> Self {
        self.displace(dx, dy);
        self
    }
    #[inline]
    pub fn unsigned_saturating_add(self, rhs: Self) -> Self {
        Self::new(
            (self.x as u16).saturating_add_signed(rhs.x) as i16,
            (self.y as u16).saturating_add_signed(rhs.y) as i16,
        )
    }
}

impl Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl From<(u16, u16)> for Pos {
    fn from(value: (u16, u16)) -> Self {
        let (x, y) = value;
        Self::new(x as i16, y as i16)
    }
}

impl From<(u32, u32)> for Pos {
    fn from(value: (u32, u32)) -> Self {
        let (x, y) = value;
        Self::new(x as i16, y as i16)
    }
}

impl From<(usize, usize)> for Pos {
    fn from(value: (usize, usize)) -> Self {
        let (x, y) = value;
        Self::new(x as i16, y as i16)
    }
}

impl From<(i16, i16)> for Pos {
    fn from(value: (i16, i16)) -> Self {
        let (x, y) = value;
        Self::new(x, y)
    }
}

impl From<(i32, i32)> for Pos {
    fn from(value: (i32, i32)) -> Self {
        let (x, y) = value;
        Self::new(x as i16, y as i16)
    }
}

impl From<(i64, i64)> for Pos {
    fn from(value: (i64, i64)) -> Self {
        let (x, y) = value;
        Self::new(x as i16, y as i16)
    }
}

impl From<(isize, isize)> for Pos {
    fn from(value: (isize, isize)) -> Self {
        let (x, y) = value;
        Self::new(x as i16, y as i16)
    }
}

impl Add for Pos {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self.displaced(rhs.x, rhs.y)
    }
}

impl Sub for Pos {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self.displaced(-rhs.x, -rhs.y)
    }
}

impl AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        self.displace(rhs.x, rhs.y);
    }
}

impl SubAssign for Pos {
    fn sub_assign(&mut self, rhs: Self) {
        self.displace(-rhs.x, -rhs.y);
    }
}

impl Mul for Pos {
    type Output = Pos;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl MulAssign for Pos {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}
