

use registry::{Component, uuid::{uuid, Uuid}};
use serde::{Serialize, Deserialize};
use crate::{Grid, Tile, Pic};

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Tilemap {
    pub grid:Grid<Tile>,
    pub floor_texture:Pic,
    pub ceiling_texture:Pic
}

impl Component for Tilemap {
    fn type_id() -> Uuid {
        uuid!("e799fda3-1ef8-438f-a22b-ea4cfdc17941")
    }
}