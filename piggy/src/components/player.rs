use serde::{Serialize, Deserialize};

#[derive(Default, Clone, Copy, Serialize, Deserialize)]
pub struct Player {
    pub pokemoncards:u32
}