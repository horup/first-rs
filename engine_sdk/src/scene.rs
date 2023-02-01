use serde::{Serialize, Deserialize};

use crate::Grid;

#[derive(Default, Clone, Copy, Serialize, Deserialize)]
pub struct Cell {
    pub wall:Option<u32>
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Scene {
    pub grid:Grid<Cell>
}

