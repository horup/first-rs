use engine_sdk::registry::Component;
use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Activator {
    Door {
        key:Option<u32>
    }
}

impl Component for Activator {
    fn id() -> engine_sdk::registry::ComponentId {
        15
    }
}

impl Default for Activator {
    fn default() -> Self {
        Self::Door { key: None }
    }
}