

use engine_sdk::glam::Vec2;
use lyon::{path::Path, lyon_tessellation::{StrokeTessellator, VertexBuffers, StrokeOptions, BuffersBuilder, StrokeVertexConstructor}, geom::point};

use crate::{Model, Graphics, Vertex};

pub struct Canvas {
    pub model:Model
}

impl Canvas {
    pub fn new(graphics:&Graphics) -> Self {
        Self {
            model:Model::new(&graphics.device)
        }
    }

    pub fn clear(&mut self) {
        self.model.vertices.clear();
        self.model.indicies.clear();
    }

    pub fn draw_line(&mut self, begin:Vec2, end:Vec2, color: [f32;4], line_width:f32) {
        let tolerance = 0.02;
        let mut builder = Path::builder();
        builder.begin(point(begin.x, begin.y));
        builder.line_to(point(end.x, end.y));
        builder.end(true);
        let line = builder.build();

        let mut stroke_tess = StrokeTessellator::new();

        let mut geometry: VertexBuffers<Vertex, u32> = VertexBuffers::new();
        stroke_tess
        .tessellate_path(
            &line,
            &StrokeOptions::tolerance(tolerance).with_line_width(line_width),
            &mut BuffersBuilder::new(&mut geometry, VertexConstructor),
        )
        .unwrap();

        let start = self.model.vertices.len() as u32;
        for i in geometry.indices.iter().rev() {
            self.model.indicies.push(start + i);
        }
        for v in geometry.vertices.iter() {
            let mut v = *v;
            v.color = color;
            self.model.vertices.push(v);
        }

        // FIX winding
        
        // https://github.com/nical/lyon/blob/0367e5a6cf1b8658a29041215c3903d865863d41/examples/wgpu/src/main.rs#L723

       // let stroke_range = fill_range.end..(geometry.indices.len() as u32);
    }

    pub fn draw_rect(&mut self, px:f32, py:f32, w:f32, h:f32, color: [f32;4]) {
        let model = &mut self.model;
        let px2 = px + w;
        let py2 = py + h;
        
        model.vertices.push(Vertex {
            position: [px, py, 0.0],
            color
        });
        model.indicies.push(model.indicies.len() as u32);
        model.vertices.push(Vertex {
            position: [px2, py2, 0.0],
            color
        });
        model.indicies.push(model.indicies.len() as u32);
        model.vertices.push(Vertex {
            position: [px, py2, 0.0],
            color
        });
        model.indicies.push(model.indicies.len() as u32);
        model.vertices.push(Vertex {
            position: [px, py, 0.0],
            color
        });
        model.indicies.push(model.indicies.len() as u32);
        model.vertices.push(Vertex {
            position: [px2, py, 0.0],
            color
        });
        model.indicies.push(model.indicies.len() as u32);
        model.vertices.push(Vertex {
            position: [px2, py2, 0.0],
            color
        });
        model.indicies.push(model.indicies.len() as u32);
    }

    pub fn draw(&mut self, graphics:&Graphics) {
        self.model.write(graphics);
        self.model.draw(graphics);
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
