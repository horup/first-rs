use crate::{CameraUniform, Vertex};
use egui::Context;
use engine_sdk::{image::{DynamicImage, GenericImage}, Atlas};
use std::collections::HashMap;
use wgpu::{
    util::DeviceExt, BindGroup, BindGroupLayout, Buffer, CommandEncoder, Device, Queue,
    RenderPipeline, SurfaceTexture, TextureFormat, TextureView,
};
use winit::{dpi::PhysicalSize, window::Window};

pub struct Graphics {
    pub pixels_per_point: f32,
    pub surface: wgpu::Surface,
    pub device: Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub render_pipeline: wgpu::RenderPipeline,
    pub camera_uniform: CameraUniform,
    pub camera_buffer: Buffer,
    pub camera_bind_group: BindGroup,
    pub textures: HashMap<u32, crate::Texture>,
    pub texture_bind_group_layout: BindGroupLayout,
    pub depth_texture_bind_group_layout: BindGroupLayout,
    pub texture_missing: crate::Texture,
    pub texture_white: crate::Texture,
    pub texture_depth: crate::Texture,
    pub render_format: TextureFormat,
    pub egui_painter: egui_wgpu::renderer::Renderer,
}

impl Graphics {
    pub async fn new<'a>(window: &'a Window) -> Self {
        let screen_size = window.inner_size();
        let pixels_per_point = window.scale_factor();
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(&window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let mut limits = wgpu::Limits::downlevel_webgl2_defaults();
        limits.max_texture_dimension_2d = 8192;
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    // WebGL doesn't support all of wgpu's features, so if
                    // we're building for the web we'll have to disable some.
                    limits,
                    // limits: wgpu::Limits::downlevel_webgl2_defaults(),
                    label: None,
                },
                None, // Trace path
            )
            .await
            .unwrap();

        let render_format = surface.get_supported_formats(&adapter)[0];
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: render_format,
            width: screen_size.width,
            height: screen_size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };
        surface.configure(&device, &config);

        let camera_uniform =
            CameraUniform::new_orth_screen(screen_size.width as f32, screen_size.height as f32);

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
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

        let depth_texture_bind_group_layout =
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
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Comparison),
                        count: None,
                    },
                ],
                label: Some("depth_texture_bind_group_layout"),
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
                entry_point: "vs_main",     // 1.
                buffers: &[Vertex::desc()], // 2.
            },
            fragment: Some(wgpu::FragmentState {
                // 3.
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    // 4.
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
                count: 1,                         // 2.
                mask: !0,                         // 3.
                alpha_to_coverage_enabled: false, // 4.
            },
            multiview: None, // 5.
        });

        // white texture
        let mut white = DynamicImage::new_rgba8(1, 1);
        white.as_mut_rgba8().unwrap().iter_mut().for_each(|c| {
            *c = 255;
        });

        let texture_white =
            crate::Texture::new(&device, &queue, &texture_bind_group_layout, &white, Atlas::default());

        // missing texture
        let mut texture_missing = DynamicImage::new_rgba8(2, 2);
        texture_missing.put_pixel(0, 0, [255, 0, 0, 255].into());
        texture_missing.put_pixel(1, 0, [0, 255, 0, 255].into());
        texture_missing.put_pixel(0, 1, [0, 0, 255, 255].into());
        texture_missing.put_pixel(1, 1, [0, 0, 0, 255].into());

        let texture_depth = crate::Texture::new_depth_texture(&device, &depth_texture_bind_group_layout, &config);

        let texture_missing = crate::Texture::new(
            &device,
            &queue,
            &texture_bind_group_layout,
            &texture_missing,
            Atlas::default()
        );

        let egui_painter = egui_wgpu::Renderer::new(&device, render_format, None, 1);
        Self {
            camera_uniform,
            surface,
            device,
            queue,
            config,
            render_pipeline,
            camera_buffer,
            camera_bind_group,
            textures: HashMap::new(),
            texture_missing: texture_missing,
            texture_white: texture_white,
            texture_bind_group_layout,
            depth_texture_bind_group_layout,
            texture_depth,
            render_format,
            egui_painter: egui_painter,
            pixels_per_point: pixels_per_point as f32,
        }
    }

    pub fn load_texture(&mut self, id: u32, image: &DynamicImage, atlas:Atlas) {
        let texture = crate::Texture::new(
            &self.device,
            &self.queue,
            &self.texture_bind_group_layout,
            image,
            atlas
        );
        self.textures.insert(id, texture);
    }

    pub fn get_atlas(&self, texture:Option<u32>) -> Atlas {
        if let Some(texture) = texture {
            if let Some(texture) = self.textures.get(&texture) {
                return texture.atlas.clone();
            }
        }

        return Atlas::default();
    }


    pub fn configure(&mut self) {
        self.surface.configure(&self.device, &self.config);
        self.texture_depth = crate::Texture::new_depth_texture(&self.device, &self.depth_texture_bind_group_layout, &self.config);
        self.update_camera();
    }

    fn update_camera(&mut self) {
        let camera_uniform = CameraUniform::new_orth_screen(
            self.config.width as f32,
            self.config.height as f32,
        );
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[camera_uniform]),
        );
    }

    pub fn draw_egui(&mut self, egui: &Context, full_output: egui::FullOutput, encoder: &mut CommandEncoder, texture_view: &TextureView) {
        let clipped_primitives = egui.tessellate(full_output.shapes);

        let sd = egui_wgpu::renderer::ScreenDescriptor {
            size_in_pixels: [self.config.width, self.config.height],
            pixels_per_point: self.pixels_per_point,
        };

        for (id, delta) in full_output.textures_delta.set.iter() {
            self.egui_painter
                .update_texture(&self.device, &self.queue, *id, delta);
        }

        self.egui_painter.update_buffers(&self.device, &self.queue, encoder, clipped_primitives.as_slice(), &sd);

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: texture_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            self.egui_painter.render(&mut render_pass, clipped_primitives.as_slice(), &sd);
        }

        for id in full_output.textures_delta.free.iter() {
            self.egui_painter.free_texture(id);
            dbg!("freeing texture");
        }
    }

    pub fn prepare(&mut self) -> (CommandEncoder, SurfaceTexture, TextureView) {
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });


        let surface_texture = match self.surface.get_current_texture() {
            Ok(surface_texture) => surface_texture,
            Err(_) => {
                self.configure();
                self.surface.get_current_texture().unwrap()
            },
        };

        let surface_view = surface_texture
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());
                encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &surface_view,
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

        return (encoder, surface_texture, surface_view);
    }

    pub fn submit(&mut self, encoder: CommandEncoder, surface_texture: SurfaceTexture) {
        self.queue.submit(std::iter::once(encoder.finish()));
        surface_texture.present();
    }
}

