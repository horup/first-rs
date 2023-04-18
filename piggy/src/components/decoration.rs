use engine_sdk::registry::{Component, uuid::uuid};
use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, Clone, Copy)]
pub struct Decoration {
}

impl Component for Decoration {
    fn type_id() -> engine_sdk::registry::uuid::Uuid {
        uuid!("2f881f90-b230-48dc-b516-cb588dfd6785")
    }
}