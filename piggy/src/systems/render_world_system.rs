use engine_sdk::{Engine, registry::Registry};

use crate::Global;

pub fn render_registry_system(registry:&mut Registry, engine:&mut dyn Engine) {
    let global = registry.singleton::<Global>().unwrap();
    // draw scene
    engine.draw_scene(
        &global.camera,
        registry,
    );
}