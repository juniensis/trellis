use trellis_graphics::primitives::{Primitive, RenderCtx, Renderable};

use crate::{
    block::{Block, Text},
    connector::Connector,
    world::{bounds::Bounds, id::Id},
};

#[derive(Debug)]
pub enum EntityKind {
    Block(Block),
    Connector(Connector),
}

#[derive(Debug)]
pub struct Entity {
    id: Id,
    kind: EntityKind,
    bounds: Bounds,
}

impl Entity {
    pub fn new_block(id: Id, world_x: i64, world_y: i64) -> Self {
        let block = Block {
            id,
            world_x,
            world_y,
            width: 4,
            height: 2,
            corners: ['+'; 4],
            verticals: '|',
            horizontals: '-',
            contents: Text::new(),
        };

        let bounds = block.bounds();

        Self {
            id,
            kind: EntityKind::Block(block),
            bounds,
        }
    }
    pub fn update_bounds(&mut self) {
        let bounds = match &self.kind {
            EntityKind::Block(b) => b.bounds(),
            EntityKind::Connector(c) => todo!(),
        };

        self.bounds = bounds;
    }
    pub fn contains(&self, x: i64, y: i64) -> bool {
        match &self.kind {
            EntityKind::Block(b) => b.bounds().contains(x, y),
            EntityKind::Connector(b) => todo!(),
        }
    }
    pub fn x(&self) -> i64 {
        match &self.kind {
            EntityKind::Block(b) => b.world_x,
            EntityKind::Connector(c) => todo!(),
        }
    }
    pub fn y(&self) -> i64 {
        match &self.kind {
            EntityKind::Block(b) => b.world_y,
            EntityKind::Connector(c) => todo!(),
        }
    }
}

impl Renderable for Entity {
    fn render(&self, ctx: RenderCtx) -> Primitive {
        match &self.kind {
            EntityKind::Block(b) => b.render(ctx),
            _ => todo!(),
        }
    }
}
