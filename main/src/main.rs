use engine::{Engine};

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();
        let current_exe_path = std::env::current_exe().unwrap();
        let mut lib_path = current_exe_path.parent().unwrap().to_path_buf();
        lib_path.push("game.dll");

        pollster::block_on(async {
            let mut engine = Engine::new().await;
            engine.set_dynamic_game(lib_path);
            engine.run().await;  
        }); 
    }   
    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init_with_level(log::Level::Info).expect("Couldn't initialize logger");
        wasm_bindgen_futures::spawn_local(async {
            let mut engine = Engine::new().await;
            engine.set_game(Box::new(game::MyGame::default()));
            engine.run().await;  
        });
    }
} 