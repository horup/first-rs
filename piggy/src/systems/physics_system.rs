use engine_sdk::{Engine, world::World};

use crate::Global;

pub fn physics_system(world:&mut World, engine:&mut dyn Engine) {
    let mut global = world.singleton::<Global>().unwrap();
    engine.physics_step(world, &mut global.collisions);
}