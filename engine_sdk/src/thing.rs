use serde::{Serialize, Deserialize};
use slotmap::{new_key_type, SlotMap, basic::Keys};

new_key_type! {pub struct Id;}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Things {
    things:SlotMap<Id, ()>,
}

impl Things {
    pub fn spawn(&mut self) -> Id {
        self.things.insert(())
    }

    pub fn despawn(&mut self, thing:Id) {
        self.things.remove(thing);
    }

    pub fn iter<'a>(&self) -> Keys<Id, ()> {
        self.things.keys()
    }
}

pub struct Iter<'a> {
    iter:slotmap::basic::Keys<'a, Id, ()>
}

impl<'a> Iterator for Iter<'a> {
    type Item = Id;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}