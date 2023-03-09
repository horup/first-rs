use serde::{Serialize, Deserialize, de::DeserializeOwned};
use slotmap::{SecondaryMap, Key};
use crate::{CSDUnsafeCell, EntityId};

#[derive(Default, Serialize, Clone)]
pub struct Components<T : Copy + Clone> {
    inner:SecondaryMap<EntityId, CSDUnsafeCell<T>>
}

type E<K, T> = SecondaryMap<K, CSDUnsafeCell<T>>;

impl<'de, T : Copy + Clone + Serialize + Deserialize<'de>> Deserialize<'de> for Components<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        match E::deserialize(deserializer) {
            Ok(inner) => {
                Ok(Components {
                    inner
                })
            },
            Err(err) => {
                Err(err)
            },
        }
    }
}

pub struct IterMut<'a, T : Copy + Clone, K:Key> {
    iter:slotmap::secondary::Iter<'a, K, CSDUnsafeCell<T>>
}

impl<'a, T : Copy + Clone, K:Key> Iterator for IterMut<'a, T, K> {
    type Item = (K, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((id, e)) = self.iter.next() {
            let e = unsafe { &mut *e.get() };
            return Some((id, e));
        }

        None
    }
}

pub struct Iter<'a, T : Copy + Clone, K:Key> {
    iter:slotmap::secondary::Iter<'a, K, CSDUnsafeCell<T>>
}

impl<'a, T : Serialize + DeserializeOwned + Copy + Clone, K:Key> Iterator for Iter<'a, T, K> {
    type Item = (K, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((id, e)) = self.iter.next() {
            let e = unsafe { & *e.get() };
            return Some((id, e));
        }

        None
    }
}


impl<T : Copy + Clone> Components<T> {
    pub fn attach(&mut self, id:EntityId, cmp:T) {
        self.inner.insert(id, CSDUnsafeCell::new(cmp));
    }

    pub fn detach(&mut self, id:EntityId) {
        self.inner.remove(id);
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    pub fn get(&self, id:EntityId) -> Option<&T> {
        if let Some(e) = self.inner.get(id) {
            return Some(unsafe {& *e.get()});
        }

        None
    }

    pub fn get_mut(&self, id:EntityId) -> Option<&mut T> {
        if let Some(e) = self.inner.get(id) {
            return Some(unsafe {&mut *e.get()});
        }

        None
    } 

    pub fn len(&self) -> usize {
        self.inner.len()
    }
}