use engine::{Engine};
use game::MyGame;

async fn run() {
    let g = MyGame::default();
    let engine = Engine::new(Box::new(g)).await;
    engine.run().await;  
}

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();
        pollster::block_on(async {
            run().await;
        }); 
    }   
    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init_with_level(log::Level::Info).expect("Couldn't initialize logger");
        wasm_bindgen_futures::spawn_local(async {
            run().await;
        });
    }
} 