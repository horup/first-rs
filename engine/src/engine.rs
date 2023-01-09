
use std::collections::HashMap;

use engine_sdk::Game;
use wgpu::{self, BufferDescriptor};
use winit::{event_loop::{EventLoop, ControlFlow}, window::{WindowBuilder}, event::{Event, WindowEvent, KeyboardInput, ElementState, VirtualKeyCode}};

use crate::{Graphics, Diagnostics, Model, Canvas};

pub struct Engine {
    pub(crate) game:Option<Box<dyn Game>>,
    pub(crate) window:Option<winit::window::Window>,
    pub(crate) event_loop:Option<winit::event_loop::EventLoop<()>>,
    pub(crate) graphics:Graphics,
    pub(crate) models: HashMap<u32, Model>,
    pub(crate) canvas: Canvas,
    pub diagnostics:Diagnostics
}

impl Engine {
    pub async fn new(game:Box<dyn Game>) -> Self {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().with_title("First-RS").build(&event_loop).unwrap();
        let graphics = Graphics::new(&window).await;
        let canvas = Canvas::new(&graphics);

        Engine { window:Some(window), event_loop:Some(event_loop), game:Some(game), graphics, diagnostics:Default::default(), models:HashMap::default(), canvas }
    }

    pub fn update(&mut self) {
        let game = self.game.take();
        if let Some(mut game) = game {
            game.update(self);
            self.game = Some(game);
        }

        self.graphics.update();
        self.canvas.draw(&self.graphics);
        self.diagnostics.measure_frame_time();
    }

    pub fn init(&mut self) {
        self.canvas.clear();
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
                    self.graphics.resize(*new_size);
                },
                _ => {
                    
                }
            },
            Event::RedrawRequested(_window_id) => {
                self.update();
            },
            Event::MainEventsCleared => {
                window.request_redraw();
            },
            _ => {}
        });
    }
}
