use glam::Vec3;
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
pub struct Camera {
    pub pos:Vec3,
    pub yaw:f32
}