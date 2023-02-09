use serde::{Serialize, Deserialize};

use crate::{Grid, Sprite};

#[derive(Default, Clone, Copy, Serialize, Deserialize)]
pub struct Cell {
    pub wall:Option<u32>
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Scene {
    pub sprites:Vec<Sprite>,
    pub ceiling_texture:u32,
    pub floor_texture:u32,
    pub grid:Grid<Cell>
}

