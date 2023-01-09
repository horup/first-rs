use crate::{Engine, Model, Vertex};
use engine_sdk::{self, glam::{Vec3, Vec2}};

impl engine_sdk::Engine for Engine {
    fn define_texture(&mut self, id: u32, texture: String) {
        self.models.insert(id, Model::new(&self.graphics.device));
        dbg!(self.models.len());
    }

    fn draw_scene(&mut self, camera: &engine_sdk::Camera, scene: &engine_sdk::Scene) {
        let tex = 0;
        if let Some(model) = self.models.get_mut(&tex) {
            //let mut vertices = Vec::with_capacity(scene.sprites.len() * 6);
            model.vertices.clear();
            for sprite in scene.sprites.iter() {
                if sprite.tex == tex {
                    let s = sprite.size / 2.0;
                    model.vertices.push(Vertex {
                        position: (sprite.pos + Vec3::new(-s, -s, 0.0)).into(),
                        color: [1.0, 1.0, 1.0, 1.0],
                    });
                    model.vertices.push(Vertex {
                        position: (sprite.pos + Vec3::new(s, s, 0.0)).into(),
                        color: [1.0, 1.0, 1.0, 1.0],
                    });
                    model.vertices.push(Vertex {
                        position: (sprite.pos + Vec3::new(-s, s, 0.0)).into(),
                        color: [1.0, 1.0, 1.0, 1.0],
                    });
                    model.vertices.push(Vertex {
                        position: (sprite.pos + Vec3::new(-s, -s, 0.0)).into(),
                        color: [1.0, 1.0, 1.0, 1.0],
                    });
                    model.vertices.push(Vertex {
                        position: (sprite.pos + Vec3::new(s, -s, 0.0)).into(),
                        color: [1.0, 1.0, 1.0, 1.0],
                    });
                    model.vertices.push(Vertex {
                        position: (sprite.pos + Vec3::new(s, s, 0.0)).into(),
                        color: [1.0, 1.0, 1.0, 1.0],
                    });
                }
            }
            model.write(&self.graphics);
            model.draw(&self.graphics);
        }
    }

    fn frame_time(&self) -> std::time::Duration {
        self.diagnostics.frame_time
    }

    fn draw_rect(&mut self, px:f32, py:f32, w:f32, h:f32, color:engine_sdk::Color) {
        self.canvas.draw_rect(px, -py-h, w, h, color.into());
    }

    fn screen_size(&self) -> engine_sdk::glam::Vec2 {
        Vec2::new(self.graphics.screen_size.width as f32, self.graphics.screen_size.height as f32)
    }
}
