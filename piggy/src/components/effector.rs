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