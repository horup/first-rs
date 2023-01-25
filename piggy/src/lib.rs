use engine_sdk::Game;
use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Piggy {
    test:u32
}

impl Game for Piggy {
    fn init(&mut self, engine:&mut dyn engine_sdk::Engine) {
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