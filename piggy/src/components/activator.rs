use serde::{Serialize, Deserialize};

use super::KeyType;

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Activator {
    Door {
        key:Option<KeyType>
    }
}

impl Default for Activator {
    fn default() -> Self {
        Self::Door { key: None }
    }
}