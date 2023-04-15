use engine_sdk::Def;
use serde::{Serialize, Deserialize};

#[derive(PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Tool {
    PlaceWall,
    PlaceEntity
}

impl ToString for Tool {
    fn to_string(&self) -> String {
        match self {
            Tool::PlaceWall {..} => "Wall".to_string(),
            Tool::PlaceEntity {..}=> "Thing".to_string(),
        }
    }
}

impl Default for Tool {
    fn default() -> Self {
        Self::PlaceWall
    }
}