use std::{mem::{size_of}, ops::Range, cmp::Ordering, f32::consts::PI};

use egui::epaint::ahash::{HashMap, HashMapExt};
use engine_sdk::{Camera, registry::{Registry, EntityId}, glam::{ivec2, IVec2, Vec3, vec3}, Sprite, Atlas, Tilemap, Pic};
use wgpu::{BufferDescriptor, BindGroup, Buffer, RenderPipeline, StencilState, DepthBiasState};

use crate::{Graphics, CameraUniform, Vertex, Model, GraphicsContext};

pub struct SceneRenderer {
    camera_buffer:Buffer,
    camera_bind_group:BindGroup,
    geometry_render_pipeline:RenderPipeline,
    sprite_render_pipeline:RenderPipeline,
    opaque_sprites:Vec<EntityId>,
    translucent_sprites:Vec<EntityId>,
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


    fn ceiling(&mut self, pic:Pic, pos:IVec2, atlas:&Atlas) {
        let color = [1.0, 1.0, 1.0, 1.0];
        let start_vertex = self.geometry.vertices.len() as u32;
        let start_index = self.geometry.indicies.len() as u32;
        let ceiling = [[1.0, 0.0, 1.0], [1.0, 1.0, 1.0], [0.0, 1.0, 1.0], [0.0, 0.0, 1.0]];
        let u = atlas.u(pic.index);
        let v = atlas.v(pic.index);
        let floor = [Vertex {
            position: ceiling[0],
            color,
            uv: [u[0], v[1]],
        }, Vertex {
            position: ceiling[1],
            color,
            uv: [u[1], v[1]],
        }, Vertex {
            position: ceiling[2],
            color,
            uv: [u[1], v[0]],
        },  Vertex {
            position: ceiling[3],
            color,
            uv: [u[0], v[0]],
        }];

        for mut v in floor {
            v.position[0] += pos.x as f32;
            v.position[1] += pos.y as f32;
            self.geometry.vertices.push(v);
        }

        self.geometry.indicies.push(start_vertex);
        self.geometry.indicies.push(start_vertex + 1);
        self.geometry.indicies.push(start_vertex + 2);
        self.geometry.indicies.push(start_vertex);
        self.geometry.indicies.push(start_vertex + 2);
        self.geometry.indicies.push(start_vertex + 3);

        let end_index = self.geometry.indicies.len() as u32;
        if let Some(DrawCall::DrawGeometry { texture, range }) = self.draw_calls.last_mut() {
            if pic.atlas == *texture {
                range.end = end_index;
                return;
            }
        }

        self.draw_calls.push(DrawCall::DrawGeometry { texture: pic.atlas, range: start_index..end_index });
    }
    
    fn floor(&mut self, pic:Pic, pos:IVec2, atlas:&Atlas) {
        let color = [1.0, 1.0, 1.0, 1.0];
        let start_vertex = self.geometry.vertices.len() as u32;
        let start_index = self.geometry.indicies.len() as u32;

        let floor = [[0.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0, 0.0], [1.0, 0.0, 0.0]];
        let u = atlas.u(pic.index);
        let v = atlas.v(pic.index);
        let floor = [Vertex {
            position: floor[0],
            color,
            uv: [u[0], v[1]],
        }, Vertex {
            position: floor[1],
            color,
            uv: [u[1], v[1]],
        }, Vertex {
            position: floor[2],
            color,
            uv: [u[1], v[0]],
        },  Vertex {
            position: floor[3],
            color,
            uv: [u[0], v[0]],
        }];

        for mut v in floor {
            v.position[0] += pos.x as f32;
            v.position[1] += pos.y as f32;
            self.geometry.vertices.push(v);
        }

        self.geometry.indicies.push(start_vertex);
        self.geometry.indicies.push(start_vertex + 1);
        self.geometry.indicies.push(start_vertex + 2);
        self.geometry.indicies.push(start_vertex);
        self.geometry.indicies.push(start_vertex + 2);
        self.geometry.indicies.push(start_vertex + 3);

        let end_index = self.geometry.indicies.len() as u32;
        if let Some(DrawCall::DrawGeometry { texture, range }) = self.draw_calls.last_mut() {
            if pic.atlas == *texture {
                range.end = end_index;
                return;
            }
        }

        self.draw_calls.push(DrawCall::DrawGeometry { texture: pic.atlas, range: start_index..end_index });
    }

