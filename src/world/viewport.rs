use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Viewport {
    pub x: i64,
    pub y: i64,
    pub w: i64,
    pub h: i64,
}

impl Viewport {
    pub fn new(w: u32, h: u32) -> Self {
        let (w, h) = (w as i64, h as i64);
        Self { x: 0, y: 0, w, h }
    }
    /// Translate the world coordinates to screen space, returning None if
    /// the world coordinates are outside of the viewport.
    pub fn translate_world_coords(&self, x: i64, y: i64) -> Option<(u32, u32)> {
        let sx = x - self.x;
        let sy = self.y - y;

        if sx >= self.w || sy >= self.h {
            return None;
        }

        let sx = u32::try_from(sx).ok()?;
        let sy = u32::try_from(sy).ok()?;

        Some((sx, sy))
    }
    pub fn follow_cursor(&mut self, cursor_world_x: i64, cursor_world_y: i64, margin: i64) {
        let (sx, sy) = (cursor_world_x - self.x, self.y - cursor_world_y);

        if sx < margin {
            self.x -= margin - sx;
        } else if sx > self.w - 1 - margin {
            self.x += sx - (self.w - 1 - margin);
        }

        if sy < margin {
            self.y += margin - sy;
        } else if sy > self.h - 1 - margin {
            self.y -= sy - (self.h - 1 - margin);
        }
    }
    pub fn move_by(&mut self, dx: i64, dy: i64) {
        self.x += dx;
        self.y += dy;
    }
    pub fn move_to(&mut self, x: i64, y: i64) {
        self.x = x;
        self.y = y;
    }
    pub fn resize(&mut self, nw: i64, nh: i64) {
        self.w = nw;
        self.h = nh;
    }
    pub fn contains(&self, x: i64, y: i64) -> bool {
        x >= self.x && x <= self.x + self.w && y <= self.y && y >= self.y - self.h
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn viewport_contains() {
        let viewport = Viewport::new(20, 20);
        assert!(viewport.contains(0, 0));
        assert!(viewport.contains(10, -10));
        assert!(viewport.contains(0, -20));
        assert!(viewport.contains(20, -20));
    }
}
