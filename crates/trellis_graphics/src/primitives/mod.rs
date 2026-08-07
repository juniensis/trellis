pub mod bounds;
pub mod region;
pub mod scatter;

pub use bounds::Bounds;
pub use region::Region;
pub use scatter::Scatter;
use trellis_core::{
    primitives::Pos,
    terminal::{
        Cell,
        style::{Stylable, Style},
    },
};

#[derive(Debug, Default, Clone, Copy)]
pub struct RenderCtx {
    pub viewport_width: u32,
    pub viewport_height: u32,
    pub time: f32,
    pub delta: f32,
}

pub trait Renderable {
    fn render(&self, ctx: RenderCtx) -> Primitive;
}

#[derive(Debug, Clone)]
pub enum Primitive {
    Region(Region),
    Scatter(Scatter),
}

impl From<Region> for Primitive {
    fn from(value: Region) -> Self {
        Self::Region(value)
    }
}

impl From<Scatter> for Primitive {
    fn from(value: Scatter) -> Self {
        Self::Scatter(value)
    }
}

impl Primitive {
    pub fn z_order(&self) -> u32 {
        match self {
            Primitive::Region(r) => r.z_order(),
            Primitive::Scatter(s) => s.z_order(),
        }
    }
    pub fn set_z_order(&mut self, z_order: u32) {
        match self {
            Primitive::Region(r) => r.set_z_order(z_order),
            Primitive::Scatter(s) => s.set_z_order(z_order),
        }
    }
    pub fn merge(&mut self, other: &Self) {
        match self {
            Primitive::Region(rx) => match other {
                Primitive::Region(ry) => rx.merge(ry),
                Primitive::Scatter(sy) => rx.merge_scatter(sy),
            },
            Primitive::Scatter(sx) => match other {
                Primitive::Region(ry) => sx.merge_region(ry),
                Primitive::Scatter(sy) => sx.merge(sy),
            },
        }
    }
    pub fn new_scatter(z_order: u32) -> Self {
        Self::Scatter(Scatter::new(z_order))
    }
    pub fn new_region(z_order: u32) -> Self {
        Self::Region(Region::new(z_order))
    }
    pub fn set_cell(&mut self, pos: impl Into<Pos>, cell: Cell) {
        let pos = pos.into();
        match self {
            Self::Region(r) => r.set_cell(pos.x as u32, pos.y as u32, cell),
            Self::Scatter(s) => s.insert(pos, cell),
        }
    }
    pub fn write_str<S: AsRef<str>>(&mut self, pos: impl Into<Pos>, string: S) {
        let pos = pos.into();
        for (offset, ch) in string.as_ref().chars().enumerate() {
            let x = pos.x + offset as i16;
            self.set_cell((x, pos.y), Cell::new(ch));
        }
    }
    pub fn write_str_styled<S: AsRef<str>>(
        &mut self,
        pos: impl Into<Pos>,
        string: S,
        style: Style,
    ) {
        let pos = pos.into();
        for (offset, ch) in string.as_ref().chars().enumerate() {
            let x = pos.x + offset as i16;
            self.set_cell((x, pos.y), Cell::new(ch).with_style(style));
        }
    }
}

impl Renderable for Primitive {
    fn render(&self, ctx: RenderCtx) -> Primitive {
        self.clone()
    }
}
