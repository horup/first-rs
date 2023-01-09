use wgpu;
use engine_sdk::glam::{self, Mat4};
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [f32;16]
}

impl CameraUniform {
    pub fn new_orth_screen(mut width:f32, mut height:f32) -> Self {
        let aspect = width / height;
        height = aspect * width;
        let matrix = Mat4::orthographic_rh(0.0, width, -height, 0.0, 0.0, -1000.0);
        //let matrix = Mat4::IDENTITY;
        Self {
            view_proj:matrix.to_cols_array()
        }
    }
}