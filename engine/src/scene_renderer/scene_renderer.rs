use std::{mem::{size_of, replace}, ops::Range};

use egui::epaint::ahash::{HashMap, HashMapExt};
use engine_sdk::{Camera, Scene, glam::{ivec2, IVec2, Vec3, vec3}, Cell};
use wgpu::{BufferDescriptor, BindGroup, Buffer, RenderPipeline, StencilState, DepthBiasState};

use crate::{Graphics, CameraUniform, Vertex, Model, GraphicsContext};

pub struct SceneRenderer {
    camera_buffer:Buffer,
    camera_bind_group:BindGroup,
    render_pipeline:RenderPipeline,
    pub geometry:Model,
    draw_calls:Vec<DrawCall>,
}

enum DrawCall {
    Clear {

    },
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
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: StencilState::default(),
                bias: DepthBiasState::default(),
            }), // 1.
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


    fn wall(&mut self, wall_texture:u32, pos:IVec2, normal:IVec2) {
        let color = [1.0, 1.0, 1.0, 1.0];
        let start_vertex = self.geometry.vertices.len() as u32;
        let start_index = self.geometry.indicies.len() as u32;

        let north = [[1.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 1.0]];
        let south = [[0.0, 1.0, 0.0], [1.0, 1.0, 0.0], [1.0, 1.0, 1.0], [0.0, 1.0, 1.0]];
        let east = [[0.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 1.0, 1.0], [0.0, 0.0, 1.0]];
        let west = [[1.0, 1.0, 0.0], [1.0, 0.0, 0.0], [1.0, 0.0, 1.0], [1.0, 1.0, 1.0]];

        //let walls = [north, south, west, east];
        let mut wall = &south;
        if normal.y > 0 {
            wall = &north;
        } else if normal.y < 0 {
            wall = &south;
        } else if normal.x < 0 {
            wall = &west;
        } else if normal.x > 0 {
            wall = &east;
        }

        let wall = [Vertex {
            position: wall[0],
            color: color,
            uv: [0.0, 1.0],
        }, Vertex {
            position: wall[1],
            color: color,
            uv: [1.0, 1.0],
        }, Vertex {
            position: wall[2],
            color: color,
            uv: [1.0, 0.0],
        },  Vertex {
            position: wall[3],
            color: color,
            uv: [0.0, 0.0],
        }];

        for mut v in wall {
            v.position[0] += pos.x as f32;
            v.position[1] += pos.y as f32;
            self.geometry.vertices.push(v);
        }

        self.geometry.indicies.push(start_vertex + 0);
        self.geometry.indicies.push(start_vertex + 1);
        self.geometry.indicies.push(start_vertex + 2);

        self.geometry.indicies.push(start_vertex + 0);
        self.geometry.indicies.push(start_vertex + 2);
        self.geometry.indicies.push(start_vertex + 3);

        let end_index = self.geometry.indicies.len() as u32;
        if let Some(DrawCall::DrawWalls { texture, range }) = self.draw_calls.last_mut() {
            if wall_texture == *texture {
                range.end = end_index;
                return;
            }
        }

        self.draw_calls.push(DrawCall::DrawWalls { texture: wall_texture, range: start_index..end_index });
            
    }

    pub fn prepare(&mut self, graphics:&mut Graphics, camera:&Camera, scene:&Scene) {
        self.geometry.clear();
        self.draw_calls.push(DrawCall::Clear {  });

        // update camera
        let camera_uniform = CameraUniform::new_scene_camera(camera, graphics.config.width as f32, graphics.config.height as f32);
        graphics.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[camera_uniform]),
        );

        // find textures in use
        let mut textures = HashMap::new();
        scene.grid.for_each(|cell, _| {
            if let Some(wall) = cell.wall {
                textures.insert(wall, ());
            }
        });
        let mut textures:Vec<u32> = textures.keys().map(|k|{*k}).collect();
        textures.sort();

        // once per texture, render walls that can be reached from a spot without a wall
        // i.e. dont render walls that are not reachable
        for texture in textures {
            scene.grid.for_each(|cell, (x,y)| {
                if cell.wall.is_none() {
                    let directions = [ivec2(0, 1), ivec2(0, -1), ivec2(1, 0), ivec2(-1, 0)];
                    for n in directions.iter() {
                        let p = ivec2(x, y) - *n;
                        if let Some(cell) = scene.grid.get((p.x, p.y)) {
                            if let Some(wall_texture) = cell.wall {
                                if wall_texture == texture {
                                    self.wall(wall_texture, p, -*n);
                                }
                            }
                        }
                    }
                }
            });
        }
    }

    pub fn draw(&mut self, graphics:&mut GraphicsContext) {
        self.geometry.write(graphics);
        let draw_calls = replace(&mut self.draw_calls, Vec::new());
        for draw_call in draw_calls {
            match draw_call {
                DrawCall::DrawWalls { texture, range } => {
                    let texture = &graphics.texture(texture).clone().texture_bind_group;
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
                        depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                            view: &graphics.texture_depth.texture_view,
                            depth_ops: Some(wgpu::Operations {
                                load: wgpu::LoadOp::Load,
                                store:true
                            }),
                            stencil_ops: None
                        }),
                    });

                    render_pass.set_pipeline(&self.render_pipeline);
                    render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
                    render_pass.set_bind_group(1, texture, &[]);
                    render_pass.set_index_buffer(self.geometry.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
                    render_pass.set_vertex_buffer(0, self.geometry.vertex_buffer.slice(..));
                    render_pass.draw_indexed(range, 0, 0..1);
                },
                DrawCall::Clear {  } => {
                    let render_pass = graphics.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("Render Pass"),
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &graphics.surface_view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }),
                                store: true,
                            },
                        })],
                        depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                            view: &graphics.texture_depth.texture_view,
                            depth_ops: Some(wgpu::Operations {
                                load: wgpu::LoadOp::Clear(1.0),
                                store:true
                            }),
                            stencil_ops: None
                        }),
                    });

                   /* render_pass.set_pipeline(&self.render_pipeline);
                    render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
                    render_pass.set_bind_group(1, texture, &[]);
                    render_pass.set_index_buffer(self.geometry.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
                    render_pass.set_vertex_buffer(0, self.geometry.vertex_buffer.slice(..));
                    render_pass.draw_indexed(range, 0, 0..1);*/
                },
            }
        }

    }
}