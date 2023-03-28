use engine_sdk::{registry::Singleton, Camera, Collision};
use serde::{Serialize, Deserialize};

use crate::systems::Flash;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Global {
    pub flash:Flash,
    pub camera:Camera,
    pub collisions:Vec<Collision>
}

impl Singleton for Global {
    fn id() -> engine_sdk::registry::SingletonId {
        20
    }
}