pub struct GraphicsContext<'a> {
    pub device: &'a Device,
    pub queue: &'a Queue,
    pub screen_size: PhysicalSize<u32>,
    pub encoder: &'a mut CommandEncoder,
    pub surface_view: &'a TextureView,
    pub render_pipeline: &'a RenderPipeline,
    pub camera_bind_group: &'a BindGroup,
    pub texture_white: &'a crate::Texture,
    pub texture_missing: &'a crate::Texture,
    pub texture_depth: &'a crate::Texture,
    pub textures: &'a HashMap<u32, crate::Texture>,
}

impl<'a> GraphicsContext<'a> {
    pub fn new(
        graphics: &'a mut Graphics,
        encoder: &'a mut CommandEncoder,
        surface_view: &'a TextureView,
    ) -> Self {
        Self {
            device: &graphics.device,
            queue: &graphics.queue,
            screen_size: PhysicalSize::new(graphics.config.width, graphics.config.height),
            encoder,
            surface_view,
            render_pipeline: &graphics.render_pipeline,
            camera_bind_group: &graphics.camera_bind_group,
            texture_white: &graphics.texture_white,
            texture_missing: &graphics.texture_missing,
            texture_depth: &graphics.texture_depth,
            textures: &graphics.textures,
        }
    }

    pub fn texture(&self, texture:u32) -> &'a crate::Texture {
        if let Some(texture) = self.textures.get(&texture) {
            return texture;
        }

        return self.texture_missing;
    }
}
