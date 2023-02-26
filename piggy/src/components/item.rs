use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Item {
    PokemonCard
}

impl Default for Item {
    fn default() -> Self {
        Self::PokemonCard
    }
}

