

use registry::{Component, uuid::{uuid, Uuid}};
use serde::{Serialize, Deserialize};
use crate::{Grid, Tile};

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Tilemap {
    pub grid:Grid<Tile>,
    pub floor_texture:u32,
    pub ceiling_texture:u32
}

impl Component for Tilemap {
    fn type_id() -> Uuid {
        uuid!("e799fda3-1ef8-438f-a22b-ea4cfdc17941")
    }
}