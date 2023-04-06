use engine_sdk::registry::{Component, uuid::uuid};
use serde::{Serialize, Deserialize};

#[derive(Default, Clone, Copy, Serialize, Deserialize)]
pub struct EmitSound {
    pub looping:bool,
    pub sound:u32 
}

impl EmitSound {
    pub fn new(sound:u32) -> Self {
        Self {
            looping:false,
            sound
        }
    }
}

impl Component for EmitSound {
    fn type_id() -> engine_sdk::registry::uuid::Uuid {
        uuid!("c37ca796-f25d-4f39-931c-a618a72ced99")
    }
}