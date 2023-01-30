use egui::{RawInput};
use engine_editor::Editor;
use engine_sdk::{glam::vec2, Game, TextureInfo};
use std::{collections::HashMap, cell::{RefCell}, mem::replace};

use winit::{
    event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent, Event},
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder}
};

use crate::{Canvas, Diagnostics, Graphics, GraphicsContext, Input, Model, SceneRenderer};

pub struct Engine {
    pub scene_renderer:SceneRenderer,
    pub new_events:Vec<engine_sdk::Event>,
    pub show_editor:bool,
    pub editor: Option<Editor>,
    pub(crate) textures:HashMap<u32, TextureInfo>,
    pub(crate) egui_ctx: egui::Context,
    pub(crate) egui_textures: HashMap<u32, egui::TextureHandle>,
    pub(crate) game: Option<Box<dyn Game>>,
    pub window: RefCell<winit::window::Window>,
    pub(crate) event_loop: Option<winit::event_loop::EventLoop<()>>,
    pub(crate) graphics: Graphics,
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
            //window.set_inner_size(PhysicalSize::new(800, 600));
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
            scene_renderer:SceneRenderer::new(&graphics),
            new_events:Vec::new(),
            show_editor: true,
            editor:Some(Editor::default()),
            textures:HashMap::default(),
            egui_ctx: egui::Context::default(),
            egui_textures: HashMap::default(),
            window: RefCell::new(window),
            event_loop: Some(event_loop),
            game: None,
            graphics,
            diagnostics: Default::default(),
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

        use engine_sdk::Engine as EngineTrait;
        if self.key_just_pressed(VirtualKeyCode::F1) {
            if self.show_editor {
                if let Some(editor) = self.editor.take() {
                    let map = editor.map.clone();
                    self.editor = Some(editor);
                    self.push_event(engine_sdk::Event::Map { map: map });
                }
            }

            self.show_editor = !self.show_editor;
        }

        // process events
        let events = replace(&mut self.new_events, Vec::new());
        for e in events {
            if let Some(mut editor) = self.editor.take() {
                editor.on_event(self, &e);
                self.editor = Some(editor);
            }
            if let Some(mut game) = self.game.take() {
                game.on_event(self, &e);
                self.game = Some(game);
            }
        }
        
        // do game update
        if self.show_editor {
            if let Some(mut editor) = self.editor.take() {
                editor.update(self);
                self.editor = Some(editor);
            }
        } else {
            let game = self.game.take();
            if let Some(mut game) = game {
                game.update(self);
                self.game = Some(game);
            }
        }

        let full_output = self.egui_ctx.end_frame();

        let mut context = GraphicsContext::new(&mut self.graphics, &mut encoder, &surface_view);
        self.scene_renderer.draw(&mut context);
        self.canvas.draw(&mut context);

        // draw ui always on top
        self.graphics
            .draw_egui(&self.egui_ctx, full_output, &mut encoder, &surface_view);

        self.graphics.submit(encoder, surface_texture);
        self.diagnostics.measure_frame_time();
        self.input.clear();
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
                        WindowEvent::MouseWheel { delta, .. } =>{
                            match delta {
                                winit::event::MouseScrollDelta::LineDelta(x, y) => {
                                    self.input.mouse_wheel_delta.x = *x;
                                    self.input.mouse_wheel_delta.y = *y;

                                },
                                winit::event::MouseScrollDelta::PixelDelta(loc) => {
                                    self.input.mouse_wheel_delta.x = loc.x as f32;
                                    self.input.mouse_wheel_delta.y = loc.y as f32;
                                },
                            }
                        }
                        WindowEvent::KeyboardInput { input, .. } => {
                            if let Some(key_code) = input.virtual_keycode {
                                match input.state {
                                    ElementState::Pressed => {
                                            self.input.keys_pressed.insert(key_code, true);
                                            self.input.keys_just_pressed.push(key_code);
                                    },
                                    ElementState::Released => {
                                        self.input.keys_pressed.remove(&key_code);
                                    },
                                }
                            }
                        }
                        _ => {}
                    }
                }
               /* Event::DeviceEvent { event, .. } => match event {
                    DeviceEvent::Key(input) => {
                        if let Some(key_code) = input.virtual_keycode {
                            match input.state {
                                ElementState::Pressed => {
                                        self.input.keys_pressed.insert(key_code, true);
                                        self.input.keys_just_pressed.push(key_code);
                                },
                                ElementState::Released => {
                                    self.input.keys_pressed.remove(&key_code);
                                },
                            }
                        }
                    }
                    _ => {}
                },*/
                Event::RedrawRequested(_window_id) => {
                    let egui_raw_inputs = egui_winit_state.take_egui_input(&self.window.borrow());
                    self.update(egui_raw_inputs);
                }
                Event::MainEventsCleared => {
                    self.window.borrow().request_redraw();
                    #[cfg(target_arch = "wasm32")]
                    {
                        let size = self.window.borrow().inner_size();
                        let new_size = winit::dpi::PhysicalSize::new(width() as u32, height() as u32);
                        if new_size != size {
                            self.window.borrow_mut().set_inner_size(new_size);
                        }
                    }
                }
                _ => {}
            }
        });
    }
}


#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern {
    fn width() -> f32;
    fn height() -> f32;
}