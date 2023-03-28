use engine_sdk::{glam::Vec3, registry::Component};
use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Serialize, Deserialize, Default)]
pub struct Door {
    pub openess:f32,
    pub close_timer:f32,
    pub direction:f32,
    pub pos:Vec3
}

impl Component for Door {
    fn id() -> engine_sdk::registry::ComponentId {
        14
    }
}


impl Door {
    pub fn time_to_open(&self) -> f32 {
        1.0
    }

    pub fn time_to_start_closing(&self) -> f32 {
        5.0
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