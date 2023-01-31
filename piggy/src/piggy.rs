use engine_sdk::{Game, image, Event, Map, Engine, DrawTextParams, glam::{vec2, vec3}, Color, DrawLineParams, Scene, Camera};
use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Piggy {
    pub current_map:Map
}

impl Piggy {
    pub fn update_scene(&mut self, engine:&mut dyn Engine) {

        let mut scene = Scene::default();
        for i in 0..scene.grid.size() {
            scene.grid.get_mut((i as i32, 0)).unwrap().wall = Some(1);
            scene.grid.get_mut((i as i32, scene.grid.size() as i32 - 1)).unwrap().wall = Some(1);
            scene.grid.get_mut((0, i as i32)).unwrap().wall = Some(1);
            scene.grid.get_mut((scene.grid.size() as i32 - 1, i as i32)).unwrap().wall = Some(1);
        }

        scene.grid.get_mut((2, 2)).unwrap().wall = Some(1);
       
        // draw scene
        engine.draw_scene(&Camera {
            pos: vec3(2.0, 2.0, 0.0),
        }, &scene);
    }
    pub fn update_ui(&mut self, engine:&mut dyn Engine) {

        // draw ui
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
        });
    }
}
