use std::{ops::Range, mem::{size_of}};

use engine_sdk::{DrawTextParams, DrawLineParams, DrawRectParams, Atlas};
use lyon::{path::Path, lyon_tessellation::{StrokeTessellator, VertexBuffers, StrokeOptions, BuffersBuilder, StrokeVertexConstructor}, geom::{point}};
use wgpu::{util::{StagingBelt}, RenderPipeline, BindGroup, Buffer, BufferDescriptor};
use crate::{Model, Graphics, Vertex, GraphicsContext, CameraUniform, Texture};
use wgpu_glyph::{ab_glyph, GlyphBrushBuilder, Section, Text, GlyphBrush, Layout};

pub struct Canvas {
    pub camera_buffer:Buffer,
    pub camera_bind_group:BindGroup,
    pub geometry:Model,
    pub glyph_brush:GlyphBrush<()>,
    pub staging_belt:StagingBelt,
    pub draw_calls:Vec<DrawCall>,
    pub render_pipeline:RenderPipeline
}

impl Canvas {
    pub fn new(graphics:&Graphics) -> Self {
        let staging_belt = wgpu::util::StagingBelt::new(1024);
        let inconsolata = ab_glyph::FontArc::try_from_slice(include_bytes!("./joystix_monospace.ttf")).unwrap();
        let glyph_brush = GlyphBrushBuilder::using_font(inconsolata)
            .build(&graphics.device, graphics.render_format);

        
        let camera_bind_group_layout =
        graphics.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("camera_bind_group_layout"),
        });

        let camera_buffer = graphics.device.create_buffer(&BufferDescriptor { label: "None".into(), size: size_of::<CameraUniform>() as u64, usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false });
        let camera_bind_group = graphics.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource:  camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        let texture_bind_group_layout =
        graphics.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    // This should match the filterable field of the
                    // corresponding Texture entry above.
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("texture_bind_group_layout"),
        });

        let shader = graphics.device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));
        let render_pipeline_layout =
            graphics.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&camera_bind_group_layout, &texture_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = graphics.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",     // 1.
                buffers: &[Vertex::desc()], // 2.
            },
            fragment: Some(wgpu::FragmentState {
                // 3.
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    // 4.
                    format: graphics.config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // 2.
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None, // 1.
            multisample: wgpu::MultisampleState {
                count: 1,                         // 2.
                mask: !0,                         // 3.
                alpha_to_coverage_enabled: false, // 4.
            },
            multiview: None, // 5.
        });

        Self {
            camera_buffer,
            camera_bind_group,
            render_pipeline,
            geometry:Model::new(&graphics.device),
            glyph_brush,
            staging_belt,
            draw_calls:Vec::new()
        }
    }

    pub fn prepare(&mut self) {
        self.geometry.clear();
        self.staging_belt.recall();
        self.draw_calls.clear();
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

    pub fn draw_rect(&mut self, params:DrawRectParams, atlas:&Atlas) {
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
        let u = atlas.u(params.atlas_index as u16);
        let v = atlas.v(params.atlas_index as u16);
        model.vertices.push(Vertex {
            position: [px, py, 0.0],
            color,
            uv:[u[0], v[0]]
        });
        model.vertices.push(Vertex {
            position: [px, py2, 0.0],
            color,
            uv:[u[0], v[1]]
        });
        model.vertices.push(Vertex {
            position: [px2, py2, 0.0],
            color,
            uv:[u[1], v[1]]
        });
        model.vertices.push(Vertex {
            position: [px2, py, 0.0],
            color,
            uv:[u[1], v[0]]
        });

        model.indicies.push(vs);
        model.indicies.push(vs + 1);
        model.indicies.push(vs + 2);
        model.indicies.push(vs);
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
                                    self.draw_calls.push(call)
                                }
                            },
                            _=> self.draw_calls.push(call)
                        }
                    },
                    None => self.draw_calls.push(call),
                }
            },
            _=> {
                self.draw_calls.push(call)
            }
        }
      
    }

    pub fn draw_geometry(&mut self, graphics:&mut GraphicsContext, diffuse_texture:&Texture, indicies:Range<u32>) {
        if self.geometry.index_buffer.size() == 0 || self.geometry.indicies.is_empty() {
            return;
        }

        {
            let mut render_pass = graphics.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: graphics.surface_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            render_pass.set_bind_group(1, &diffuse_texture.texture_bind_group, &[]);
            render_pass.set_index_buffer(self.geometry.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.set_vertex_buffer(0, self.geometry.vertex_buffer.slice(..));
            render_pass.draw_indexed(indicies, 0, 0..1);
        }
    }

    pub fn draw(&mut self, graphics:&mut GraphicsContext) {
        // write geometry to buffer
        self.geometry.write(graphics);

        // update camera
        let camera_uniform = CameraUniform::new_orth_screen(graphics.screen_size.width as f32, graphics.screen_size.height as f32);
        graphics.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[camera_uniform]),
        );

        // schedule draw calls
        let draw_calls = std::mem::take(&mut self.draw_calls);
        for draw_call in draw_calls {
            match draw_call {
                DrawCall::Text(params) => {
                    let h_align = match params.horizontal_align {
                        engine_sdk::HorizontalAlign::Left => wgpu_glyph::HorizontalAlign::Left,
                        engine_sdk::HorizontalAlign::Center => wgpu_glyph::HorizontalAlign::Center,
                        engine_sdk::HorizontalAlign::Right => wgpu_glyph::HorizontalAlign::Right,
                    };
                    let v_align = match params.vertical_align {
                        engine_sdk::VerticalAlign::Top => wgpu_glyph::VerticalAlign::Top,
                        engine_sdk::VerticalAlign::Center => wgpu_glyph::VerticalAlign::Center,
                        engine_sdk::VerticalAlign::Bottom => wgpu_glyph::VerticalAlign::Bottom,
                    };
                    self.glyph_brush.queue(Section {
                        screen_position: params.screen_pos.into(),
                      //  bounds: (size.width as f32, size.height as f32),
                        text: vec![Text::new(&params.text)
                            .with_color::<[f32;4]>(params.color.into())
                            .with_scale(params.scale)],
                            layout:Layout::SingleLine { line_breaker: wgpu_glyph::BuiltInLineBreaker::UnicodeLineBreaker, h_align, v_align },
                        ..Section::default()
                    });
                    let size = graphics.screen_size;
                    self.glyph_brush.draw_queued(
                        graphics.device, 
                        &mut self.staging_belt, 
                        graphics.encoder, 
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
                   
                    self.draw_geometry(graphics, texture, range);
                }
            }
        }
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
