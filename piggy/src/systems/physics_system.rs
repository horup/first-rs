use engine_sdk::Engine;

use crate::State;

pub fn physics_system(state:&mut State, engine:&mut dyn Engine) {
    let mut world = state.as_world();
    world.physics_step(engine.dt());
}