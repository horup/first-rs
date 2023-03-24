use std::default;

use engine_sdk::world::Singleton;
use serde::{Serialize, Deserialize};

use crate::systems::Flash;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Global {
    pub flash:Flash
}

impl Singleton for Global {
    fn id() -> engine_sdk::world::SingletonId {
        20
    }
}