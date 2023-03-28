use engine_sdk::registry::Component;
use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Serialize, Deserialize, Default)]
pub struct Item {
    pub amount:f32
}

impl Component for Item {
    fn id() -> engine_sdk::registry::ComponentId {
        12
    }
}

impl Item {
    pub fn new(amount:f32) -> Self {
        Self {
            amount
        }
    }
}
