use egui::{FontDefinitions, RawInput};
use engine_sdk::{glam::vec2, Game};
use std::{collections::HashMap, cell::{RefCell, RefMut}};

use winit::{
    event::{DeviceEvent, ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, Window},
};

use crate::{Canvas, Diagnostics, Graphics, GraphicsContext, Input, Model};

pub struct Engine {
    pub(crate) egui_ctx: egui::Context,
    pub(crate) game: Option<Box<dyn Game>>,
    pub window: RefCell<winit::window::Window>,
    pub(crate) event_loop: Option<winit::event_loop::EventLoop<()>>,
    pub(crate) graphics: Graphics,
    pub(crate) models: HashMap<u32, Model>,
    pub(crate) canvas: Canvas,
    pub diagnostics: Diagnostics,
    pub input: Input,
    #[cfg(not(target_arch = "wasm32"))]
    pub hot_reloader: Option<crate::hot_reloader::HotReloader>,
}

impl Engine {
    pub async fn new() -> Self {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title("First-RS")
            .build(&event_loop)
            .unwrap();
        #[cfg(target_arch = "wasm32")]
        {
            // Winit prevents sizing with CSS, so we have to set
            // the size manually when on web.
            use winit::dpi::PhysicalSize;
            window.set_inner_size(PhysicalSize::new(450, 400));
            use winit::platform::web::WindowExtWebSys;
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| {
                    let dst = doc.get_element_by_id("main")?;
                    let canvas = web_sys::Element::from(window.canvas());
                    dst.append_child(&canvas).ok()?;
                    Some(())
                })
                .expect("Couldn't append canvas to document body.");
        }

        let graphics = Graphics::new(&window).await;
        let canvas = Canvas::new(&graphics);

        Engine {
            egui_ctx: egui::Context::default(),
            window: RefCell::new(window),
            event_loop: Some(event_loop),
            game: None,
            graphics,
            diagnostics: Default::default(),
            models: HashMap::default(),
            canvas,
            input: Input::default(),
            #[cfg(not(target_arch = "wasm32"))]
            hot_reloader: None,
        }
    }

    pub fn set_game(&mut self, game: Box<dyn Game>) {
        self.game = Some(game);
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn set_game_hotreload(&mut self, lib_path: std::path::PathBuf) {
        self.hot_reloader = Some(crate::hot_reloader::HotReloader::new(lib_path));
    }

    pub fn update(&mut self, egui_raw_input: RawInput) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let hot_reloader = self.hot_reloader.take();
            if let Some(mut hot_reloader) = hot_reloader {
                hot_reloader.update(self);
                self.hot_reloader = Some(hot_reloader);
            }
        }

        let (mut encoder, surface_texture, surface_view) = self.graphics.prepare();
        self.canvas.prepare();

        self.egui_ctx.begin_frame(egui_raw_input);

        // do game update
        let game = self.game.take();
        if let Some(mut game) = game {
            game.update(self);
            self.game = Some(game);
        }

        let full_output = self.egui_ctx.end_frame();

        let mut context = GraphicsContext::new(&mut self.graphics, &mut encoder, &surface_view);
        self.canvas.draw(&mut context);

        // draw ui always on top
        self.graphics
            .draw_egui(&self.egui_ctx, full_output, &mut encoder, &surface_view);

        self.graphics.submit(encoder, surface_texture);
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
        //let window = self.window.take().unwrap();
        self.init();

        let mut egui_winit_state = egui_winit::State::new(&event_loop);

        event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == self.window.borrow().id() => {
                    let res = egui_winit_state.on_event(&self.egui_ctx, &event);
                    if res.consumed == true {
                        // egui consumed the event
                        return;
                    }

                    match event {
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
                            self.graphics.config.width = new_size.width.max(1);
                            self.graphics.config.height = new_size.height.max(1);
                            self.graphics.configure(); 
                        }
                        WindowEvent::CursorMoved { position, .. } => {
                            self.input.mouse_pos = vec2(position.x as f32, position.y as f32);
                        }
                        WindowEvent::MouseInput { button, state, .. } => {
                            let button = match button {
                                winit::event::MouseButton::Left => 0,
                                winit::event::MouseButton::Right => 1,
                                winit::event::MouseButton::Middle => 2,
                                winit::event::MouseButton::Other(_) => 3,
                            };
                            match state {
                                ElementState::Pressed => self.input.mouse_pressed[button] = true,
                                ElementState::Released => self.input.mouse_pressed[button] = false,
                            }
                        }
                        _ => {}
                    }
                }
                Event::DeviceEvent { event, .. } => match event {
                    DeviceEvent::Key(input) => {
                        self.input.keys_pressed.insert(input.scancode, true);
                        self.input.keys_just_pressed.push(input.scancode);
                    }
                    _ => {}
                },
                Event::RedrawRequested(_window_id) => {
                    let egui_raw_inputs = egui_winit_state.take_egui_input(&self.window.borrow());
                    self.update(egui_raw_inputs);
                    self.input.clear();
                }
                Event::MainEventsCleared => {
                    self.window.borrow().request_redraw();
                }
                _ => {}
            }
        });
    }
}
