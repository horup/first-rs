use std::{path::PathBuf, str::FromStr};
use engine::{Engine};
use piggy::Piggy;

async fn init() -> Engine {
    let mut engine = Engine::new().await;
    engine.window.borrow_mut().set_title("Piggy");
    engine.set_game(Box::<Piggy>::default());
    engine
}

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();
        pollster::block_on(async {
            let mut engine = init().await;
            engine.show_editor = false;
            #[cfg(debug_assertions)]
            {
                let lib_path = std::env::current_exe().unwrap().parent().unwrap().to_path_buf().join(PathBuf::from_str("piggy.dll").unwrap());
                engine.set_game_hotreload(lib_path);
            }
            engine.run().await;  
        }); 
    }   
    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init_with_level(log::Level::Info).expect("Couldn't initialize logger");
        wasm_bindgen_futures::spawn_local(async {
            let mut engine = init().await;
            engine.run().await;  
        });
    }
} 