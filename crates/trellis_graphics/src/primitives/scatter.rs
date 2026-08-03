use std::collections::HashMap;

use trellis_core::terminal::{Buffer, cell::Cell};

use crate::primitives::{Region, bounds::Bounds};
use trellis_core::primitives::Pos;

#[derive(Debug, Clone)]
pub struct Scatter {
    cells: HashMap<Pos, Cell>,
    z_order: u32,
}

impl Scatter {
    #[inline]
    pub fn build<P: Into<Pos>, C: IntoIterator<Item = (P, Cell)>>(cells: C, z_order: u32) -> Self {
        Self {
            cells: HashMap::from_iter(cells.into_iter().map(|(p, c)| (p.into(), c))),
            z_order,
        }
    }
    pub fn new(z_order: u32) -> Self {
        Self {
            cells: HashMap::new(),
            z_order,
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
            dst.set_cell(pos.x as u32, pos.y as u32, cell);
        }
    }
    #[inline]
    pub fn z_order(&self) -> u32 {
        self.z_order
    }
    #[inline]
    pub fn set_z_order(&mut self, z_order: u32) {
        self.z_order = z_order;
    }
    #[inline]
    pub fn merge(&mut self, other: &Self) {
        for (p, c) in other.iter() {
            if self.z_order < other.z_order {
                self.insert(p, c);
            } else {
                if !self.cells.contains_key(&p) {
                    self.insert(p, c);
                }
            }
        }
    }
    #[inline]
    pub fn merge_region(&mut self, other: &Region) {
        for (p, c) in other.positioned_iter() {
            if self.z_order < other.z_order() {
                self.insert(p, c);
            } else {
                if !self.cells.contains_key(&p) {
                    self.insert(p, c);
                }
            }
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
    fn bounds() {
        let mut scatter = Scatter::build([((0, 0), Cell::new(' ')), ((5, 5), Cell::new(' '))], 0);

        println!("{:?}", scatter.bounds());
    }
}
