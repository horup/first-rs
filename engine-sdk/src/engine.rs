use std::time::{Duration};

use glam::Vec2;
use image::DynamicImage;

use crate::{Camera, Scene, Color};

pub trait Engine {
    fn load_texture(&mut self, id:u32, image:&DynamicImage);
    fn draw_scene(&mut self, camera:&Camera, scene:&Scene);
    fn frame_time(&self) -> Duration;

    /**
    On the screen with y axis pointing downards
    */
    fn draw_rect(&mut self, params:DrawRectParams);

    /**
    On the screen with y axis pointing downards
    */
    fn draw_line(&mut self, params:DrawLineParams);
    fn screen_size(&self) -> Vec2;

    fn draw_text(&mut self, params:DrawTextParams);
}


#[derive(Clone, Debug, Default)]
pub struct DrawRectParams {
    pub pos:Vec2,
    pub size:Vec2,
    pub color:Color
}

#[derive(Clone, Debug, Default)]
pub struct DrawTextParams {
    pub screen_pos:Vec2,
    pub text:String,
    pub scale:f32,
    pub color:Color
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DrawLineParams {
    pub begin:Vec2,
    pub end:Vec2,
    pub line_width:f32,
    pub color:Color
}
