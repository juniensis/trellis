use serde::{Deserialize, Serialize};

pub mod error;

use crate::{
    block::Block,
    connector::Connector,
    world::{id::Id, viewport::Viewport},
};

#[derive(Deserialize, Serialize)]
pub struct TrellisFile {
    world_cursor_x: i64,
    world_cursor_y: i64,
    viewport: Viewport,
    blocks: Vec<Block>,
    connectors: Vec<Connector>,
}

impl TrellisFile {
    pub fn new(cursor_x: i64, cursor_y: i64, viewport: Viewport) -> Self {
        Self {
            world_cursor_x: cursor_x,
            world_cursor_y: cursor_y,
            viewport,
            blocks: Vec::new(),
            connectors: Vec::new(),
        }
    }
    pub fn set_cursor(&mut self, cursor_x: i64, cursor_y: i64) {
        self.world_cursor_x = cursor_x;
        self.world_cursor_y = cursor_y;
    }
    pub fn set_viewport(&mut self, viewport: Viewport) {
        self.viewport = viewport;
    }
    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }
    pub fn add_connector(&mut self, connector: Connector) {
        self.connectors.push(connector);
    }
    pub fn to_bytes(&self) -> error::Result<Vec<u8>> {
        serde_json::to_vec_pretty(self).map_err(|_| error::Error::MalformedFile)
    }
    pub fn from_bytes(bytes: &[u8]) -> error::Result<Self> {
        serde_json::from_slice(bytes).map_err(|_| error::Error::MalformedFile)
    }
}
