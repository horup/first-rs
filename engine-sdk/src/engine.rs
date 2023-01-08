use std::time::{Instant, Duration};

use crate::{Camera, Scene, Color};
pub trait Engine {
    fn define_texture(&mut self, id:u32, texture:String);
    fn draw_scene(&mut self, camera:&Camera, scene:&Scene);
    fn frame_time(&self) -> Duration;
    fn draw_rect(&mut self, px:f32, py:f32, w:f32, h:f32, color:Color);
}
