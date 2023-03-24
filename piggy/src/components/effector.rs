use engine_sdk::world::Component;
use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Effector {
    ExitMarker
}

impl Default for Effector {
    fn default() -> Self {
        Effector::ExitMarker
    }
}

impl Component for Effector {
    fn id() -> engine_sdk::world::ComponentId {
        13
    }
}