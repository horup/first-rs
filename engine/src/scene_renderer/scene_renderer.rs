use std::{mem::size_of, ops::Range};

use engine_sdk::{Camera, Scene};
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
                cull_mode: Some(wgpu::Face::Back),
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

    pub fn prepare(&mut self, graphics:&mut Graphics, camera:&Camera, scene:&Scene) {
        self.geometry.clear();

        // update camera
        let camera_uniform = CameraUniform::new_orth_screen(graphics.config.width as f32, graphics.config.height as f32);
        graphics.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[camera_uniform]),
        );

    }

    pub fn draw(&mut self, graphics:&mut GraphicsContext) {

        // update camera
      /*   let camera_uniform = CameraUniform::new_orth_screen(graphics.screen_size.width as f32, graphics.screen_size.height as f32);
        graphics.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[camera_uniform]),
        );*/
        
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
        render_pass.draw_indexed(0..0, 0, 0..1);


        self.draw_calls.clear();
    }
}