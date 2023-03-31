use engine_sdk::{Engine, registry::Registry};

use crate::singletons::GameState;

pub fn render_world_system(registry:&mut Registry, engine:&mut dyn Engine) {
    let global = registry.singleton::<GameState>().unwrap();
    // draw scene
    engine.draw_scene(
        &global.camera,
        registry,
    );
}