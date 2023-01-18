use serde::{Serialize, Deserialize};
use crate::Grid;

#[derive(Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct MapCell {
    #[serde(default)]
    pub wall:Option<u32>,
    #[serde(default)]
    pub thing:Option<u32>
}

#[derive(Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Map {
    pub version:u16,
    pub grid:Grid<MapCell>
}

impl Map {
    pub fn new(size:usize) -> Self {
        Self {
            grid:Grid::new(size),
            version:1
        }
    }
}