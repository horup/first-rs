use engine_sdk::registry::{EntityId, Component, uuid::uuid};
use serde::{Serialize, Deserialize};


#[derive(Default, Clone, Copy, Serialize, Deserialize)]
pub struct Local {
    me:Option<EntityId>
}

impl Component for Local {
    fn type_id() -> engine_sdk::registry::uuid::Uuid {
        uuid!("2a0c2467-6f54-4d8f-8b62-3f167cc07b1a")
    }
}