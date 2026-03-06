use std::collections::{HashMap, HashSet};

use trellis_ui::render::region::Region;

use crate::world::{
    elements::Element,
    id::{ElementID, SystemID},
    systems::System,
};

pub mod elements;
pub mod id;
pub mod systems;

pub struct World<Message> {
    elements: HashMap<ElementID, Element<Message>>,
    systems: HashMap<SystemID, System>,
    collision: HashMap<(i32, i32), ElementID>,
}

impl<Message> World<Message> {
    pub fn new() -> Self {
        Self {
            elements: HashMap::new(),
            systems: HashMap::new(),
            collision: HashMap::new(),
        }
    }
    pub fn add_element(&mut self, element: Element<Message>) -> ElementID {
        let id = ElementID::new();
        for coords in element.coords_contained() {
            self.collision.insert(coords, id);
        }
        self.elements.insert(id, element);
        id
    }
    pub fn remove_element(&mut self, id: ElementID) -> Option<Element<Message>> {
        self.elements.remove(&id)
    }
    pub fn send_message_to(
        &mut self,
        message: Message,
        targets: &[ElementID],
    ) -> Option<Vec<Message>> {
        let mut ret = Vec::new();
        for id in targets.iter() {
            if let Some(target) = self.elements.get_mut(id)
                && let Some(tail) = target.widget.update(&message)
            {
                ret.push(tail);
            }
        }

        (!ret.is_empty()).then_some(ret)
    }
    pub fn send_message_to_all(&mut self, message: Message) -> Option<Vec<Message>> {
        let mut ret = Vec::new();

        for target in self.elements.values_mut() {
            if let Some(tail) = target.widget.update(&message) {
                ret.push(tail);
            }
        }

        (!ret.is_empty()).then_some(ret)
    }
    pub fn get_visible_regions(&self, pos: (i32, i32), dim: (u16, u16)) -> Vec<(u16, u16, Region)> {
        let mut ret = Vec::new();
        let mut seen = HashSet::new();

        for coord in (pos.1..pos.1 + dim.1 as i32)
            .flat_map(move |y| (pos.0..pos.0 + dim.0 as i32).map(move |x| (x, y)))
        {
            if let Some(id) = self.collision.get(&coord)
                && !seen.contains(id)
                && let Some(elem) = self.elements.get(id)
            {
                if let Some(trans) = elem.coords_transformed(pos, dim) {
                    ret.push((trans.0, trans.1, elem.widget.render(Region::new())));
                }
                seen.insert(*id);
            }
        }

        ret
    }
}
