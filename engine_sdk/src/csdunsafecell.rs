use std::{cell::UnsafeCell, ops::{Deref, DerefMut}};
use serde::{Serialize, Deserialize};

#[derive(Default)]
pub struct CSDUnsafeCell<T:Clone>(UnsafeCell<T>);

impl<'de, T : Clone> CSDUnsafeCell<T> {
    pub fn new(t:T) -> Self {
        Self(UnsafeCell::new(t))
    }
}

impl<T : Clone> Clone for CSDUnsafeCell<T> {
    fn clone(&self) -> Self {
        let e = unsafe {
            &*self.0.get()
        };

        CSDUnsafeCell(UnsafeCell::new(e.clone()))
    }
}

impl<T : Clone + Serialize> Serialize for CSDUnsafeCell<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let e = unsafe {
            &*self.0.get()
        };

        e.serialize(serializer)
    }
}

impl<'de, T : Clone + Deserialize<'de>> Deserialize<'de> for CSDUnsafeCell<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        match T::deserialize(deserializer) {
            Ok(t) => {
                Ok(Self(UnsafeCell::new(t)))
            },
            Err(err) => {
                Err(err)
            },
        }

    }
}

impl<T : Clone> Deref for CSDUnsafeCell<T> {
    type Target = UnsafeCell<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T : Clone> DerefMut for CSDUnsafeCell<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}