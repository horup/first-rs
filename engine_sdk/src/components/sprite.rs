use glam::vec2;
use glam::{Vec3, Vec2};
use registry::uuid::{Uuid, uuid};
use serde::{Serialize, Deserialize};
use registry::Component;

use crate::Pic;

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum SpriteType {
    Facing,
    Wall,
    Floor
}

impl Default for SpriteType {
    fn default() -> Self {
        Self::Facing
    }
}

#[derive(Copy, Clone, Serialize, Deserialize, Default)]
pub struct Sprite {
    pub pos:Vec3,
    pub vel:Vec3,
    pub pic:Pic,
    pub opacity:Option<f32>,
    pub sprite_type:SpriteType,
    pub facing:f32,
    pub radius:f32,
    pub clips:bool,
    pub hidden:bool
}

impl Component for Sprite {
    fn type_id() -> Uuid {
        uuid!("f9e8d901-e8dd-483b-9666-6306df63ad01")
    }
}

impl Sprite {
    pub fn tile_index(&self) -> (i32, i32) {
        let index = self.pos.as_ivec3().truncate();
        index.into()
    }

    pub fn facing_as_vec2(&self) -> Vec2 {
        vec2(self.facing.cos(), self.facing.sin())
    }
}