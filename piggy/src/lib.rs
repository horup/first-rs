use engine_sdk::{Game, image};
use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Piggy {
    test:u32
}

impl Game for Piggy {
    fn init(&mut self, engine:&mut dyn engine_sdk::Engine) {
        macro_rules! load_texture {
            ($id:expr, $path:expr) => {
                engine.load_texture($id, &image::load_from_memory(include_bytes!($path)).unwrap());
            };
        }
        
        load_texture!(1, "../assets/textures/brick_wall.png");
        load_texture!(2, "../assets/textures/bush_wall.png");
        load_texture!(3, "../assets/textures/white_wall.png");
        load_texture!(4, "../assets/textures/player.png");
        load_texture!(5, "../assets/textures/viktor.png");
        load_texture!(6, "../assets/textures/william.png");
    }

    fn update(&mut self, engine:&mut dyn engine_sdk::Engine) {
        let events = engine.pop_events();
        if events.len() > 0 {
            dbg!("events");
        }
    }
}


#[no_mangle]
pub fn create() -> Box<dyn Game> {
    Box::new(Piggy::default())
}