    fn wall(&mut self, pic:Pic, atlas:&Atlas, pos:IVec2, normal:IVec2) {
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

        let u = atlas.u(pic.index);
        let v = atlas.v(pic.index);
        let wall = [Vertex {
            position: wall[0],
            color,
            uv: [u[0], v[1]],
        }, Vertex {
            position: wall[1],
            color,
            uv: [u[1], v[1]],
        }, Vertex {
            position: wall[2],
            color,
            uv: [u[1], v[0]],
        },  Vertex {
            position: wall[3],
            color,
            uv: [u[0], v[0]],
        }];

        for mut v in wall {
            v.position[0] += pos.x as f32;
            v.position[1] += pos.y as f32;
            self.geometry.vertices.push(v);
        }

        self.geometry.indicies.push(start_vertex);
        self.geometry.indicies.push(start_vertex + 1);
        self.geometry.indicies.push(start_vertex + 2);

        self.geometry.indicies.push(start_vertex);
        self.geometry.indicies.push(start_vertex + 2);
        self.geometry.indicies.push(start_vertex + 3);

        let end_index = self.geometry.indicies.len() as u32;
        if let Some(DrawCall::DrawGeometry { texture, range }) = self.draw_calls.last_mut() {
            if pic.atlas == *texture {
                range.end = end_index;
                return;
            }
        }

        self.draw_calls.push(DrawCall::DrawGeometry { texture: pic.atlas, range: start_index..end_index });
            
    }

    fn sprite(&mut self, camera:&Camera, sprite:&Sprite, atlas:&Atlas) {
        let color = [1.0, 1.0, 1.0, sprite.opacity.unwrap_or(1.0)];
        let start_vertex = self.sprites.vertices.len() as u32;
        let start_index = self.sprites.indicies.len() as u32;
        let u = atlas.u(sprite.pic.index as u16);
        let v = atlas.v(sprite.pic.index as u16);
        match sprite.sprite_type {
            engine_sdk::SpriteType::Wall | engine_sdk::SpriteType::Facing => {
                let sr = 0.5;
                let sh = 0.5;
                let facing = sprite.facing - PI / 2.0;
                let n = match sprite.sprite_type {
                    engine_sdk::SpriteType::Wall => vec3(facing.cos() * sr, facing.sin() * sr, 0.0),
                    _ => -camera.left() * vec3(sr, sr, 0.0),
                };
                let wall = [[-n.x, -n.y, -sh], [n.x, n.y, -sh], [n.x, n.y, sh], [-n.x, -n.y, sh]];
                let wall = [Vertex {
                    position: wall[0],
                    color,
                    uv: [u[0], v[1]],
                }, Vertex {
                    position: wall[1],
                    color,
                    uv: [u[1], v[1]],
                }, Vertex {
                    position: wall[2],
                    color,
                    uv: [u[1], v[0]],
                },  Vertex {
                    position: wall[3],
                    color,
                    uv: [u[0], v[0]],
                }];

                for mut v in wall {
                    let p:Vec3 = v.position.into();
                    let p = p + sprite.pos;
                    v.position = p.into();
                    self.sprites.vertices.push(v);
                }
            },
            engine_sdk::SpriteType::Floor => {
                let sr = 0.5;
                let facing = sprite.facing - PI / 2.0;
                let a = vec3(facing.cos(), facing.sin(), 0.0);
                let b = -vec3(a.y, -a.x, 0.0);
                let wall = [-sr * a + -sr *b, -sr * a + sr * b, sr * a + sr * b, sr * a - sr * b];
                let u = atlas.u(sprite.pic.index as u16);
                let v = atlas.v(sprite.pic.index as u16);
                let wall = [Vertex {
                    position: wall[0].into(),
                    color,
                    uv: [u[0], v[1]],
                }, Vertex {
                    position: wall[1].into(),
                    color,
                    uv: [u[1], v[1]],
                }, Vertex {
                    position: wall[2].into(),
                    color,
                    uv: [u[1], v[0]],
                },  Vertex {
                    position: wall[3].into(),
                    color,
                    uv: [u[0], v[0]],
                }];

                for mut v in wall {
                    let p:Vec3 = v.position.into();
                    let p = p + sprite.pos;
                    v.position = p.into();
                    self.sprites.vertices.push(v);
                }
            },
        }
        
        self.sprites.indicies.push(start_vertex);
        self.sprites.indicies.push(start_vertex + 1);
        self.sprites.indicies.push(start_vertex + 2);

        self.sprites.indicies.push(start_vertex);
        self.sprites.indicies.push(start_vertex + 2);
        self.sprites.indicies.push(start_vertex + 3);

        let end_index = self.sprites.indicies.len() as u32;
        if let Some(DrawCall::DrawSprite { texture, range }) = self.draw_calls.last_mut() {
            if sprite.pic.atlas == *texture {
                range.end = end_index;
                return;
            }
        }

        self.draw_calls.push(DrawCall::DrawSprite { texture: sprite.pic.atlas, range: start_index..end_index });
            
    }

