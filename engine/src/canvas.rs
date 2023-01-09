use wgpu::{Device, Color};

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
    }

    pub fn draw_rect(&mut self, px:f32, py:f32, w:f32, h:f32, color: [f32;4]) {
        let model = &mut self.model;
        let px2 = px + w;
        let py2 = py + h;
        
        model.vertices.push(Vertex {
            position: [px, py, 0.0],
            color
        });
        model.vertices.push(Vertex {
            position: [px2, py2, 0.0],
            color
        });
        model.vertices.push(Vertex {
            position: [px, py2, 0.0],
            color
        });
        model.vertices.push(Vertex {
            position: [px, py, 0.0],
            color
        });
        model.vertices.push(Vertex {
            position: [px2, py, 0.0],
            color
        });
        model.vertices.push(Vertex {
            position: [px2, py2, 0.0],
            color
        });
    }

    pub fn draw(&mut self, graphics:&Graphics) {
        self.model.write(graphics);
        self.model.draw(graphics);
    }
}