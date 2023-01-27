use engine_sdk::{Game, image, Event, Map, Engine, DrawTextParams, glam::vec2, Color};
use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Piggy {
    pub current_map:Map
}

impl Piggy {
    pub fn update_ui(&mut self, engine:&mut dyn Engine) {
        engine.draw_text(DrawTextParams {
            screen_pos: vec2(0.0, 0.0),
            text: "Hello world".into(),
            scale: 16.0,
            color: Color::WHITE,
        })
    }
}
