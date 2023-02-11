use glam::Vec3;
use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Sprite {
    pub pos:Vec3,
    pub texture:u32,
    pub opacity:f32
}