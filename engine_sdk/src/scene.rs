use crate::Grid;

#[derive(Default, Clone, Copy)]
pub struct Cell {
    pub wall:Option<u32>
}

#[derive(Default, Clone)]
pub struct Scene {
    pub grid:Grid<Cell>
}

