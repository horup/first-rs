use std::collections::HashMap;

use engine_sdk::{
    glam::{vec2, Vec2},
    Game,
};

use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent, DeviceEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::{Canvas, Diagnostics, Graphics, GraphicsContext, Model, Input};

pub struct Engine {
    pub(crate) game: Option<Box<dyn Game>>,
    pub(crate) window: Option<winit::window::Window>,
    pub(crate) event_loop: Option<winit::event_loop::EventLoop<()>>,
    pub(crate) graphics: Graphics,
    pub(crate) models: HashMap<u32, Model>,
    pub(crate) canvas: Canvas,
    pub diagnostics: Diagnostics,
    pub input: Input,
}

impl Engine {
    pub async fn new(game: Box<dyn Game>) -> Self {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title("First-RS")
            .build(&event_loop)
            .unwrap();
        let graphics = Graphics::new(&window).await;
        let canvas = Canvas::new(&graphics);

        Engine {
            window: Some(window),
            event_loop: Some(event_loop),
            game: Some(game),
            graphics,
            diagnostics: Default::default(),
            models: HashMap::default(),
            canvas,
            input:Input::default(),
        }
    }

    pub fn update(&mut self) {
        self.canvas.prepare();

        // do game update
        let game = self.game.take();
        if let Some(mut game) = game {
            game.update(self);
            self.game = Some(game);
        }

        // render results:
        self.graphics.prepare();
        let mut context = GraphicsContext::new(&mut self.graphics);
        self.canvas.draw(&mut context);
        self.graphics.present();
        self.diagnostics.measure_frame_time();
    }

    pub fn init(&mut self) {
        self.canvas.prepare();
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
                }
                WindowEvent::CursorMoved { position, ..} =>{
                    self.input.mouse_pos = vec2(position.x as f32, position.y as f32);
                }
                WindowEvent::MouseInput { button, state, .. } => {
                    let button = match button {
                        winit::event::MouseButton::Left => 0,
                        winit::event::MouseButton::Right => 1,
                        winit::event::MouseButton::Middle => 2,
                        winit::event::MouseButton::Other(_) => 3
                    };
                    match state {
                        ElementState::Pressed => self.input.mouse_pressed[button] = true,
                        ElementState::Released => self.input.mouse_pressed[button] = false,
                    }
                }
                _ => {}
            },
            Event::DeviceEvent { event, .. } => {
                match event {
                    DeviceEvent::Key(input) =>{
                        self.input.keys_pressed.insert(input.scancode, true);
                        self.input.keys_just_pressed.push(input.scancode);
                    }
                    _=>{}
                }
            }
            Event::RedrawRequested(_window_id) => {
                self.update();
                self.input.clear();
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        });
    }
}
