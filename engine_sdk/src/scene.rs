use serde::{Serialize, Deserialize};

use crate::Grid;

#[derive(Default, Clone, Copy, Serialize, Deserialize)]
pub struct Cell {
    pub wall:Option<u32>
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Scene {
    pub ceiling_texture:u32,
    pub floor_texture:u32,
    pub grid:Grid<Cell>
}

