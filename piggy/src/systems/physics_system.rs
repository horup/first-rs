use std::mem::take;

use engine_sdk::Engine;

pub fn physics_system(world:&mut World, engine:&mut dyn Engine) {
    let mut collisions = take(&mut state.collisions);
    //world.physics_step(engine.dt(), &mut collisions);
    //state.collisions = collisions;
}