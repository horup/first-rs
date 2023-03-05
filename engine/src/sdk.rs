use std::{rc::Rc};

use crate::{Engine};
use engine_sdk::{
    self,
    glam::{Vec2},
    image::DynamicImage,
    DrawRectParams, TextureAtlas, Event, LoadAtlasParams,
};
use winit::{event::VirtualKeyCode, window::CursorGrabMode};

impl engine_sdk::Engine for Engine {
    fn load_atlas(&mut self, id: u32, image: &DynamicImage, params:LoadAtlasParams) {
        self.graphics.load_texture(id, image, params.atlas);
        self.textures.insert(
            id,
            TextureAtlas::new(id, Rc::new(image.clone()), params.atlas, params.editor_props),
        );
    }

    fn draw_scene(&mut self, camera: &engine_sdk::Camera, scene: &mut engine_sdk::World) {
        self.scene_renderer.prepare(&mut self.graphics, camera, scene);
    }

    fn draw_rect(&mut self, params: DrawRectParams) {
        let atlas = self.graphics.get_atlas(params.texture);
        self.canvas.draw_rect(params, &atlas);
    }

    fn screen_size(&self) -> engine_sdk::glam::Vec2 {
        Vec2::new(
            self.graphics.config.width as f32,
            self.graphics.config.height as f32,
        )
    }

    fn draw_line(&mut self, params: engine_sdk::DrawLineParams) {
        self.canvas.draw_lines([params].into());
    }

    fn draw_text(&mut self, params: engine_sdk::DrawTextParams) {
        self.canvas.draw_text(params);
    }

    fn atlas(&self, id: &u32) -> Option<engine_sdk::TextureAtlas> {
        self.textures.get(id).cloned()
    }

    fn mouse_pos(&self) -> Vec2 {
        self.input.mouse_pos
    }

    fn mouse_down(&self, button: u8) -> bool {
        self.input.mouse_pressed[button as usize % 4]
    }

    fn key_down(&self, key_code: VirtualKeyCode) -> bool {
        if let Some(k) = self.input.keys_pressed.get(&key_code) {
            return *k;
        }

        false
    }

    fn keys_just_pressed(&self) -> &[VirtualKeyCode] {
        &self.input.keys_just_pressed
    }

    fn egui(&self) -> &egui::Context {
        &self.egui_ctx
    }

    fn dt(&self) -> f32 {
        self.diagnostics.frame_time.as_millis() as f32 / 1000.0
    }

    fn mouse_wheel_delta(&self) -> Vec2 {
        self.input.mouse_wheel_delta
    }

    fn atlases(&self) -> Vec<TextureAtlas> {
        let textures: Vec<TextureAtlas> = self
            .textures.values().cloned()
            .collect();
        textures
    }

    fn egui_texture(&mut self, id:&u32) -> Option<egui::TextureHandle> {
        if let Some(texture) = self.egui_textures.get(id) {
            return Some(texture.clone());
        }
        if let Some(texture) = self.textures.get(id) {
            let img = texture.image();
            let img = img.crop_imm(0, 0, texture.width(0), texture.height(0));
            let w = img.width() as usize;
            let h = img.height() as usize;
            let bytes = img.to_rgba8().to_vec();
            let texture = self.egui_ctx.load_texture(format!("{}", id), egui::ColorImage::from_rgba_unmultiplied([w,h], &bytes), egui::TextureOptions::NEAREST);
            self.egui_textures.insert(*id, texture.clone());

            return Some(texture);
        }

        None
    }

    fn push_event(&mut self, event:Event) {
        self.new_events.push(event);
    }

    fn set_cursor_visible(&mut self, visible:bool) {
        let window = self.window.borrow_mut();
        window.set_cursor_visible(visible);
        match visible {
            true => {
                let _ = window.set_cursor_grab(CursorGrabMode::None);
            },
            false => {
                let _ = window.set_cursor_grab(CursorGrabMode::Confined);
            },
        }
        
    }

    fn mouse_motion(&self) -> Vec2 {
        self.input.mouse_motion
    }
}
