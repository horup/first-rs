use serde::{Serialize, Deserialize};
use slotmap::new_key_type;
use crate::{Grid, Sprite, Entities};

new_key_type! {pub struct SpriteId;}

#[derive(Default, Clone, Copy, Serialize, Deserialize)]
pub struct Cell {
    pub wall:Option<u32>
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Scene {
    pub sprites:Entities<SpriteId, Sprite>,
    pub ceiling_texture:u32,
    pub floor_texture:u32,
    pub grid:Grid<Cell>
}

