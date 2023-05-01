use engine_sdk::{registry::{Component, uuid::uuid}, Timer};
use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Modifier {
    Trapped {
        expire:Timer
    }
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Modifiers {
    pub modifiers:Vec<Modifier>
}

impl Component for Modifiers {
    fn type_id() -> engine_sdk::registry::uuid::Uuid {
        uuid!("27c3f7e4-b95a-4c2c-803a-8af468953a61")
    }
}

impl Modifiers {
    pub fn is_trapped(&self) -> bool {
        for modifier in self.modifiers.iter() {
            match modifier {
                Modifier::Trapped { .. } => return true,
            }
        }
        false
    }

    pub fn trap(&mut self, secs:f32) {
        self.modifiers.push(Modifier::Trapped { expire: Timer::new(secs) });
    }
}