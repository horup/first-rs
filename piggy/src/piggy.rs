use std::f32::consts::PI;

use engine_sdk::{Game, image, Event, Map, Engine, DrawTextParams, glam::{vec2, vec3}, Color, DrawLineParams, Scene, Camera, DrawRectParams};
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

        // debug draw scene and camera
        let camera = Camera {
            pos: vec3(8.0, 8.0, 0.5),
            yaw: PI + PI / 4.0
        };
        let s = 16.0;
        for y in 0..scene.grid.size() as i32 {
            for x in 0..scene.grid.size() as i32 {
                let cell = scene.grid.get((x, y)).unwrap();
                let p = vec2(x as f32, y as f32);
                if let Some(wall) = cell.wall {
                    engine.draw_rect(DrawRectParams {
                        pos: p * s,
                        size: vec2(s, s),
                        color: Color::BLACK,
                        texture: None,
                    })
                }
            }
        }

        let p = vec2(camera.pos.x, camera.pos.y) * s;
        let s = vec2(s, s) / 2.0;
        engine.draw_rect(DrawRectParams {
            pos: p - s / 2.0,
            size: s,
            color: Color::WHITE,
            texture: None,
        });

        let p2 = p + vec2(camera.yaw.cos(), camera.yaw.sin()) * s * 2.0;
        engine.draw_line(DrawLineParams {
            begin: p,
            end: p2,
            line_width: 1.0,
            color: Color::RED,
        });

       
        // draw scene
        engine.draw_scene(&camera, &scene);
    }
    pub fn update_ui(&mut self, engine:&mut dyn Engine) {

        // draw ui
        let margin = vec2(16.0, 16.0);
        let center = engine.screen_size() / 2.0;
        /*engine.draw_text(DrawTextParams {
            screen_pos: margin + vec2(0.0, 0.0),
            text: "Hello world".into(),
            scale: 16.0,
            color: Color::WHITE,
        });*/

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
