use engine_sdk::{registry::{uuid::{uuid, Uuid}, Component}, Camera, Collision};
use serde::{Serialize, Deserialize};

use crate::systems::Flash;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Global {
    pub flash:Flash,
    pub camera:Camera,
    pub collisions:Vec<Collision>
}

impl Component for Global {
    fn type_id() -> Uuid {
        uuid!("3555ae54-18b6-4c72-a740-3ff6ee4102ae")
    }
}