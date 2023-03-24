use engine_sdk::{Engine, world::World, Camera};

pub fn render_world_system(world:&mut World, engine:&mut dyn Engine) {
    let cam = world.singleton::<Camera>().unwrap();
    // draw scene
    engine.draw_scene(
        &cam,
        world,
    );
}