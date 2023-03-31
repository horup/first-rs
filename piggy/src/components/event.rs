use engine_sdk::registry::{Component, uuid::uuid};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct RespawnEvent {

}

#[derive(Serialize, Deserialize, Clone)]
pub enum Event {
    Respawn(RespawnEvent)
}

impl Default for Event {
    fn default() -> Self {
        Self::Respawn(RespawnEvent {  })
    }
}

impl Component for Event {
    fn type_id() -> engine_sdk::registry::uuid::Uuid {
        uuid!("49bb8d29-9d53-4f83-86c3-9e35b03514c0")
    }
}