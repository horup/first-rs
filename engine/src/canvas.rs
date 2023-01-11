use engine_sdk::{glam::Vec2, DrawTextParams};
use lyon::{path::Path, lyon_tessellation::{StrokeTessellator, VertexBuffers, StrokeOptions, BuffersBuilder, StrokeVertexConstructor}, geom::point};
use wgpu::util::StagingBelt;
use crate::{Model, Graphics, Vertex, GraphicsContext};
use wgpu_glyph::{ab_glyph, GlyphBrushBuilder, Section, Text, GlyphBrush};

pub struct Canvas {
    pub model:Model,
    pub glyph_brush:GlyphBrush<()>,
    pub staging_belt:StagingBelt,
    pub draw_calls:Vec<DrawCall>
}

impl Canvas {
    pub fn new(graphics:&Graphics) -> Self {
        let staging_belt = wgpu::util::StagingBelt::new(1024);
        let inconsolata = ab_glyph::FontArc::try_from_slice(include_bytes!("../fonts/joystix_monospace.ttf")).unwrap();
        let glyph_brush = GlyphBrushBuilder::using_font(inconsolata)
            .build(&graphics.device, wgpu::TextureFormat::Bgra8UnormSrgb);
        Self {
            model:Model::new(&graphics.device),
            glyph_brush,
            staging_belt,
            draw_calls:Vec::new()
        }
    }

    pub fn prepare(&mut self) {
        self.model.vertices.clear();
        self.model.indicies.clear();
        self.staging_belt.recall();
    }

    pub fn draw_line(&mut self, begin:Vec2, end:Vec2, color: [f32;4], line_width:f32) {
        let tolerance = 0.02;
        let mut builder = Path::builder();
        builder.begin(point(begin.x, -begin.y));
        builder.line_to(point(end.x, -end.y));
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
    }

    pub fn draw_text(&mut self, params:DrawTextParams) {
       /* self.glyph_brush.queue(Section {
            screen_position: (30.0, 30.0),
            bounds: (self.width as f32, size.height as f32),
            text: vec![Text::new("Hello wgpu_glyph!")
                .with_color([0.0, 0.0, 0.0, 1.0])
                .with_scale(40.0)],
            ..Section::default()
        });*/

        self.draw_calls.push(DrawCall::Text(params));
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

    pub fn draw(&mut self, graphics:&mut GraphicsContext) {
        let size = graphics.screen_size;
        self.model.write(graphics);
        self.model.draw(graphics);
       // self.model.write(graphics);
       // self.model.draw(graphics);

        let draw_calls = self.draw_calls.drain(..);
        for draw_call in draw_calls {
            match draw_call {
                DrawCall::Text(params) => {
                    self.glyph_brush.queue(Section {
                        screen_position: params.screen_pos.into(),
                      //  bounds: (size.width as f32, size.height as f32),
                        text: vec![Text::new(&params.text)
                            .with_color::<[f32;4]>(params.color.into())
                            .with_scale(params.scale)],
                        ..Section::default()
                    });
                },
            }
        }

        self.draw_glyph(graphics);
    }

    fn draw_glyph(&mut self, graphics:&mut GraphicsContext) {
       /* let output = graphics.surface.get_current_texture().unwrap();
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());*/
        let size = graphics.screen_size;
        self.glyph_brush.draw_queued(
            &graphics.device, 
            &mut self.staging_belt, 
            &mut graphics.encoder, 
            graphics.surface_view, 
            size.width, 
            size.height)
            .unwrap();

        self.staging_belt.finish();
    }

    pub fn finish(&mut self) {
        self.staging_belt.finish();
    }
}

pub enum DrawCall {
    Text(DrawTextParams)
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
