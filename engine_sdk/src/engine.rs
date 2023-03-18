use glam::Vec2;
use image::DynamicImage;
use winit::{event::VirtualKeyCode};

use crate::{Camera, World, Color, Event, Atlas, TextureAtlas, EditorProps};

pub trait Engine {
    fn egui(&self) -> &egui::Context;
    fn egui_texture(&mut self, id:&u32) -> Option<egui::TextureHandle>;
    fn load_atlas(&mut self, id:u32, image:&DynamicImage, params:LoadAtlasParams);
    fn atlas(&self, id:&u32) -> Option<TextureAtlas>;
    fn atlases(&self) -> Vec<TextureAtlas>;
    fn draw_scene(&mut self, camera:&Camera, scene:&mut World);
    fn dt(&self) -> f32;
    fn draw_rect(&mut self, params:DrawRectParams);
    fn draw_line(&mut self, params:DrawLineParams);
    fn screen_size(&self) -> Vec2;
    fn draw_text(&mut self, params:DrawTextParams);
    fn mouse_pos(&self) -> Vec2;
    fn mouse_down(&self, button:u8) -> bool;
    fn mouse_wheel_delta(&self) -> Vec2;
    fn mouse_motion(&self) -> Vec2;
    fn key_down(&self, key_code:VirtualKeyCode) -> bool;
    fn keys_just_pressed(&self) -> &[VirtualKeyCode];
    fn push_event(&mut self, event:Event);
    fn key_just_pressed(&self, key_code:VirtualKeyCode) -> bool {
        self.keys_just_pressed().iter().any(|kc| kc == &key_code)
    }
    fn set_cursor_visible(&mut self, visible:bool);
    fn cursor_visible(&self) -> bool;
}


#[derive(Clone, Debug, Default)]
pub struct LoadAtlasParams {
    pub atlas:Atlas,
    pub editor_props:EditorProps
}

#[derive(Clone, Debug, Default)]
pub struct DrawRectParams {
    pub pos:Vec2,
    pub size:Vec2,
    pub color:Color,
    pub texture:Option<u32>,
    pub atlas_index:f32
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
