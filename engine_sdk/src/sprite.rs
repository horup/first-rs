use glam::Vec3;
use serde::{Serialize, Deserialize};

#[derive(Default, Copy, Clone, Serialize, Deserialize)]
pub struct Sprite {
    pub pos:Vec3,
    pub texture:u32
}