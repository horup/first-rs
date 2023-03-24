use engine_sdk::{Engine, world::World, Camera};

use crate::Global;

pub fn render_world_system(world:&mut World, engine:&mut dyn Engine) {
    let global = world.singleton::<Global>().unwrap();
    // draw scene
    engine.draw_scene(
        &global.camera,
        world,
    );
}