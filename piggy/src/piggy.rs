use std::f32::consts::PI;

use engine_sdk::{Game, image, Event, Map, Engine, DrawTextParams, glam::{vec2, vec3}, Color, DrawLineParams, Scene, Camera, DrawRectParams, VirtualKeyCode};
use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Piggy {
    pub current_map:Map,
    pub camera:Camera,
    pub scene:Scene
}

impl Piggy {
    pub fn update_controls(&mut self, engine:&mut dyn Engine) {
        let dt = engine.dt();
        let speed = 1.0;
        if engine.key_down(VirtualKeyCode::A) {
            self.camera.pos.x -= speed * dt;
        }
        if engine.key_down(VirtualKeyCode::D) {
            self.camera.pos.x += speed * dt;
        }
        if engine.key_down(VirtualKeyCode::W) {
            self.camera.pos.y -= speed * dt;
        }
        if engine.key_down(VirtualKeyCode::S) {
            self.camera.pos.y += speed * dt;
        }

        if engine.mouse_down(0) {
            self.camera.yaw -= speed * dt;
        } 
        if engine.mouse_down(1) {
            self.camera.yaw += speed * dt;
        }
        
    }
    pub fn update_scene(&mut self, engine:&mut dyn Engine) {
        let s = 16.0;
        for y in 0..self.scene.grid.size() as i32 {
            for x in 0..self.scene.grid.size() as i32 {
                let cell = self.scene.grid.get((x, y)).unwrap();
                let p = vec2(x as f32, y as f32);
                if let Some(_) = cell.wall {
                    engine.draw_rect(DrawRectParams {
                        pos: p * s,
                        size: vec2(s, s),
                        color: Color::BLACK,
                        texture: None,
                    })
                }
            }
        }

        let p = vec2(self.camera.pos.x, self.camera.pos.y) * s;
        let s = vec2(s, s) / 2.0;
        engine.draw_rect(DrawRectParams {
            pos: p - s / 2.0,
            size: s,
            color: Color::WHITE,
            texture: None,
        });

        let p2 = p + vec2(self.camera.yaw.cos(), self.camera.yaw.sin()) * s * 2.0;
        engine.draw_line(DrawLineParams {
            begin: p,
            end: p2,
            line_width: 1.0,
            color: Color::RED,
        });
       
        // draw scene
        engine.draw_scene(&self.camera, &self.scene);
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
