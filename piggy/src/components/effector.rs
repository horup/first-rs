use engine_sdk::registry::Component;
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
    fn id() -> engine_sdk::registry::ComponentId {
        13
    }
}