use std::mem::take;

use engine_sdk::Engine;

use crate::State;

pub fn physics_system(state:&mut State, engine:&mut dyn Engine) {
    let mut collisions = take(&mut state.collisions);
    let mut world = state.as_world();
    world.physics_step(engine.dt(), &mut collisions);
    state.collisions = collisions;
}