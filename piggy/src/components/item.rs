use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Item {
    PokemonCard,
    Key {
        key_type:u32
    }
}

impl Default for Item {
    fn default() -> Self {
        Self::PokemonCard
    }
}

