use std::{path::PathBuf, str::FromStr};
use engine::{Engine, engine_sdk::Engine as EngineTrait, engine_sdk::image};



async fn init() -> Engine {
    let mut engine = Engine::new().await;
    engine.window.borrow_mut().set_title("First-RS Editor");
    engine.set_game(Box::<engine_editor::Editor>::default());
    macro_rules! load_texture {
        ($id:expr, $path:expr) => {
            engine.load_texture($id, &image::load_from_memory(include_bytes!($path)).unwrap());
        };
    }
    
    load_texture!(1, "../assets/textures/brick_wall.png");
    load_texture!(2, "../assets/textures/bush_wall.png");
    load_texture!(3, "../assets/textures/white_wall.png");

    engine
}

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();
        pollster::block_on(async {
            let mut engine = init().await;
            #[cfg(debug_assertions)]
            {
                let lib_path = std::env::current_exe().unwrap().parent().unwrap().to_path_buf().join(PathBuf::from_str("engine_editor.dll").unwrap());
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