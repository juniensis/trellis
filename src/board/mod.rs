use std::collections::{HashMap, HashSet};

pub mod chunk;

use crate::{
    entities::{Entity, EntityKind, text_box::TextBox},
    id::Id,
    viewport::Viewport,
};
use chunk::Chunk;

#[derive(Debug, Default)]
pub struct Board {
    //chunks: HashMap<(i64, i64), Chunk>,
    entities: HashMap<Id, Entity>,
    counter: u32,
    freelist: Vec<Id>,
}

impl Board {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn create_textbox(&mut self, x: i64, y: i64) {
        let id = self.next_id();

        let mut textbox = Entity::new(id, EntityKind::TextBox(TextBox::default()), x, y);
        self.entities.insert(textbox.id, textbox);
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
}

impl Board {
    pub fn all_within_viewport(&self, viewport: &Viewport) -> impl Iterator<Item = &Entity> {
        self.entities
            .values()
            .filter(|&x| viewport.contains(x.x, x.y))
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
