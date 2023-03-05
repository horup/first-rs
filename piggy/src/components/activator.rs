use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Activator {
    Door {
        key:Option<u32>
    }
}

impl Default for Activator {
    fn default() -> Self {
        Self::Door { key: None }
    }
}