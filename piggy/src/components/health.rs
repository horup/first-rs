use engine_sdk::registry::{EntityId, Component, uuid::{Uuid, uuid}};
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

impl Component for Health {
    fn type_id() -> Uuid {
        uuid!("f5e29883-d9cc-480f-8c80-dba9e721bac3")
    }
}

impl Default for Health {
    fn default() -> Self {
        let max = 100.0;
        Self { current: max, max, killer:None }
    }
}