use engine_sdk::registry::{Component, uuid::uuid, EntityId};
use serde::{Serialize, Deserialize};

#[derive(Default, Clone, Copy, Serialize, Deserialize)]
pub struct Trap {
    pub triggered:bool,
    pub owner:Option<EntityId>
}

impl Component for Trap {
    fn type_id() -> engine_sdk::registry::uuid::Uuid {
        uuid!("d042688b-36fc-4153-a748-d1bc3097de39")
    }
}