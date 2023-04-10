use engine_sdk::{Engine, registry::Registry};

use crate::singletons::Global;

pub fn render_world_system(registry:&mut Registry, engine:&mut dyn Engine) {
    let global = registry.singleton::<Global>().unwrap();
    // draw scene
    engine.draw_scene(
        &global.camera,
        registry,
    );
}