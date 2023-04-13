use registry::{Component, uuid::uuid};
use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct SoundEmitter {
    pub sound:u32,
    pub position_secs:f64,
    pub loops:bool
}

impl SoundEmitter {
    pub fn once(sound:u32) -> Self {
        Self {
            sound,
            position_secs: 0.0,
            loops: false,
        }
    }

    pub fn loops(sound:u32) -> Self {
        Self {
            sound,
            position_secs: 0.0,
            loops: true,
        }
    }
}

impl Component for SoundEmitter {
    fn type_id() -> registry::uuid::Uuid {
        uuid!("29b46439-ca3f-4845-b739-703235351a29")
    }
}