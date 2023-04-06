use engine_sdk::registry::{Component, uuid::{Uuid, uuid}};
use serde::{Serialize, Deserialize};

use crate::sounds;

#[derive(Clone, Copy, Serialize, Deserialize, Default)]
pub struct Item {
    pub amount:f32,
    pub pickup_sound:u32
}

impl Component for Item {
    fn type_id() -> Uuid {
        uuid!("4d30694e-759a-4efb-92d7-c0b1ddcb8c4e")
    }
}

impl Item {
    pub fn new(amount:f32) -> Self {
        Self {
            amount,
            pickup_sound:sounds::PICKUP
        }
    }

    pub fn with_pickup_sound(mut self, pickup_sound:u32) -> Self {
        self.pickup_sound = pickup_sound;
        self
    }
}
