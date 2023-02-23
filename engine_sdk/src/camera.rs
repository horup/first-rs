use glam::{Vec3, vec3};
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize, Clone, Copy)]
pub struct Camera {
    pub pos:Vec3,
    pub facing:f32
}

impl Camera {
  
    /// Calculates the forward vector of the camera body, ignoring pitch
    pub fn forward_body(&self) -> Vec3 {
        vec3(self.facing.cos(), self.facing.sin(), 0.0)
    }

    /// Calculate the left vector of the camera body
    pub fn left(&self) -> Vec3 {
        let forward = self.forward_body();
        -vec3(-forward.y, forward.x, 0.0)
    }
}