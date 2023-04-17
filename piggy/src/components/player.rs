use engine_sdk::{registry::{Component, uuid::{Uuid, uuid}}, Timer, egui::epaint::ahash::HashMap, Pic};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Inventory {
    items:HashMap<Pic, f32>
}

impl Inventory {
    pub fn has(&self, item_type: Pic) -> bool {
        self.amount(item_type) > 0.0
    }
    pub fn amount(&self, item_type: Pic) -> f32 {
        if let Some(amount) = self.items.get(&item_type) {
            return *amount;
        }
        0.0
    }

    pub fn add(&mut self, item_type: Pic, amount:f32) {
        if let Some(current) = self.items.get_mut(&item_type) {
            *current += amount;
        }
        else {
            self.items.insert(item_type, amount);
        }
    }

    pub fn set(&mut self, item_type: Pic, amount:f32) {
        if let Some(current) = self.items.get_mut(&item_type) {
            *current = amount;
        }
        else {
            self.items.insert(item_type, amount);
        }
    }
}

#[derive(Default, Clone, Serialize, Deserialize)]
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
    Active {
        fade_in_timer:Timer
    },
    BeingCought {
        turn_around_timer:Timer
    },
    Cought {
        fade_out_timer:Timer,
    },
    CanRespawn,
    Escaped {
        fade_out_timer:Timer
    },
    CanContinue {
    },
    ShowFinalScore {
       
    }
}

impl PlayerState {
    pub fn is_active(&self) -> bool {
        match  self {
            PlayerState::Active { fade_in_timer } => {
                if fade_in_timer.is_done() {
                    return true;
                }

                false
            },
            _=>false
        }
    }
    pub fn is_being_cought_or_cought(&self) -> bool {
        match self {
            Self::BeingCought { .. } | Self::Cought { .. } => true,
            _=> false
        }
    }
    pub fn set_being_cought(&mut self) {
        match self {
            Self::Active { .. } => {
                *self = Self::BeingCought { turn_around_timer: Timer::new(1.0) }
            },
            _=>{}
        }
    }
    pub fn set_cought(&mut self) {
        match self {
            PlayerState::BeingCought {..} => *self = Self::Cought {
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

    pub fn set_escaped(&mut self) {
        match *self {
            PlayerState::Active {..} => {
                let timeout = 1.0;
                *self = PlayerState::Escaped { fade_out_timer: Timer::new(timeout)  }
            },
            _ => {}
        }
    }
    pub fn set_can_continue(&mut self) {
        match self {
            Self::Escaped { .. } => {
                *self = Self::CanContinue {  }
            },
            _=>{}
        }
    }

    pub fn set_final(&mut self) {
        *self = PlayerState::ShowFinalScore {  };
    }
}

impl Default for PlayerState {
    fn default() -> Self {
        Self::Active { fade_in_timer:Timer::new(1.0) }
    }
}
