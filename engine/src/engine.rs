use wgpu::{self, Backends, Device};
use winit::{event_loop::{EventLoop, ControlFlow}, window::{WindowBuilder, self}, event::{Event, WindowEvent, KeyboardInput, ElementState, VirtualKeyCode}};

use crate::{Game, Context};

pub struct Engine<T : Game> {
    game:T,
    window:Option<winit::window::Window>,
    event_loop:Option<winit::event_loop::EventLoop<()>>,
    surface: wgpu::Surface,
    device: Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
}

impl<T : Game> Engine<T> {
    pub async fn new(game:T) -> Self {
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


        Engine { window:Some(window), event_loop:Some(event_loop), game, surface, queue, config, device  }
    }

    pub fn context(&self) -> Context {
        Context::new(&self.surface, &self.device, &self.queue)
    }

    pub async fn run(mut self) {
        let event_loop = self.event_loop.take().unwrap();
        let window = self.window.take().unwrap();
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
                _ => {
                    
                }
            },
            Event::RedrawRequested(window_id) => {
                {
                    let ctx = self.context();
                    ctx.draw();
                }
            },
            Event::MainEventsCleared => {
                window.request_redraw();
            },
            _ => {}
        });
    }
}