    pub fn prepare(&mut self, graphics:&mut Graphics, camera:&Camera, scene:&Registry) {
        self.geometry.clear();
        self.sprites.clear();
        self.opaque_sprites.clear();
        self.translucent_sprites.clear();
        self.draw_calls.push(DrawCall::Clear {  });
        let tilemap = scene.singleton::<Tilemap>().unwrap();

        // update camera
        let camera_uniform = CameraUniform::new_scene_camera(camera, graphics.config.width as f32, graphics.config.height as f32);
        graphics.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[camera_uniform]),
        );

        // find wall pics in use
        let mut pics = HashMap::new();
        tilemap.grid.for_each(|cell, _| {
            if let Some(pic) = cell.wall {
                pics.insert(pic, ());
            }
        });
        let mut pics:Vec<Pic> = pics.keys().copied().collect();
        pics.sort();

        // once per texture, prepare walls that can be reached from a spot without a wall
        // i.e. dont prepare walls that are not reachable
        for pic in pics {
            if let Some(tex) = graphics.textures.get(&pic.atlas) {
                let atlas = &tex.atlas;
                tilemap.grid.for_each(|cell, (x,y)| {
                    if cell.wall.is_none() {
                        let directions = [ivec2(0, 1), ivec2(0, -1), ivec2(1, 0), ivec2(-1, 0)];
                        for n in directions.iter() {
                            let p = ivec2(x, y) - *n;
                            if let Some(cell) = tilemap.grid.get((p.x, p.y)) {
                                if let Some(wall_pic) = cell.wall {
                                    if wall_pic == pic {

                                        self.wall(wall_pic, atlas,  p, -*n);
                                    }
                                }
                            }
                        }
                    }
                });
            }
        }

        let atlas = graphics.get_atlas(Some(tilemap.floor_texture));
        // draw floor 
        tilemap.grid.for_each(|cell, (x,y)| {
            if cell.wall.is_none() {
                self.floor(tilemap.floor_texture, IVec2::new(x, y), &atlas);
            }
        });

        let atlas = graphics.get_atlas(Some(tilemap.ceiling_texture));
        // draw ceiling
        tilemap.grid.for_each(|cell, (x,y)| {
            if cell.wall.is_none() {
                self.ceiling(tilemap.ceiling_texture, IVec2::new(x, y), &atlas);
            }
        });

        // sort sprites into translucent and opaque
        // and find textures in use
        let mut textures = HashMap::new();
        //let visible = |(_, sprite):&(EntityId, &Sprite)| !sprite.hidden;
        let sprites = scene.components::<Sprite>();
        for id in scene.iter() {
            if let Some(sprite) = sprites.get(id) {
                if !sprite.hidden {
                    textures.insert(sprite.pic.atlas, ());
                    if sprite.opacity.is_none() {
                        self.opaque_sprites.push(id);
                    } else {
                        self.translucent_sprites.push(id)
                    }
                }
                
            }
            
        }
        let mut textures:Vec<u32> = textures.keys().copied().collect();
        textures.sort();

        let sprites = std::mem::take(&mut self.opaque_sprites);
        // draw opaque sprites
        for texture in textures {
            for sprite in sprites.iter() {
                if let Some(sprite) = scene.component::<Sprite>(*sprite) {
                    if sprite.pic.atlas == texture {
                        if let Some(texture) = graphics.textures.get(&texture) {
                            self.sprite(camera, &sprite, &texture.atlas);
                        }
                    }
                }
            }
        }
        self.opaque_sprites = sprites;

        let mut sprites = std::mem::take(&mut self.translucent_sprites);
        // sort sprites based upon texture (might improve performance since textures of same type might be closer together ?)
        sprites.sort_by(|a, b|{
            if let (Some(a), Some(b)) = (scene.component::<Sprite>(*a), scene.component::<Sprite>(*b)) {
                if a.pic.atlas < b.pic.atlas {
                    return Ordering::Greater;
                } else if a.pic.atlas > b.pic.atlas {
                    return Ordering::Less;
                }
            }
            Ordering::Equal
        });

        // then sort sprites based upon distance to camera
        sprites.sort_by(|a, b|{
            if let (Some(a), Some(b)) = (scene.component::<Sprite>(*a), scene.component::<Sprite>(*b)) {
                let a = (a.pos - camera.pos).length_squared();
                let b = (b.pos - camera.pos).length_squared();
                if a < b {
                    return Ordering::Greater;
                } else if a > b {
                    return Ordering::Less;
                }
            }

            Ordering::Equal
        });

        // and draw
        for sprite in sprites.iter() {
            if let Some(sprite) = scene.component::<Sprite>(*sprite) {
                if let Some(texture) = graphics.textures.get(&sprite.pic.atlas) {
                    self.sprite(camera, &sprite, &texture.atlas);
                }
            }
        }
        self.translucent_sprites = sprites;
    }

    pub fn draw(&mut self, graphics:&mut GraphicsContext) {
        self.geometry.write(graphics);
        self.sprites.write(graphics);
        let draw_calls = std::mem::take(&mut self.draw_calls);
        for draw_call in draw_calls {
            match draw_call {
                DrawCall::DrawGeometry { texture, range } => {
                    let texture = &graphics.texture(texture).clone().texture_bind_group;
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
                            view: graphics.surface_view,
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
                            view: graphics.surface_view,
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