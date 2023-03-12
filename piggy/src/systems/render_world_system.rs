use engine_sdk::Engine;
use crate::State;

pub fn render_world_system(state:&mut State, engine:&mut dyn Engine) {
    let cam = state.camera;
    // draw scene
    engine.draw_scene(
        &cam,
        &mut state.as_world(),
    );
}