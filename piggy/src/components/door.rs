use engine_sdk::{glam::Vec3, registry::{Component, uuid::{uuid, Uuid}}};
use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Serialize, Deserialize, Default)]
pub struct Door {
    pub openess:f32,
    pub close_timer:f32,
    pub direction:f32,
    pub pos:Vec3
}

impl Component for Door {
    fn type_id() -> Uuid {
        uuid!("cb502ab6-c152-4c03-8c5f-2aa039979166")
    }
}


impl Door {
    pub fn time_to_open(&self) -> f32 {
        1.0
    }

    pub fn time_to_start_closing(&self) -> f32 {
        2.0
    }

    pub fn open(&mut self) {
        self.direction = 1.0;
        self.close_timer = 0.0;
    }

    pub fn close(&mut self) {
        self.direction = -1.0;
        self.close_timer = 0.0;
    }

    pub fn is_open(&self) -> bool {
        self.openess == 1.0
    }
}