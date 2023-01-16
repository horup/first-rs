use std::collections::HashMap;
use egui::{FontDefinitions, RawInput};
use engine_sdk::{
    glam::{vec2},
    Game,
};

use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent, DeviceEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::{Canvas, Diagnostics, Graphics, GraphicsContext, Model, Input};

pub struct Engine {
    pub(crate) egui_ctx: egui::Context,
    pub(crate) game: Option<Box<dyn Game>>,
    pub(crate) window: Option<winit::window::Window>,
    pub(crate) event_loop: Option<winit::event_loop::EventLoop<()>>,
    pub(crate) graphics: Graphics,
    pub(crate) models: HashMap<u32, Model>,
    pub(crate) canvas: Canvas,
    pub diagnostics: Diagnostics,
    pub input: Input,
    #[cfg(not(target_arch = "wasm32"))]
    pub hot_reloader:Option<crate::hot_reloader::HotReloader>
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
            egui_ctx:egui::Context::default(),
            window: Some(window),
            event_loop: Some(event_loop),
            game: None,
            graphics,
            diagnostics: Default::default(),
            models: HashMap::default(),
            canvas,
            input:Input::default(),
            #[cfg(not(target_arch = "wasm32"))]
            hot_reloader:None
        }
    }

    pub fn set_game(&mut self, game:Box<dyn Game>) {
        self.game = Some(game);
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn set_game_hotreload(&mut self, lib_path:std::path::PathBuf) {
        self.hot_reloader = Some(crate::hot_reloader::HotReloader::new(lib_path));
    }

    pub fn update(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let hot_reloader = self.hot_reloader.take();
            if let Some(mut hot_reloader) = hot_reloader {
                hot_reloader.update(self);
                self.hot_reloader = Some(hot_reloader);
            }
        }
        self.canvas.prepare();

        // do game update
        let game = self.game.take();
        if let Some(mut game) = game {
            game.update(self);
            self.game = Some(game);
        }

        // generate ui for rendering
        let full_output = self.egui_ctx.run(RawInput::default(), |egui_ctx| {
            //my_app.ui(egui_ctx); // add panels, windows and widgets to `egui_ctx` here
            egui::CentralPanel::default().show(egui_ctx, |ui| {
                ui.heading("My egui Application");
                ui.horizontal(|ui| {
                    let name_label = ui.label("Your name: ");
                });
               
                ui.label(format!("Hello '{}', age {}", 1, 2));
            });
           // egui_ctx.
        });

        // render canvas
        self.graphics.prepare();
        let mut context = GraphicsContext::new(&mut self.graphics);
        self.canvas.draw(&mut context);

        // draw ui always on top
        self.graphics.draw_ui(&self.egui_ctx, full_output);

        // present and measure time
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

        // let egui_platform = egui_winit_platform::Platform::new(egui_winit_platform::PlatformDescriptor {
        //     physical_width: window.inner_size().width as u32,
        //     physical_height: window.inner_size().width as u32,
        //     scale_factor: window.scale_factor(),
        //     font_definitions: FontDefinitions::default(),
        //     style: Default::default(),
        // });

        // let mut egui_rpass = RenderPass::new(&self.graphics.device, self.graphics.render_format, 1);

       


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
