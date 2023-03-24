use engine_sdk::world::EntityId;
use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Health {
    pub current:f32,
    pub max:f32,
    pub killer:Option<EntityId>
}

impl Health {
    pub fn is_alive(&self) -> bool {
        self.current > 0.0
    }

    pub fn kill(&mut self, killer:Option<EntityId>) {
        self.current = 0.0;
        self.killer = killer;
    }
}

impl Default for Health {
    fn default() -> Self {
        let max = 100.0;
        Self { current: max, max, killer:None }
    }
}