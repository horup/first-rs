use std::{fs::Metadata, path::PathBuf};

use engine_sdk::Game;
use libloading::Library;

pub struct HotReloader {
    pub(crate) game_lib_path: PathBuf,
    pub(crate) game_lib: Option<Library>,
    pub(crate) game_lib_metadata: Option<Metadata>,
}

impl HotReloader {
    pub fn new() -> Self {
        Self { game_lib_path: PathBuf::default(), game_lib: None, game_lib_metadata: None }
    }

    pub fn update(&mut self, game:&mut Option<Box<dyn Game>>) {
        
    }

}