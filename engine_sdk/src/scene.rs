use glam::Vec3;
use serde::{Serialize, Deserialize};
use slotmap::new_key_type;
use crate::{Grid, Sprite, Entities};

new_key_type! {pub struct SpriteId;}

#[derive(Default, Clone, Copy, Serialize, Deserialize)]
pub struct Cell {
    pub wall:Option<u32>
}

pub struct Scene<'a> {
    sprites:&'a Entities<SpriteId, Sprite>,
    pub ceiling_texture:u32,
    pub floor_texture:u32,
    grid:&'a Grid<Cell>
}

impl<'a> Scene<'a> {
    pub fn new(sprites:&'a Entities<SpriteId, Sprite>, grid:&'a Grid<Cell>) -> Self {
        Self {
            sprites,
            ceiling_texture: 0,
            floor_texture: 0,
            grid,
        }
    }

    pub fn sprites(&self) -> &'a Entities<SpriteId, Sprite> {
        self.sprites
    }

    pub fn grid(&self) -> &'a Grid<Cell> {
        self.grid
    }

    pub fn clip_move(&self, id:SpriteId, new_pos:Vec3) {
        if let Some(sprite) = self.sprites.get_mut(id) {
            sprite.pos = new_pos;
        }
    }
}

