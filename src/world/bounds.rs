#[derive(Debug)]
pub struct BoundingBox {
    pub x0: i64,
    pub y0: i64,
    pub x1: i64,
    pub y1: i64,
}

impl BoundingBox {
    pub fn new(x0: i64, y0: i64, x1: i64, y1: i64) -> Self {
        Self { x0, x1, y0, y1 }
    }
    pub fn contains(&self, x: i64, y: i64) -> bool {
        x >= self.x0 && x <= self.x1 && y <= self.y0 && y >= self.y1
    }
    pub fn overlaps_box(&self, other: &BoundingBox) -> bool {
        let (x0, x1, y0, y1) = (other.x0, other.x1, other.y0, other.y1);
        self.x0 <= x1 && self.x1 >= x0 && self.y0 <= y1 && self.y1 >= y0
    }
    pub fn overlaps_cloud(&self, other: &BoundingCloud) -> bool {
        other.overlaps_box(self)
    }
    pub fn overlaps(&self, other: &Bounds) -> bool {
        match other {
            Bounds::Box(b) => self.overlaps_box(b),
            Bounds::Cloud(c) => self.overlaps_cloud(c),
        }
    }
}
#[derive(Debug)]

pub struct BoundingCloud {
    pub points: Vec<(i64, i64)>,
}

impl BoundingCloud {
    pub fn new(points: impl IntoIterator<Item = (i64, i64)>) -> Self {
        Self {
            points: points.into_iter().collect(),
        }
    }
    pub fn contains(&self, x: i64, y: i64) -> bool {
        for (cx, cy) in self.points.iter() {
            if (x + 1 == *cx || x == *cx || x - 1 == *cx)
                && (y + 1 == *cy || y == *cy || y - 1 == *cy)
            {
                return true;
            }
        }
        false
    }
    pub fn overlaps_cloud(&self, other: &BoundingCloud) -> bool {
        for (cx, cy) in self.points.iter() {
            for (vx, vy) in other.points.iter() {
                if cx == vx && cy == vy {
                    return true;
                }
            }
        }
        false
    }
    pub fn overlaps_box(&self, other: &BoundingBox) -> bool {
        for (cx, cy) in self.points.iter() {
            if other.contains(*cx, *cy) {
                return true;
            }
        }
        false
    }
    pub fn overlaps(&self, other: &Bounds) -> bool {
        match other {
            Bounds::Box(b) => self.overlaps_box(b),
            Bounds::Cloud(c) => self.overlaps_cloud(c),
        }
    }
}

#[derive(Debug)]
pub enum Bounds {
    Box(BoundingBox),
    Cloud(BoundingCloud),
}

impl Bounds {
    pub fn contains(&self, x: i64, y: i64) -> bool {
        match self {
            Self::Cloud(c) => c.contains(x, y),
            Self::Box(b) => b.contains(x, y),
        }
    }
    pub fn overlaps(&self, other: &Self) -> bool {
        match self {
            Self::Cloud(c) => c.overlaps(other),
            Self::Box(b) => b.overlaps(other),
        }
    }
}
