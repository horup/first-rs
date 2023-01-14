use engine::Engine;
use game::MyGame;

fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Info).expect("Couldn't initialize logger");
    wasm_bindgen_futures::spawn_local(async {
        let g = MyGame::default();
        let engine = Engine::new(Box::new(g)).await;
        engine.run().await;
    });
}
