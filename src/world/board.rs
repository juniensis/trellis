use std::collections::{HashMap, HashSet};

use crate::world::{entity::Entity, id::Id, viewport::Viewport};

pub struct Chunk {
    chunk_x: i32,
    chunk_y: i32,
    visitors: HashSet<Id>,
}

fn chunk_of(x: i64, y: i64) -> (i32, i32) {
    (x.div_euclid(64) as i32, y.div_euclid(64) as i32)
}

impl Chunk {
    pub fn new(chunk_x: i32, chunk_y: i32) -> Self {
        Self {
            chunk_x,
            chunk_y,
            visitors: HashSet::new(),
        }
    }
}

#[derive(Debug, Default)]
pub struct Board {
    entities: HashMap<Id, Entity>,
    counter: u32,
    freelist: Vec<Id>,
}

impl Board {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn create_block(&mut self, x: i64, y: i64) {
        let id = self.next_id();
        let block = Entity::new_block(id, x, y);
        self.entities.insert(id, block);
    }
    pub fn remove_entity(&mut self, id: Id) -> Option<Entity> {
        self.entities.remove(&id)
    }
    pub fn next_id(&mut self) -> Id {
        if let Some(free) = self.freelist.pop() {
            free
        } else {
            let ret = Id::new(0, self.counter);
            self.counter += 1;
            ret
        }
    }
    pub fn free_id(&mut self, id: Id) {
        self.freelist.push(Id {
            generation: id.generation + 1,
            id: id.id,
        });
    }
    pub fn all_within_viewport(&self, viewport: &Viewport) -> impl Iterator<Item = &Entity> {
        self.entities
            .values()
            .filter(|&x| viewport.contains(x.x(), x.y()))
    }
    pub fn try_get_at_cursor(
        &self,
        viewport: &Viewport,
        cursor_x: i64,
        cursor_y: i64,
    ) -> Option<&Entity> {
        self.all_within_viewport(viewport)
            .find(|&x| x.contains(cursor_x, cursor_y))
    }
}
