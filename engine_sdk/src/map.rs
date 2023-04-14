use serde::{Serialize, Deserialize};
use crate::{Grid, Pic};

#[derive(Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Entity {
    pub pic:Pic,
    pub facing:f32,
    pub class:String
}

#[derive(Clone, Copy, Default, Serialize, Deserialize, PartialEq)]
pub struct Wall {
    pub pic:Pic
}

#[derive(Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct MapCell {
    #[serde(default)]
    pub wall:Option<Wall>,
    #[serde(default)]
    pub entity:Option<Entity>
}

#[derive(Clone, Default, Serialize, Deserialize, PartialEq)]
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