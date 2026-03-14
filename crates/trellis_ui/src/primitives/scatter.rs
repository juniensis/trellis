use std::collections::HashMap;

use trellis_core::terminal::{Buffer, cell::Cell};

use crate::primitives::bounds::Bounds;
use trellis_core::primitives::Pos;

#[derive(Debug, Clone)]
pub struct Scatter {
    cells: HashMap<Pos, Cell>,
}

impl Scatter {
    #[inline]
    pub fn new<P: Into<Pos>, C: IntoIterator<Item = (P, Cell)>>(cells: C) -> Self {
        Self {
            cells: HashMap::from_iter(cells.into_iter().map(|(p, c)| (p.into(), c))),
        }
    }
    #[inline]
    pub fn insert<P: Into<Pos>>(&mut self, pos: P, cell: Cell) {
        self.cells.insert(pos.into(), cell);
    }
    #[inline]
    pub fn remove<P: Into<Pos>>(&mut self, pos: P) -> Option<(Pos, Cell)> {
        let p = pos.into();
        self.cells.remove(&p).map(|x| (p, x))
    }
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (Pos, Cell)> {
        self.cells.iter().map(|(x, y)| (*x, *y))
    }
    #[inline]
    pub fn composite(&self, pos: impl Into<Pos>, dst: &mut Buffer) {
        let offset = pos.into();
        for (p, cell) in self.iter() {
            let pos = p + offset;
            dst.set_cell(pos.x as usize, pos.y as usize, cell);
        }
    }
    #[inline]
    pub fn bounds(&self) -> Bounds {
        let (mut x_min, mut x_max) = (i16::MAX, i16::MIN);
        let (mut y_min, mut y_max) = (i16::MAX, i16::MIN);

        for (p, _) in self.iter() {
            x_min = x_min.min(p.x);
            x_max = x_max.max(p.x);
            y_min = y_min.min(p.y);
            y_max = y_max.max(p.y);
        }

        Bounds::new(x_min..=x_max, y_min..=y_max)
    }
}

#[cfg(test)]
mod tests {
    use trellis_core::terminal::cell::Cell;

    use crate::primitives::{self, scatter::Scatter};

    #[test]
    fn bounds_t() {
        let mut scatter = Scatter::new([((0, 0), Cell::new(' ')), ((5, 5), Cell::new(' '))]);

        println!("{:?}", scatter.bounds());
    }
}
