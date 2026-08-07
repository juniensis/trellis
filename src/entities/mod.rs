use trellis_graphics::primitives::{RenderCtx, Renderable};

use crate::{entities::text_box::TextBox, id::Id};

pub mod text_box;

#[derive(Debug)]
pub enum EntityKind {
    TextBox(TextBox),
    Connector { from: Id, to: Id },
}

impl EntityKind {
    pub fn width(&self) -> i64 {
        match self {
            Self::TextBox(t) => t.width() as i64,
            Self::Connector { from, to } => todo!(),
        }
    }
    pub fn height(&self) -> i64 {
        match self {
            Self::TextBox(t) => t.height() as i64,
            Self::Connector { from, to } => todo!(),
        }
    }
    pub fn contains(&self, entity_x: i64, entity_y: i64, cursor_x: i64, cursor_y: i64) -> bool {
        match self {
            Self::TextBox(t) => {
                let (x1, x2) = (entity_x, entity_x + self.width());
                let (y1, y2) = (entity_y, entity_y - self.height());
                cursor_x >= x1 && cursor_x <= x2 && cursor_y <= y1 && cursor_y >= y2
            }
            Self::Connector { from, to } => todo!(),
        }
    }
}

#[derive(Debug)]
pub struct Entity {
    pub id: Id,
    pub kind: EntityKind,
    pub x: i64,
    pub y: i64,
}

impl Entity {
    pub fn new(id: Id, kind: EntityKind, x: i64, y: i64) -> Self {
        Self { id, kind, x, y }
    }
    pub fn contains(&self, x: i64, y: i64) -> bool {
        self.kind.contains(self.x, self.y, x, y)
    }
}

impl Renderable for Entity {
    fn render(&self, ctx: RenderCtx) -> trellis_graphics::primitives::Primitive {
        match &self.kind {
            EntityKind::TextBox(t) => t.render(ctx),
            _ => todo!(),
        }
    }
}
