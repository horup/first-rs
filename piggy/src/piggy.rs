use engine_sdk::{Game, image, Event, Map, Engine, DrawTextParams, glam::vec2, Color, DrawLineParams};
use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Piggy {
    pub current_map:Map
}

impl Piggy {
    pub fn update_ui(&mut self, engine:&mut dyn Engine) {
        let margin = vec2(16.0, 16.0);
        let center = engine.screen_size() / 2.0;
        engine.draw_text(DrawTextParams {
            screen_pos: margin + vec2(0.0, 0.0),
            text: "Hello world".into(),
            scale: 16.0,
            color: Color::WHITE,
        });

        let l = 8.0;
        let w = 1.0;
        engine.draw_line(DrawLineParams {
            begin: center + vec2(-l,0.0),
            end: center + vec2(l,0.0),
            line_width: w,
            color: Color::WHITE
        });

        engine.draw_line(DrawLineParams {
            begin: center + vec2(0.0,-l),
            end: center + vec2(0.0,l),
            line_width: w,
            color: Color::WHITE
        })
    }
}
