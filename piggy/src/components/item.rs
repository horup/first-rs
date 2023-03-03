use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum KeyType {
    Gold,
    Blue
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Item {
    PokemonCard,
    Key {
        key_type:KeyType
    }
}

impl Default for Item {
    fn default() -> Self {
        Self::PokemonCard
    }
}

