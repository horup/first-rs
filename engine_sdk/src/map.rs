use egui::epaint::ahash::HashMap;
use glam::Vec3;
use serde::{Serialize, Deserialize};
use crate::Grid;

#[derive(Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Entity {
    pub pos:Vec3,
    pub atlas:u32,
    pub atlas_index:u32,
    pub class:String,
    pub properties:HashMap<String, String>
}

#[derive(Clone, Copy, Default, Serialize, Deserialize, PartialEq)]
pub struct Wall {

}

#[derive(Clone, Copy, Default, Serialize, Deserialize, PartialEq)]
pub struct MapCell {
    #[serde(default)]
    pub wall:Option<u32>,
    #[serde(default)]
    pub thing:Option<u32>,
    #[serde(default)]
    pub thing_facing:f32,
    #[serde(default)]
    pub wall_index:u16,
    #[serde(default)]
    pub thing_index:u16,
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