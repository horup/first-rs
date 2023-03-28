use engine_sdk::registry::Component;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Copy, Serialize, Deserialize)]
pub struct Inventory {
    items: [(u32, f32); 32],
}

impl Inventory {
    pub fn has(&self, item_type: u32) -> bool {
        let amount = self.amount(item_type);
        amount > 0.0
    }
    pub fn amount(&self, item_type: u32) -> f32 {
        return self.items.iter().find(|(item_type2, _)| {
            *item_type2 == item_type
        }).map_or(0.0, |item| item.1);
    }

    pub fn add(&mut self, item_type: u32, amount:f32) {
        let current = self.amount(item_type);
        self.set(item_type, amount + current);
    }

    pub fn set(&mut self, item_type: u32, amount:f32) {
        let mut item = self.items.iter_mut().find(|(item_type2, _)| *item_type2 == item_type);
        if item.is_none() {
            item = self.items.iter_mut().find(|(item_type2, _)| *item_type2 == 0);
        }

        let item = item.expect("failed to set item");
        item.0 = item_type;
        item.1 = amount;
    }
}

#[derive(Default, Copy, Clone, Serialize, Deserialize)]
pub struct Player {
    pub inventory: Inventory,
    pub state:PlayerState
}

impl Component for Player {
    fn id() -> engine_sdk::registry::ComponentId {
        10
    }
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum PlayerState {
    Ready,
    Cought {
        timer_sec:f32,
    },
    CanRespawn
}

impl PlayerState {
    pub fn cought(&mut self) {
        match self {
            PlayerState::Ready => *self = Self::Cought {
                timer_sec: 2.0,
            },
            _ => {}
        }
    }

    pub fn ready(&mut self) {
        *self = PlayerState::Ready;
    }

    pub fn to_ready_to_respawn(&mut self) {
        match self {
            PlayerState::Ready => {},
            _=> {}
        }
    }

    pub fn can_respawn(&mut self) {
        match  self {
            PlayerState::Cought { .. } => {
                *self = PlayerState::CanRespawn;
            },
            _ => {},
        }
    }
}

impl Default for PlayerState {
    fn default() -> Self {
        Self::Ready
    }
}
