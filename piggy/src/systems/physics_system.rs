use engine_sdk::{Engine, registry::Registry};

use crate::{singletons::GameState};

pub fn physics_system(registry:&mut Registry, engine:&mut dyn Engine) {
    let mut global = registry.singleton_mut::<GameState>().unwrap();
    engine.physics_step(registry, &mut global.collisions);
}