use wgpu::{self, Backends};
use winit::{event_loop::{EventLoop, ControlFlow}, window::{WindowBuilder, self}, event::{Event, WindowEvent, KeyboardInput, ElementState, VirtualKeyCode}};

pub struct Engine {
    window:winit::window::Window,
    event_loop:winit::event_loop::EventLoop<()>
}

impl Engine {
    pub async fn new() -> Self {
       /* let instance = wgpu::Instance::new(Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();*/

        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().with_title("First-RS").build(&event_loop).unwrap();


        Engine { window, event_loop }
    }

    pub async fn run(self) {
        let event_loop = self.event_loop;
        let window = self.window;
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
                _ => {}
            },
            _ => {}
        });
    }
}