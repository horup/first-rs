use crate::{Camera, Scene};
pub trait Engine {
    fn define_texture(&mut self, id:u32, texture:String);
    fn draw_scene(&mut self, camera:&Camera, scene:&Scene);
    
}
