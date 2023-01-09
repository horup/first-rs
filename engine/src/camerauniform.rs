
use engine_sdk::glam::{Mat4};
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
}