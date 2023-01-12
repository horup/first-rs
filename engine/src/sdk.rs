use crate::{Engine, Model, Vertex};
use engine_sdk::{self, glam::{Vec3, Vec2}, DrawRectParams};
use lyon::{path::{Path}, geom::point, lyon_tessellation::{StrokeTessellator, StrokeOptions, BuffersBuilder, VertexBuffers, StrokeVertexConstructor}};

impl engine_sdk::Engine for Engine {
    fn define_texture(&mut self, id: u32, _texture: String) {
        self.models.insert(id, Model::new(&self.graphics.device));
        dbg!(self.models.len());
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
           // model.write(&self.graphics);
           // model.draw(&self.graphics);
        }
    }

    fn frame_time(&self) -> std::time::Duration {
        self.diagnostics.frame_time
    }

    fn draw_rect(&mut self, params:DrawRectParams) {
        self.canvas.draw_rect(params);
    }

    fn screen_size(&self) -> engine_sdk::glam::Vec2 {
        Vec2::new(self.graphics.screen_size.width as f32, self.graphics.screen_size.height as f32)
    }

    fn draw_line(&mut self, mut params:engine_sdk::DrawLineParams) {
        //self.canvas.draw_line(params.begin, params.end, params.color.into(), params.line_width);
        self.canvas.draw_lines([params].into());
    }

    fn draw_text(&mut self, params:engine_sdk::DrawTextParams) {
        self.canvas.draw_text(params);
    }
}

