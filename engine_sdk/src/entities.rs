use serde::{Serialize, Deserialize, de::DeserializeOwned};
use slotmap::{SlotMap, Key};
use crate::{CSDUnsafeCell};

#[derive(Default, Serialize, Clone)]
pub struct Entities<K:Key, T:Copy + Clone + Serialize + DeserializeOwned> {
    inner:SlotMap<K, CSDUnsafeCell<T>>
}

type E<K, T> = SlotMap<K, CSDUnsafeCell<T>>;

impl<'de, K, T> Deserialize<'de> for Entities<K, T> where K:Key, T:Copy + Clone + Serialize + DeserializeOwned {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        match E::deserialize(deserializer) {
            Ok(inner) => {
                Ok(Entities {
                    inner
                })
            },
            Err(err) => {
                Err(err)
            },
        }
    }
}

pub struct IterMut<'a, K:Key, T:Copy + Clone + Serialize + DeserializeOwned> {
    iter:slotmap::basic::Iter<'a, K, CSDUnsafeCell<T>>
}

impl<'a, K, T:Copy + Clone + Serialize + DeserializeOwned> Iterator for IterMut<'a, K, T> where K:Key {
    type Item = (K, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((id, e)) = self.iter.next() {
            let e = unsafe { &mut *e.get() };
            return Some((id, e));
        }

        None
    }
}

pub struct Iter<'a, K:Key, T:Copy + Clone + Serialize + DeserializeOwned> {
    iter:slotmap::basic::Iter<'a, K, CSDUnsafeCell<T>>
}

impl<'a, K:Key, T:Copy + Clone + Serialize + DeserializeOwned> Iterator for Iter<'a, K, T> {
    type Item = (K, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((id, e)) = self.iter.next() {
            let e = unsafe { & *e.get() };
            return Some((id, e));
        }

        None
    }
}


impl<K,T:Copy + Clone + Serialize + DeserializeOwned> Entities<K, T> where K:Key {
    pub fn spawn(&mut self, sprite:T) -> K {
        self.inner.insert(CSDUnsafeCell::new(sprite))
    }

    pub fn despawn(&mut self, id:K) {
        self.inner.remove(id);
    }

    pub fn iter_mut(&self) -> IterMut<K, T> {
        IterMut {
            iter:self.inner.iter()
        }
    }

    pub fn iter(&self) -> Iter<K, T> {
        Iter {
            iter:self.inner.iter()
        }
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    pub fn get(&self, id:K) -> Option<&T> {
        if let Some(e) = self.inner.get(id) {
            return Some(unsafe {& *e.get()});
        }

        None
    }

    pub fn get_mut(&self, id:K) -> Option<&mut T> {
        if let Some(e) = self.inner.get(id) {
            return Some(unsafe {&mut *e.get()});
        }

        None
    } 

    pub fn len(&self) -> usize {
        self.inner.len()
    }
}