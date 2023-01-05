
use std::collections::HashMap;

use engine_sdk::Game;
use wgpu::{self, BufferDescriptor};
use winit::{event_loop::{EventLoop, ControlFlow}, window::{WindowBuilder}, event::{Event, WindowEvent, KeyboardInput, ElementState, VirtualKeyCode}};

use crate::{Graphics, Diagnostics, Model};

pub struct Engine {
    pub(crate) game:Option<Box<dyn Game>>,
    pub(crate) window:Option<winit::window::Window>,
    pub(crate) event_loop:Option<winit::event_loop::EventLoop<()>>,
    pub(crate) graphics:Graphics,
    pub diagnostics:Diagnostics
}

impl Engine {
    pub async fn new(game:Box<dyn Game>) -> Self {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().with_title("First-RS").build(&event_loop).unwrap();
        let size = window.inner_size();
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
                limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
               // limits: wgpu::Limits::downlevel_webgl2_defaults(),
                label: None,
            },
            None, // Trace path
        ).await.unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };
        surface.configure(&device, &config);

        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        let render_pipeline_layout =
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main", // 1.
                buffers: &[], // 2.
            },
            fragment: Some(wgpu::FragmentState { // 3.
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState { // 4.
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
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

        let max_vertices = 1024*1024*16;
        /*let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: &[0;max_vertices],//bytemuck::cast_slice(VERTICES),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );*/

       

        let render = Graphics {
            surface,
            device,
            queue,
            config,
            render_pipeline,
            models:HashMap::default()
        };

        Engine { window:Some(window), event_loop:Some(event_loop), game:Some(game), graphics: render, diagnostics:Default::default()  }
    }

    pub fn tick(&mut self) {
        let game = self.game.take();
        if let Some(mut game) = game {
            game.update(self);
            self.game = Some(game);
        }
        self.diagnostics.measure_frame_time();
    }

    pub fn init(&mut self) {
        let game = self.game.take();
        if let Some(mut game) = game {
            game.init(self);
            self.game = Some(game);
        }
    }

    pub async fn run(mut self) {
        let event_loop = self.event_loop.take().unwrap();
        let window = self.window.take().unwrap();
        self.init();
        
        event_loop.run(move |event, _, control_flow| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(new_size) => {
                    self.graphics.resize(new_size);
                },
                _ => {
                    
                }
            },
            Event::RedrawRequested(_window_id) => {
                self.tick();
            },
            Event::MainEventsCleared => {
                window.request_redraw();
            },
            _ => {}
        });
    }
}

impl engine_sdk::Engine for Engine {
    fn define_texture(&mut self, id:u32, texture:String) {
        self.graphics.models.insert(id, Model::new(&self.graphics.device));
        dbg!(self.graphics.models.len());
    }

    fn draw_scene(&mut self, camera:&engine_sdk::Camera, scene:&engine_sdk::Scene) {
        let (output, view, mut encoder) = self.graphics.begin();

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
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
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.graphics.render_pipeline);
            let size = 256*256;
            render_pass.draw(0..(size*6), 0..1);
        }

        self.graphics.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }

    fn frame_time(&self) -> std::time::Duration {
        self.diagnostics.frame_time
    }
}