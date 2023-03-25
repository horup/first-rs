use std::ops::{Deref, DerefMut};

use serde::{Serialize, Deserialize};
use world::Singleton;
use crate::{Grid, Tile};

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Tilemap {
    pub grid:Grid<Tile>,
    pub floor_texture:u32,
    pub ceiling_texture:u32
}

impl Singleton for Tilemap {
    fn id() -> world::SingletonId {
        crate::TILEMAP_ID
    }
}