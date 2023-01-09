use std::time::{Duration};

use glam::Vec2;

use crate::{Camera, Scene, Color};

#[derive(Clone, Copy, Debug, Default)]
pub struct DrawLineParams {
    pub begin:Vec2,
    pub end:Vec2
}

pub trait Engine {
    fn define_texture(&mut self, id:u32, texture:String);
    fn draw_scene(&mut self, camera:&Camera, scene:&Scene);
    fn frame_time(&self) -> Duration;

    /**
    On the screen with y axis pointing downards
    */
    fn draw_rect(&mut self, px:f32, py:f32, w:f32, h:f32, color:Color);

    /**
    On the screen with y axis pointing downards
    */
    fn draw_line(&mut self, params:DrawLineParams);
    fn screen_size(&self) -> Vec2;
}
