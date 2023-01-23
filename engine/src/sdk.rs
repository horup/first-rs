use std::rc::Rc;

use crate::{Engine, Vertex};
use engine_sdk::{
    self,
    glam::{Vec2, Vec3},
    image::DynamicImage,
    DrawRectParams, TextureInfo,
};
use winit::event::VirtualKeyCode;

impl engine_sdk::Engine for Engine {
    fn load_texture(&mut self, id: u32, image: &DynamicImage) {
        self.graphics.load_texture(id, image);
        self.textures.insert(
            id,
            TextureInfo {
                id,
                width: image.width() as f32,
                height: image.height() as f32,
                image: Rc::new(image.clone())
            },
        );
    }

    fn draw_scene(&mut self, _camera: &engine_sdk::Camera, scene: &engine_sdk::Scene) {
        return;
        let tex = 0;
        if let Some(model) = self.models.get_mut(&tex) {
            //let mut vertices = Vec::with_capacity(scene.sprites.len() * 6);
            model.vertices.clear();
            model.indicies.clear();
            for sprite in scene.sprites.iter() {
                if sprite.tex == tex {
                    let s = sprite.size / 2.0;
                    model.vertices.push(Vertex {
                        position: (sprite.pos + Vec3::new(-s, -s, 0.0)).into(),
                        color: [1.0, 1.0, 1.0, 1.0],
                        uv: [0.0, 0.0],
                    });
                    model.vertices.push(Vertex {
                        position: (sprite.pos + Vec3::new(s, s, 0.0)).into(),
                        color: [1.0, 1.0, 1.0, 1.0],
                        uv: [0.0, 0.0],
                    });
                    model.vertices.push(Vertex {
                        position: (sprite.pos + Vec3::new(-s, s, 0.0)).into(),
                        color: [1.0, 1.0, 1.0, 1.0],
                        uv: [0.0, 0.0],
                    });
                    model.vertices.push(Vertex {
                        position: (sprite.pos + Vec3::new(-s, -s, 0.0)).into(),
                        color: [1.0, 1.0, 1.0, 1.0],
                        uv: [0.0, 0.0],
                    });
                    model.vertices.push(Vertex {
                        position: (sprite.pos + Vec3::new(s, -s, 0.0)).into(),
                        color: [1.0, 1.0, 1.0, 1.0],
                        uv: [0.0, 0.0],
                    });
                    model.vertices.push(Vertex {
                        position: (sprite.pos + Vec3::new(s, s, 0.0)).into(),
                        color: [1.0, 1.0, 1.0, 1.0],
                        uv: [0.0, 0.0],
                    });
                }
            }
            // model.write(&self.graphics);
            // model.draw(&self.graphics);
        }
    }

    fn draw_rect(&mut self, params: DrawRectParams) {
        self.canvas.draw_rect(params);
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

    fn texture(&self, id: &u32) -> Option<engine_sdk::TextureInfo> {
        match self.textures.get(id) {
            Some(v) => Some(v.clone()),
            None => None,
        }
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

        return false;
    }

    fn keys_just_pressed(&self) -> &[VirtualKeyCode] {
        &self.input.keys_just_pressed
    }

    fn egui(&self) -> &egui::Context {
        &self.egui_ctx
    }

    fn dt(&self) -> f32 {
        return self.diagnostics.frame_time.as_millis() as f32 / 1000.0;
    }

    fn mouse_wheel_delta(&self) -> Vec2 {
        self.input.mouse_wheel_delta
    }

    fn textures(&self) -> Vec<TextureInfo> {
        let textures: Vec<TextureInfo> = self
            .textures
            .iter()
            .map(|(_, v)| {
                return v.clone();
            })
            .collect();
        textures
    }

    fn egui_texture(&mut self, id:&u32) -> Option<egui::TextureHandle> {
        if let Some(texture) = self.egui_textures.get(id) {
            return Some(texture.clone());
        }
        if let Some(texture) = self.textures.get(id) {
            let w = texture.image.width() as usize;
            let h = texture.image.height() as usize;
            let bytes = texture.image.to_rgba8().to_vec();
            let texture = self.egui_ctx.load_texture(format!("{}", id), egui::ColorImage::from_rgba_unmultiplied([w,h], &bytes), egui::TextureOptions::NEAREST);
            self.egui_textures.insert(*id, texture.clone());

            return Some(texture);
        }

        return None;
    }
}
