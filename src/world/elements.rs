use trellis_ui::render::region::Region;

use crate::{widgets::Widget, world::id::ElementID};

pub struct Element<Message> {
    pub widget: Box<dyn Widget<Message>>,
    pos: (i32, i32),
}

impl<Message> Element<Message> {
    pub fn bounds(&self) -> (i32, i32, i32, i32) {
        let x = self.pos.0;
        let y = self.pos.1;
        let mut tmp = Region::new();
        let (w, h) = self.widget.render(tmp).size();

        (x, x + w as i32, y, y + h as i32)
    }
    pub fn coords_contained(&self) -> impl Iterator<Item = (i32, i32)> {
        let (x1, x2, y1, y2) = self.bounds();

        (y1..y2).flat_map(move |y| (x1..x2).map(move |x| (x, y)))
    }
    pub fn coords(&self) -> (i32, i32) {
        self.pos
    }
    pub fn coords_transformed(&self, offset: (i32, i32), size: (u16, u16)) -> Option<(u16, u16)> {
        let (vx, vy) = offset;
        let (wx, wy) = self.pos;
        let screen_x = wx - vx;
        let screen_y = wy - vy;

        let (term_width, term_height) = (size.0 as i32, size.1 as i32);

        if screen_x < 0 || screen_y < 0 || screen_x >= term_width || screen_y >= term_height {
            return None; // fully offscreen, cull
        }

        Some((screen_x as u16, screen_y as u16))
    }
}
