use serde::{Serialize, Deserialize};
use slotmap::{new_key_type, SlotMap, basic::Keys};

new_key_type! {pub struct EntityId;}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Entities {
    ids:SlotMap<EntityId, ()>,
}

impl Entities {
    pub fn spawn(&mut self) -> EntityId {
        self.ids.insert(())
    }

    pub fn despawn(&mut self, thing:EntityId) {
        self.ids.remove(thing);
    }

    pub fn iter<'a>(&self) -> Keys<EntityId, ()> {
        self.ids.keys()
    }
}

pub struct Iter<'a> {
    iter:slotmap::basic::Keys<'a, EntityId, ()>
}

impl<'a> Iterator for Iter<'a> {
    type Item = EntityId;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}