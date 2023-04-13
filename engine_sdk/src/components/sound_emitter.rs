use registry::{Component, uuid::uuid};
use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, Clone, Copy)]
pub struct SoundEmitter {
    pub sound:u32,
    pub position_secs:f32,
    pub loops:bool
}

impl Component for SoundEmitter {
    fn type_id() -> registry::uuid::Uuid {
        uuid!("29b46439-ca3f-4845-b739-703235351a29")
    }
}