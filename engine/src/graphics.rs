use engine_sdk::glam::Vec2;
use wgpu::{Device, TextureView, CommandEncoder, SurfaceTexture, util::DeviceExt, Buffer, BindGroup, Texture, Queue, RenderPipeline};
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

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
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

        let shader = device.create_shader_module(wgpu::include_wgsl!("../shaders/shader.wgsl"));
        let render_pipeline_layout =
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&camera_bind_group_layout],
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
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
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
            encoder:None
        }

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

    pub fn begin(&mut self) {
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

    pub fn finish(&mut self) {
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
    pub camera_bind_group:&'a BindGroup
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
            camera_bind_group:&graphics.camera_bind_group
        }
    }
}