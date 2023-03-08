use serde::{Serialize, Deserialize};
use slotmap::{new_key_type, SlotMap, basic::Keys};

new_key_type! {pub struct Thing;}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Things {
    inner:SlotMap<Thing, ()>
}

impl Things {
    pub fn spawn(&mut self) -> Thing {
        self.inner.insert(())
    }

    pub fn despawn(&mut self, thing:Thing) {
        self.inner.remove(thing);
    }

    pub fn iter<'a>(&self) -> Keys<Thing, ()> {
        self.inner.keys()
    }
}

pub struct Iter<'a> {
    iter:slotmap::basic::Keys<'a, Thing, ()>
}

impl<'a> Iterator for Iter<'a> {
    type Item = Thing;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

#[test]
fn test() {
    let mut things = Things::default();
    let thing1 = things.spawn();
    let thing2 = things.spawn();

    things.despawn(thing1);
    things.spawn();
    for thing in things.iter() {
        dbg!(thing);
    }
}