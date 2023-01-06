use crate::{Engine, Model, Vertex};
use engine_sdk::{self, glam::Vec3};

impl engine_sdk::Engine for Engine {
    fn define_texture(&mut self, id:u32, texture:String) {
        self.models.insert(id, Model::new(&self.graphics.device));
        dbg!(self.models.len());
    }

    fn draw_scene(&mut self, camera:&engine_sdk::Camera, scene:&engine_sdk::Scene) {
        let tex = 0;
        if let Some(model) = self.models.get_mut(&tex) {
            let mut vertices = Vec::with_capacity(scene.sprites.len() * 6);
            for sprite in scene.sprites.iter() {
                if sprite.tex == tex {
                    let s = 0.01;
                    vertices.push(Vertex {
                        position: (sprite.pos + Vec3::new(-s, -s, 0.0)).into(),
                        color: [1.0, 1.0, 1.0, 1.0],
                    });
                    vertices.push(Vertex {
                        position: (sprite.pos + Vec3::new(s, s, 0.0)).into(),
                        color: [1.0, 1.0, 1.0, 1.0],
                    });
                    vertices.push(Vertex {
                        position: (sprite.pos + Vec3::new(-s, s, 0.0)).into(),
                        color: [1.0, 1.0, 1.0, 1.0],
                    });
                    vertices.push(Vertex {
                        position: (sprite.pos + Vec3::new(-s, -s, 0.0)).into(),
                        color: [1.0, 1.0, 1.0, 1.0],
                    });
                    vertices.push(Vertex {
                        position: (sprite.pos + Vec3::new(s, -s, 0.0)).into(),
                        color: [1.0, 1.0, 1.0, 1.0],
                    });
                    vertices.push(Vertex {
                        position: (sprite.pos + Vec3::new(s, s, 0.0)).into(),
                        color: [1.0, 1.0, 1.0, 1.0],
                    });
                }
            }
            model.set_vertices(&self.graphics.device, &self.graphics.queue, &vertices);
            model.draw(&self.graphics);
        }
    }

    fn frame_time(&self) -> std::time::Duration {
        self.diagnostics.frame_time
    }
}