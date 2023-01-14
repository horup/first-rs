use std::collections::HashMap;

use engine_sdk::image::{DynamicImage, RgbaImage, GenericImage};
use wgpu::{Device, TextureView, CommandEncoder, SurfaceTexture, util::DeviceExt, Buffer, BindGroup, Texture, Queue, RenderPipeline, BindGroupLayout, TextureFormat};
use winit::{dpi::PhysicalSize, window::Window};
use crate::{Vertex, CameraUniform};

pub struct Graphics {
    pub surface: wgpu::Surface,
    pub surface_view: Option<TextureView>,
    pub surface_texture: Option<SurfaceTexture>,
    pub encoder:Option<CommandEncoder>,
    pub device: Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub render_pipeline: wgpu::RenderPipeline,
    pub camera_uniform:CameraUniform,
    pub camera_buffer:Buffer,
    pub camera_bind_group:BindGroup,
    pub screen_size:PhysicalSize<u32>,
    pub textures:HashMap<u32, crate::Texture>,
    pub texture_bind_group_layout:BindGroupLayout,
    pub texture_missing:crate::Texture,
    pub texture_white:crate::Texture,
    pub render_format:TextureFormat
}

impl Graphics {
    pub async fn new<'a>(window:&'a Window) -> Self {
        let screen_size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(&window) };
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                // WebGL doesn't support all of wgpu's features, so if
                // we're building for the web we'll have to disable some.
                limits: wgpu::Limits::downlevel_webgl2_defaults(),
               // limits: wgpu::Limits::downlevel_webgl2_defaults(),
                label: None,
            },
            None, // Trace path
        ).await.unwrap();

        let render_format =  surface.get_supported_formats(&adapter)[0];
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: render_format,
            width: screen_size.width,
            height: screen_size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };
        surface.configure(&device, &config);

        let camera_uniform = CameraUniform::new_orth_screen(screen_size.width as f32, screen_size.height as f32);
        
        let camera_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&[camera_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );
        
        let camera_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
            label: Some("camera_bind_group_layout"),
        });
        
        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding(),
                }
            ],
            label: Some("camera_bind_group"),
        });

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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

        let shader = device.create_shader_module(wgpu::include_wgsl!("../shaders/shader.wgsl"));
        let render_pipeline_layout =
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&camera_bind_group_layout, &texture_bind_group_layout],
            push_constant_ranges: &[],
        });
    
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main", // 1.
                buffers: &[Vertex::desc()], // 2.
            },
            fragment: Some(wgpu::FragmentState { // 3.
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState { // 4.
                    format: config.format,
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
                count: 1, // 2.
                mask: !0, // 3.
                alpha_to_coverage_enabled: false, // 4.
            },
            multiview: None, // 5.
        });

        // white texture
        let mut white = DynamicImage::new_rgba8(1, 1);
        white.as_mut_rgba8().unwrap().iter_mut().for_each(|c|{
            *c = 255;
        });
        
        let texture_white = crate::Texture::new(&device, &queue, &texture_bind_group_layout, &white);

        // missing texture
        let mut texture_missing = DynamicImage::new_rgba8(2, 2);
        texture_missing.put_pixel(0, 0, [255, 0, 0, 255].into());
        texture_missing.put_pixel(1, 0, [0, 255, 0, 255].into());
        texture_missing.put_pixel(0, 1, [0, 0, 255, 255].into());
        texture_missing.put_pixel(1, 1, [0, 0, 0, 255].into());

        
        let texture_missing = crate::Texture::new(&device, &queue, &texture_bind_group_layout, &texture_missing);


        Self {
            camera_uniform,
            surface,
            device,
            queue,
            config,
            render_pipeline,
            camera_buffer,
            camera_bind_group,
            screen_size,
            surface_view:None,
            surface_texture:None,
            encoder:None,
            textures:HashMap::new(),
            texture_missing: texture_missing,
            texture_white: texture_white,
            texture_bind_group_layout,
            render_format
        }

    }

    pub fn load_texture(&mut self, id:u32, image:&DynamicImage) {
        let texture = crate::Texture::new(&self.device, &self.queue, &self.texture_bind_group_layout, image);
        self.textures.insert(id, texture);
    }

    pub fn resize(&mut self, new_size:PhysicalSize<u32>) {
        self.screen_size = new_size;
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&self.device, &self.config);
    }

    fn update_camera(&mut self) {
        let camera_uniform = CameraUniform::new_orth_screen(self.screen_size.width as f32, self.screen_size.height as f32);
        self.queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[camera_uniform]));
    }

    fn clear(&mut self) {
        self.encoder.as_mut().unwrap().begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: self.surface_view.as_ref().unwrap(),
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: true,
                },
            })],
            ..Default::default()
        });
    }

    pub fn prepare(&mut self) {
        let surface_texture = self.surface.get_current_texture().unwrap();
        let surface_view = surface_texture.texture.create_view(&wgpu::TextureViewDescriptor::default()); 
        self.surface_view = Some(surface_view);
        self.surface_texture = Some(surface_texture);
        let encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        self.encoder = Some(encoder);

        self.update_camera();
        self.clear();
        //run.set_pipeline(&self.render_pipeline);
    }

    pub fn present(&mut self) {
        self.surface_view = None;
        let encoder = self.encoder.take().unwrap();
        let surface_texture = self.surface_texture.take().unwrap();
        self.queue.submit(std::iter::once(encoder.finish()));
        surface_texture.present();
    }
}


pub struct GraphicsContext<'a> {
    pub device:&'a Device,
    pub queue:&'a Queue,
    pub screen_size:PhysicalSize<u32>,
    pub encoder:&'a mut CommandEncoder,
    pub surface_view:&'a TextureView,
    pub surface_texture:&'a SurfaceTexture,
    pub render_pipeline:&'a RenderPipeline,
    pub camera_bind_group:&'a BindGroup,
    pub texture_white:&'a crate::Texture,
    pub texture_missing:&'a crate::Texture,
    pub textures:&'a HashMap<u32, crate::Texture>
}

impl<'a> GraphicsContext<'a> {
    pub fn new(graphics:&'a mut Graphics) -> Self {
        Self {
            device:&graphics.device,
            queue:&graphics.queue,
            screen_size:graphics.screen_size,
            encoder:graphics.encoder.as_mut().unwrap(),
            surface_texture:graphics.surface_texture.as_ref().unwrap(),
            surface_view:graphics.surface_view.as_ref().unwrap(),
            render_pipeline:&graphics.render_pipeline,
            camera_bind_group:&graphics.camera_bind_group,
            texture_white:&graphics.texture_white,
            texture_missing:&graphics.texture_missing,
            textures:&graphics.textures
        }
    }
}