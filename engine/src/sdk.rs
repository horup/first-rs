use crate::{Engine, Model, Vertex};
use engine_sdk::{self, glam::{Vec3, Vec2}};
use lyon::{path::{Path}, geom::point, lyon_tessellation::{StrokeTessellator, StrokeOptions, BuffersBuilder, VertexBuffers, StrokeVertexConstructor}};

impl engine_sdk::Engine for Engine {
    fn define_texture(&mut self, id: u32, _texture: String) {
        self.models.insert(id, Model::new(&self.graphics.device));
        dbg!(self.models.len());
    }

    fn draw_scene(&mut self, _camera: &engine_sdk::Camera, scene: &engine_sdk::Scene) {
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

    fn draw_line(&mut self, params:engine_sdk::DrawLineParams) {
        let tolerance = 0.02;
        let mut builder = Path::builder();
        builder.begin(point(params.begin.x, params.begin.y));
        builder.line_to(point(params.end.x, params.end.y));
        builder.end(true);
        let line = builder.build();

        let mut stroke_tess = StrokeTessellator::new();

        let mut geometry: VertexBuffers<Vertex, u16> = VertexBuffers::new();
        stroke_tess
        .tessellate_path(
            &line,
            &StrokeOptions::tolerance(tolerance),
            &mut BuffersBuilder::new(&mut geometry, VertexConstructor),
        )
        .unwrap();


        // https://github.com/nical/lyon/blob/0367e5a6cf1b8658a29041215c3903d865863d41/examples/wgpu/src/main.rs#L723

       // let stroke_range = fill_range.end..(geometry.indices.len() as u32);
    }
}


pub struct VertexConstructor;

impl StrokeVertexConstructor<Vertex> for VertexConstructor {
    fn new_vertex(&mut self, vertex: lyon::lyon_tessellation::StrokeVertex) -> Vertex {
        Vertex {
            position: [vertex.position().x, vertex.position().y, 0.0],
            color: [1.0, 0.0, 0.0, 1.0],
        }
    }
}
