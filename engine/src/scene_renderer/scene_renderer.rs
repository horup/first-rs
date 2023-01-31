use std::{mem::{size_of, replace}, ops::Range};

use engine_sdk::{Camera, Scene, glam::{ivec2, IVec2, Vec3, vec3}, Cell};
use wgpu::{BufferDescriptor, BindGroup, Buffer, RenderPipeline};

use crate::{Graphics, CameraUniform, Vertex, Model, GraphicsContext};

pub struct SceneRenderer {
    camera_buffer:Buffer,
    camera_bind_group:BindGroup,
    render_pipeline:RenderPipeline,
    pub geometry:Model,
    draw_calls:Vec<DrawCall>
}

enum DrawCall {
    DrawWalls {
        texture:u32,
        range:Range<u32>
    }
}

impl SceneRenderer {
    pub fn new(graphics:&Graphics) -> Self {
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
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: graphics.config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,//Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None, // 1.
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None, // 5.
        });

        Self {
            geometry:Model::new(&graphics.device),
            render_pipeline,
            camera_buffer, 
            camera_bind_group,
            draw_calls:Vec::new()
        }
    }


    fn wall(&mut self, cell:&Cell, pos:IVec2, normal:IVec2) {
        if let Some(texture) = cell.wall {
            let color = [1.0, 1.0, 1.0, 1.0];
            let start_vertex = self.geometry.vertices.len() as u32;
            let start_index = self.geometry.indicies.len() as u32;

            let wall = [Vertex {
                position: [0.0, 0.0, 0.0],
                color: color,
                uv: [0.0, 1.0],
            }, Vertex {
                position: [1.0, 0.0, 0.0],
                color: color,
                uv: [1.0, 1.0],
            }, Vertex {
                position: [1.0, 1.0, 0.0],
                color: color,
                uv: [1.0, 0.0],
            },  Vertex {
                position: [0.0, 1.0, 0.0],
                color: color,
                uv: [0.0, 0.0],
            }];

            for mut v in wall {
                v.position[0] += pos.x as f32;
                v.position[2] += pos.y as f32;
                self.geometry.vertices.push(v);
            }

            self.geometry.indicies.push(start_vertex + 0);
            self.geometry.indicies.push(start_vertex + 1);
            self.geometry.indicies.push(start_vertex + 2);

            self.geometry.indicies.push(start_vertex + 0);
            self.geometry.indicies.push(start_vertex + 2);
            self.geometry.indicies.push(start_vertex + 3);




            let end_index = self.geometry.indicies.len() as u32;
            self.draw_calls.push(DrawCall::DrawWalls { texture, range: start_index..end_index });
        }
    }

    pub fn prepare(&mut self, graphics:&mut Graphics, camera:&Camera, scene:&Scene) {
        self.geometry.clear();

        // update camera
        let camera_uniform = CameraUniform::new_fps(vec3(0.0, 0.0, -1.0), vec3(0.0, 0.0, 0.0));//CameraUniform::new_orth_screen(10.0, 10.0);
        graphics.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[camera_uniform]),
        );

        // update grid
        let size = scene.grid.size();
        for y in 0..size {
            for x in 0..size {
                let x = x as i32;
                let y = y as i32;
                if let Some(tile) = scene.grid.get((x, y)) {
                    if tile.wall.is_none() {
                        let normals = [ivec2(0, 1), ivec2(0, -1), ivec2(1, 0), ivec2(-1, 0)];
                        for n in normals.iter() {
                            let p = ivec2(x, y) - *n;
                            if let Some(cell) = scene.grid.get((p.x, p.y)) {
                                self.wall(cell, p, *n);
                            }
                        }
                    }
                }
            }
        }

    }

    pub fn draw(&mut self, graphics:&mut GraphicsContext) {
        self.geometry.write(graphics);
        let draw_calls = replace(&mut self.draw_calls, Vec::new());
        for draw_call in draw_calls {
            match draw_call {
                DrawCall::DrawWalls { texture, range } => {
                    let mut render_pass = graphics.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("Render Pass"),
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &graphics.surface_view,
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
                    render_pass.set_bind_group(1, &graphics.texture_missing.texture_bind_group, &[]);
                    render_pass.set_index_buffer(self.geometry.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
                    render_pass.set_vertex_buffer(0, self.geometry.vertex_buffer.slice(..));
                    render_pass.draw_indexed(range, 0, 0..1);
                },
            }
        }

    }
}