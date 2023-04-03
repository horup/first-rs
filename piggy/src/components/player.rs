use engine_sdk::{registry::{Component, uuid::{Uuid, uuid}}, Timer};
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
    fn type_id() -> Uuid {
        uuid!("0fd10bc1-9587-466b-9576-e4672d397530")
    }
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum PlayerState {
    Ready {
        fade_in_timer:Timer
    },
    Cought {
        fade_out_timer:Timer,
    },
    CanRespawn,
    Won {
        fade_out_timer:Timer
    },
    CanContinue {
        
    }
}

impl PlayerState {
    pub fn set_cought(&mut self) {
        match self {
            PlayerState::Ready {..} => *self = Self::Cought {
                fade_out_timer: Timer::new(1.0),
            },
            _ => {}
        }
    }

    pub fn set_can_respawn(&mut self) {
        match  self {
            PlayerState::Cought { .. } => {
                *self = PlayerState::CanRespawn;
            },
            _ => {},
        }
    }

    pub fn set_won(&mut self) {
        match *self {
            PlayerState::Ready {..} => {
                let timeout = 1.0;
                *self = PlayerState::Won { fade_out_timer: Timer::new(timeout)  }
            },
            _ => {}
        }
    }
}

impl Default for PlayerState {
    fn default() -> Self {
        Self::Ready { fade_in_timer:Timer::new(1.0) }
    }
}
