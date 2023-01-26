use engine_sdk::{Game, image, Event, Map};
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

        let map:Map = serde_json::from_str(include_str!("../assets/maps/test.map")).unwrap();
        engine.push_event(Event::LoadMap { map });
    }

    fn update(&mut self, engine:&mut dyn engine_sdk::Engine) {
        
    }

    fn on_event(&mut self, engine:&mut dyn engine_sdk::Engine, event:&Event) {
        match event {
            Event::LoadMap { map } => {
                dbg!("load map");
            },
            _=>{}
        }
    }

    
}


#[no_mangle]
pub fn create() -> Box<dyn Game> {
    Box::new(Piggy::default())
}