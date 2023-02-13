use serde::{Serialize, Deserialize, de::DeserializeOwned};
use slotmap::{SecondaryMap, Key};
use crate::{CSDUnsafeCell};

#[derive(Default, Serialize, Clone)]
pub struct Components<K:Key, T : Copy + Clone> {
    inner:SecondaryMap<K, CSDUnsafeCell<T>>
}

type E<K, T> = SecondaryMap<K, CSDUnsafeCell<T>>;

impl<'de, T : Copy + Clone + Serialize + Deserialize<'de>, K:Key> Deserialize<'de> for Components<K, T> {
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


impl<T : Copy + Clone, K:Key> Components<K, T> {
    pub fn attach(&mut self, id:K, cmp:T) {
        self.inner.insert(id, CSDUnsafeCell::new(cmp));
    }

    pub fn detach(&mut self, id:K) {
        self.inner.remove(id);
    }

    pub fn iter_mut(&self) -> IterMut<T, K> {
        IterMut {
            iter:self.inner.iter()
        }
    }

    pub fn iter(&self) -> Iter<T, K> {
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