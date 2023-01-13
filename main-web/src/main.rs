use engine::Engine;
use log::info;
use game::MyGame;

fn main() {
    console_log::init_with_level(log::Level::Info).expect("Couldn't initialize logger");
    wasm_bindgen_futures::spawn_local(async {
        let g = MyGame::default();
        let engine = Engine::new(Box::new(g)).await;
      //  engine.run().await;  
        info!("hello world");
    });
}
