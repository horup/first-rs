
use serde::{Serialize, Deserialize};

#[derive(Default, Clone, Copy, Serialize, Deserialize)]
pub struct Player {
    pub pokemoncards:u32,
    pub has_key_gold:bool,
    pub has_key_blue:bool
}