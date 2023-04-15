use serde::{Serialize, Deserialize};
use crate::Pic;

#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Tile {
    pub wall:Option<Pic>,
    pub clips:bool
}
