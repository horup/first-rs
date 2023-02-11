use std::{mem::{size_of, replace}, ops::Range, f32::consts::PI, cmp::Ordering};

use egui::epaint::ahash::{HashMap, HashMapExt};
use engine_sdk::{Camera, Scene, glam::{ivec2, IVec2, Vec3, vec3, Mat4, Mat3}, Cell, Sprite};
use wgpu::{BufferDescriptor, BindGroup, Buffer, RenderPipeline, StencilState, DepthBiasState};

use crate::{Graphics, CameraUniform, Vertex, Model, GraphicsContext};

pub struct SceneRenderer {
    camera_buffer:Buffer,
    camera_bind_group:BindGroup,
    geometry_render_pipeline:RenderPipeline,
    sprite_render_pipeline:RenderPipeline,
    opaque_sprites:Vec<usize>,
    translucent_sprites:Vec<usize>,
    geometry:Model,
    sprites:Model,
    draw_calls:Vec<DrawCall>,
}

enum DrawCall {
    Clear {

    },
    DrawGeometry {
        texture:u32,
        range:Range<u32>
    },
    DrawSprite {
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

        let geometry_render_pipeline = graphics.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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

        let sprite_render_pipeline = graphics.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
                cull_mode: None,
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
            sprites:Model::new(&graphics.device),
            geometry:Model::new(&graphics.device),
            geometry_render_pipeline,
            sprite_render_pipeline,
            camera_buffer, 
            camera_bind_group,
            draw_calls:Vec::new(),
            opaque_sprites: Vec::new(),
            translucent_sprites: Vec::new(),
        }
    }


    fn ceiling(&mut self, ceiling_texture:u32, pos:IVec2) {
        let color = [1.0, 1.0, 1.0, 1.0];
        let start_vertex = self.geometry.vertices.len() as u32;
        let start_index = self.geometry.indicies.len() as u32;
        let ceiling = [[1.0, 0.0, 1.0], [1.0, 1.0, 1.0], [0.0, 1.0, 1.0], [0.0, 0.0, 1.0]];
        let floor = [Vertex {
            position: ceiling[0],
            color: color,
            uv: [0.0, 1.0],
        }, Vertex {
            position: ceiling[1],
            color: color,
            uv: [1.0, 1.0],
        }, Vertex {
            position: ceiling[2],
            color: color,
            uv: [1.0, 0.0],
        },  Vertex {
            position: ceiling[3],
            color: color,
            uv: [0.0, 0.0],
        }];

        for mut v in floor {
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
        if let Some(DrawCall::DrawGeometry { texture, range }) = self.draw_calls.last_mut() {
            if ceiling_texture == *texture {
                range.end = end_index;
                return;
            }
        }

        self.draw_calls.push(DrawCall::DrawGeometry { texture: ceiling_texture, range: start_index..end_index });
    }
    fn floor(&mut self, floor_texture:u32, pos:IVec2) {
        let color = [1.0, 1.0, 1.0, 1.0];
        let start_vertex = self.geometry.vertices.len() as u32;
        let start_index = self.geometry.indicies.len() as u32;

        let floor = [[0.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0, 0.0], [1.0, 0.0, 0.0]];
        let floor = [Vertex {
            position: floor[0],
            color: color,
            uv: [0.0, 1.0],
        }, Vertex {
            position: floor[1],
            color: color,
            uv: [1.0, 1.0],
        }, Vertex {
            position: floor[2],
            color: color,
            uv: [1.0, 0.0],
        },  Vertex {
            position: floor[3],
            color: color,
            uv: [0.0, 0.0],
        }];

        for mut v in floor {
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
        if let Some(DrawCall::DrawGeometry { texture, range }) = self.draw_calls.last_mut() {
            if floor_texture == *texture {
                range.end = end_index;
                return;
            }
        }

        self.draw_calls.push(DrawCall::DrawGeometry { texture: floor_texture, range: start_index..end_index });
    }

    fn wall(&mut self, wall_texture:u32, pos:IVec2, normal:IVec2) {
        let s = 0.5;
        let color = if normal.x == 0 {[1.0, 1.0, 1.0, 1.0]} else {[s, s, s, 1.0]};
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
        if let Some(DrawCall::DrawGeometry { texture, range }) = self.draw_calls.last_mut() {
            if wall_texture == *texture {
                range.end = end_index;
                return;
            }
        }

        self.draw_calls.push(DrawCall::DrawGeometry { texture: wall_texture, range: start_index..end_index });
            
    }

    fn sprite(&mut self, camera:&Camera, sprite:&Sprite) {
        let pos = sprite.pos;
        let color = [1.0, 1.0, 1.0, sprite.opacity];
        let start_vertex = self.sprites.vertices.len() as u32;
        let start_index = self.sprites.indicies.len() as u32;
        let sr = 0.5;
        let sh = 1.0;
        let n = camera.left() * vec3(sr, sr, 1.0);
        let wall = [[-n.x, -n.y, 0.0], [n.x, n.y, 0.0], [n.x, n.y, sh], [-n.x, -n.y, sh]];
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
            let p:Vec3 = v.position.into();
            let p = p + sprite.pos;
            v.position = p.into();
            self.sprites.vertices.push(v);
        }

        self.sprites.indicies.push(start_vertex + 0);
        self.sprites.indicies.push(start_vertex + 1);
        self.sprites.indicies.push(start_vertex + 2);

        self.sprites.indicies.push(start_vertex + 0);
        self.sprites.indicies.push(start_vertex + 2);
        self.sprites.indicies.push(start_vertex + 3);

        let end_index = self.sprites.indicies.len() as u32;
        if let Some(DrawCall::DrawSprite { texture, range }) = self.draw_calls.last_mut() {
            if sprite.texture == *texture {
                range.end = end_index;
                return;
            }
        }

        self.draw_calls.push(DrawCall::DrawSprite { texture: sprite.texture, range: start_index..end_index });
            
    }

    pub fn prepare(&mut self, graphics:&mut Graphics, camera:&Camera, scene:&Scene) {
        self.geometry.clear();
        self.sprites.clear();
        self.opaque_sprites.clear();
        self.translucent_sprites.clear();
        self.draw_calls.push(DrawCall::Clear {  });

        // update camera
        let camera_uniform = CameraUniform::new_scene_camera(camera, graphics.config.width as f32, graphics.config.height as f32);
        graphics.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[camera_uniform]),
        );

        // find wall textures in use
        let mut textures = HashMap::new();
        scene.grid.for_each(|cell, _| {
            if let Some(wall) = cell.wall {
                textures.insert(wall, ());
            }
        });
        let mut textures:Vec<u32> = textures.keys().map(|k|{*k}).collect();
        textures.sort();

        // once per texture, prepare walls that can be reached from a spot without a wall
        // i.e. dont prepare walls that are not reachable
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

        // draw floor 
        scene.grid.for_each(|cell, (x,y)| {
            if cell.wall.is_none() {
                self.floor(scene.floor_texture, IVec2::new(x, y));
            }
        });

        // draw ceiling
        scene.grid.for_each(|cell, (x,y)| {
            if cell.wall.is_none() {
                self.ceiling(scene.ceiling_texture, IVec2::new(x, y));
            }
        });

        // sort sprites into translucent and opaque
        // and find textures in use
        let mut textures = HashMap::new();
        for (index, sprite) in scene.sprites.iter().enumerate() {
            textures.insert(sprite.texture, ());
            if sprite.opacity == 1.0 {
                self.opaque_sprites.push(index);
            } else {
                self.translucent_sprites.push(index)
            }
        }
        let mut textures:Vec<u32> = textures.keys().map(|k|{*k}).collect();
        textures.sort();

        let sprites = replace(&mut self.opaque_sprites, Vec::new());
        // draw opaque sprites
        for texture in textures {
            for sprite in sprites.iter() {
                if let Some(sprite) = scene.sprites.get(*sprite as usize) {
                    if sprite.texture == texture {
                        self.sprite(camera, sprite);
                    }
                }
            }
        }
        self.opaque_sprites = sprites;

        let mut sprites = replace(&mut self.translucent_sprites, Vec::new());
        // sort sprites based upon distance to camera
        sprites.sort_by(|a, b|{
            if let (Some(a), Some(b)) = (scene.sprites.get(*a), scene.sprites.get(*b)) {
                let a = (a.pos - camera.pos).length_squared();
                let b = (b.pos - camera.pos).length_squared();
                if a < b {
                    return Ordering::Greater;
                } else if a > b {
                    return Ordering::Less;
                }
            }

            return Ordering::Equal;
        });
        // and draw
        for sprite in sprites.iter() {
            if let Some(sprite) = scene.sprites.get(*sprite as usize) {
                self.sprite(camera, sprite);
            }
        }
        self.translucent_sprites = sprites;
    }

    pub fn draw(&mut self, graphics:&mut GraphicsContext) {
        self.geometry.write(graphics);
        self.sprites.write(graphics);
        let draw_calls = replace(&mut self.draw_calls, Vec::new());
        for draw_call in draw_calls {
            match draw_call {
                DrawCall::DrawGeometry { texture, range } => {
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

                    render_pass.set_pipeline(&self.geometry_render_pipeline);
                    render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
                    render_pass.set_bind_group(1, texture, &[]);
                    render_pass.set_index_buffer(self.geometry.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
                    render_pass.set_vertex_buffer(0, self.geometry.vertex_buffer.slice(..));
                    render_pass.draw_indexed(range, 0, 0..1);
                },
                DrawCall::DrawSprite { texture, range } => {
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

                    render_pass.set_pipeline(&self.sprite_render_pipeline);
                    render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
                    render_pass.set_bind_group(1, texture, &[]);
                    render_pass.set_index_buffer(self.sprites.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
                    render_pass.set_vertex_buffer(0, self.sprites.vertex_buffer.slice(..));
                    render_pass.draw_indexed(range, 0, 0..1);
                },
                DrawCall::Clear {  } => {
                    graphics.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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