use std::ops::Range;

use engine_sdk::{glam::Vec2, DrawTextParams, DrawLineParams, DrawRectParams};
use lyon::{path::Path, lyon_tessellation::{StrokeTessellator, VertexBuffers, StrokeOptions, BuffersBuilder, StrokeVertexConstructor}, geom::{point, Box2D, euclid::Point2D}};
use wgpu::util::StagingBelt;
use crate::{Model, Graphics, Vertex, GraphicsContext};
use wgpu_glyph::{ab_glyph, GlyphBrushBuilder, Section, Text, GlyphBrush};

pub struct Canvas {
    pub geometry:Model,
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
            geometry:Model::new(&graphics.device),
            glyph_brush,
            staging_belt,
            draw_calls:Vec::new()
        }
    }

    pub fn prepare(&mut self) {
        self.geometry.clear();
        self.staging_belt.recall();
    }

    pub fn draw_lines(&mut self, mut lines:Vec<DrawLineParams>) {
        let start = self.geometry.indicies.len() as u32;
        for p in lines.drain(..) {
            let tolerance = 0.02;
            let mut builder = Path::builder();
            builder.begin(point(p.begin.x, -p.begin.y));
            builder.line_to(point(p.end.x, -p.end.y));
            builder.end(true);
            let line = builder.build();
    
            let mut stroke_tess = StrokeTessellator::new();
    
            let mut geometry: VertexBuffers<Vertex, u32> = VertexBuffers::new();
            stroke_tess
            .tessellate_path(
                &line,
                &StrokeOptions::tolerance(tolerance).with_line_width(p.line_width),
                &mut BuffersBuilder::new(&mut geometry, VertexConstructor),
            )
            .unwrap();
    
            let start = self.geometry.vertices.len() as u32;
            for i in geometry.indices.iter().rev() {
                self.geometry.indicies.push(start + i);
            }
            for v in geometry.vertices.iter() {
                let mut v = *v;
                v.color = p.color.into();
                self.geometry.vertices.push(v);
            }
            let end = self.geometry.vertices.len() as u32;
        }
        let end = self.geometry.indicies.len() as u32;
        self.push_draw_call(DrawCall::Geometry {
            range: start..end,
            diffuse_texture:None
        });
        
    }

    pub fn draw_text(&mut self, params:DrawTextParams) {
        self.push_draw_call(DrawCall::Text(params));
    }

    pub fn draw_rect(&mut self, params:DrawRectParams) {
        let px = params.pos.x;
        let py = -params.pos.y;
        let w = params.size.x;
        let h = params.size.y;
        let px2 = px + w;
        let py2 = py - h;
        let color = params.color.into();

        let model = &mut self.geometry;

        let vs = model.vertices.len() as u32;
        let start = model.indicies.len() as u32;

        model.vertices.push(Vertex {
            position: [px, py, 0.0],
            color,
            uv:[0.0, 0.0]
        });
        model.vertices.push(Vertex {
            position: [px, py2, 0.0],
            color,
            uv:[0.0, 1.0]
        });
        model.vertices.push(Vertex {
            position: [px2, py2, 0.0],
            color,
            uv:[1.0, 1.0]
        });
        model.vertices.push(Vertex {
            position: [px2, py, 0.0],
            color,
            uv:[1.0, 0.0]
        });

        model.indicies.push(vs + 0);
        model.indicies.push(vs + 1);
        model.indicies.push(vs + 2);
        model.indicies.push(vs + 0);
        model.indicies.push(vs + 2);
        model.indicies.push(vs + 3);

        let end = model.indicies.len() as u32;
        self.push_draw_call(DrawCall::Geometry {
            range: start..end,
            diffuse_texture:params.texture
        });
    }

    pub fn push_draw_call(&mut self, call:DrawCall) {
        match &call {
            DrawCall::Geometry { range, diffuse_texture } => {
                let diffuse_texture1 = diffuse_texture;
                let range1 = range;
                match self.draw_calls.last_mut() {
                    Some(last) => {
                        match last {
                            DrawCall::Geometry { range, diffuse_texture } => {
                                if diffuse_texture1 == diffuse_texture {
                                    range.end = range1.end;
                                } else {
                                    return self.draw_calls.push(call);
                                }
                            },
                            _=> return self.draw_calls.push(call)
                        }
                    },
                    None => return self.draw_calls.push(call),
                }
            },
            _=> {
                return self.draw_calls.push(call);
            }
        }
      
    }

    pub fn draw(&mut self, graphics:&mut GraphicsContext) {
        // write geometry to buffer
        self.geometry.write(graphics);
        let size = graphics.screen_size;

        // schedule draw calls
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
                },
                DrawCall::Geometry { range, diffuse_texture } => {
                    let mut texture = graphics.texture_white;
                    match diffuse_texture {
                        Some(id) => {
                            match graphics.textures.get(&id) {
                                Some(tex) => texture = tex,
                                None => texture = graphics.texture_missing,
                            }
                        },
                        None => {},
                    }
                   
                    self.geometry.draw_indexed(graphics, range, texture);
                }
            }
        }

    }

    pub fn finish(&mut self) {
        self.staging_belt.finish();
    }
}

pub enum DrawCall {
    Text(DrawTextParams),
    Geometry {
        range:Range<u32>,
        diffuse_texture:Option<u32>
    }
}

pub struct VertexConstructor;

impl StrokeVertexConstructor<Vertex> for VertexConstructor {
    fn new_vertex(&mut self, vertex: lyon::lyon_tessellation::StrokeVertex) -> Vertex {
        Vertex {
            position: [vertex.position().x, vertex.position().y, 0.0],
            color: [1.0, 0.0, 0.0, 1.0],
            uv:[0.0, 0.0]
        }
    }
}
