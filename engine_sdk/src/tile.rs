use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Tile {
    pub wall:Option<u32>,
    pub clips:bool
}
