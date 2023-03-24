use engine_sdk::Engine;

pub fn render_world_system(world:&mut World, engine:&mut dyn Engine) {
    let cam = state.camera;
    // draw scene
    engine.draw_scene(
        &cam,
        world,
    );
}