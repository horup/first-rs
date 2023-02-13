use glam::Vec3;
use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum SpriteType {
    Normal,
    Wall,
    Floor
}

impl Default for SpriteType {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Copy, Clone, Serialize, Deserialize, Default)]
pub struct Sprite {
    pub pos:Vec3,
    pub texture:u32,
    pub opacity:Option<f32>,
    pub sprite_type:SpriteType,
    pub facing:f32
}