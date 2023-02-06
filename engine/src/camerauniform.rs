
use std::f32::consts::PI;

use engine_sdk::{glam::{Mat4, Vec3, vec3}, Camera};
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

    pub fn new_scene_camera(camera:&Camera, width:f32, height:f32) -> Self {
        let aspect = width / height;
        let translate = -camera.pos;//Vec3::new(8.0, 8.0, 1.5);
        let flip_y = Mat4::from_scale(vec3(1.0, -1.0, 1.0));
        let rot_x = Mat4::from_rotation_x(-PI / 2.0);
        let rot_y = Mat4::from_rotation_y(PI / 2.0 + camera.yaw);
        let matrix = Mat4::perspective_rh(PI / 2.0, aspect, 0.0, 1.0) * rot_y * rot_x * flip_y * Mat4::from_translation(translate);
        Self {
            view_proj:matrix.to_cols_array()
        }
    }
}