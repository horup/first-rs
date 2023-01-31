
use engine_sdk::glam::{Mat4, Vec3};
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [f32;16]
}

impl CameraUniform {
    pub fn new_orth_screen(width:f32, height:f32) -> Self {
        let matrix = Mat4::orthographic_rh(0., width, -height, 0.0, 0.0, -1000.0);
        Self {
            view_proj:matrix.to_cols_array()
        }
    }

    pub fn new_fps(eye:Vec3, at:Vec3) -> Self {
        let eye = -Vec3::new(0.0, 2.0, 20.0);
        let matrix = Mat4::perspective_rh(2.0, 1.0, 0.0, 100.0) * Mat4::from_translation(eye);
        Self {
            view_proj:matrix.to_cols_array()
        }
    }
}