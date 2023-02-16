use std::{fs::Metadata, path::PathBuf};

use egui::Context;
use engine_sdk::Game;
use instant::Duration;
use libloading::Library;

use crate::Engine;

pub struct HotReloader {
    pub(crate) game_lib_path: PathBuf,
    pub(crate) game_lib: Option<Library>,
    pub(crate) game_lib_metadata: Option<Metadata>,
}

impl HotReloader {
    pub fn new(game_lib_path:PathBuf) -> Self {
        Self {
            game_lib_path,
            game_lib: None,
            game_lib_metadata: None,
        }
    }

    pub fn call_game_create(&mut self) -> Option<Box<dyn Game>> {
        if let Some(lib) = self.game_lib.take() {
            unsafe {
                if let Ok(f) = lib.get::<fn() -> Box<dyn Game>>(b"create") {
                    let game = f();
                    self.game_lib = Some(lib);
                    return Some(game);
                }
            }
            self.game_lib = Some(lib);
        }

        None
    }

    pub fn update(&mut self, engine:&mut Engine) {
        let metadata = std::fs::metadata(&self.game_lib_path);
        if let Ok(metadata) = metadata {
            let mut load_new = false;
            let mut unload = false;
            if self.game_lib_metadata.is_none()
                || self.game_lib_metadata.clone().unwrap().modified().unwrap()
                    != metadata.modified().unwrap()
            {
                load_new = true;
                if self.game_lib.is_some() {
                    unload = true;
                }
            }

            let mut state: Vec<u8> = Vec::new();
            if unload {
                {
                    let g = engine.game.take();
                    if let Some(g) = g {
                        state = g.serialize();
                    }
                    engine.egui_ctx = Context::default();
                    engine.egui_textures.clear();
                }
                // self.game = UnsafeCell::default();
                if let Some(lib) = self.game_lib.take() {
                    lib.close().unwrap();
                }
            }

            let load_new = load_new;
            while load_new {
                let mut to = std::env::current_exe().unwrap();
                to.pop();
                to.push("hot.module");

                if std::fs::copy(self.game_lib_path.clone(), to.clone()).is_ok() {
                    unsafe {
                        let lib = libloading::Library::new(to.clone());
                        match lib {
                            Ok(lib) => {
                                self.game_lib_metadata = Some(metadata);
                                self.game_lib = Some(lib);
                                let mut g = self.call_game_create().unwrap();
                                g.init(engine);
                                if unload {
                                    g.deserialize(&state);
                                }

                                engine.game = Some(g);

                                break;
                            }
                            Err(err) => {
                                println!("Could not load game lib with err:{:?}", err);
                                break;
                            }
                        }
                    }
                } else {
                    // retry
                    std::thread::sleep(Duration::from_millis(1000));
                    continue;
                }
            }

            if load_new && !unload {
                //self.events.push(Event::Start);
            }
        } else {
            println!("Could not load metadata of game lib");
        }
    }
}
