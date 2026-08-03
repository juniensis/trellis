use std::collections::{HashMap, HashSet};

pub mod chunk;

use crate::{entities::Entity, id::Id};
use chunk::Chunk;

#[derive(Debug, Default)]
pub struct Board {
    chunks: HashMap<(i64, i64), Chunk>,
    entities: HashMap<Id, Entity>,
}
