use std::f32::consts::PI;

use engine_sdk::{
    glam::vec2, Camera, Cell, Color, DrawLineParams, DrawRectParams, Engine, Entities, Grid, Map,
    Scene, Sprite, SpriteId, VirtualKeyCode,
};
use serde::{Deserialize, Serialize};

use crate::textures;

#[derive(Default, Serialize, Deserialize)]
pub struct Piggy {
    pub current_map: Map,
    pub camera: Camera,
    pub sprites: Entities<SpriteId, Sprite>,
    pub grid: Grid<Cell>,
}

impl Piggy {
    pub fn update_controls(&mut self, engine: &mut dyn Engine) {
        let dt = engine.dt();
        let speed = 3.0;
        let left = self.camera.left();
        let forward = self.camera.forward_body();
        if engine.key_down(VirtualKeyCode::A) {
            self.camera.pos += speed * dt * left;
        }
        if engine.key_down(VirtualKeyCode::D) {
            self.camera.pos -= speed * dt * left;
        }
        if engine.key_down(VirtualKeyCode::W) {
            self.camera.pos += speed * dt * forward;
        }
        if engine.key_down(VirtualKeyCode::S) {
            self.camera.pos -= speed * dt * forward;
        }

        let turn_speed = PI / 4.0;
        self.camera.yaw += turn_speed * dt * engine.mouse_motion().x;
    }
    pub fn update_scene(&mut self, engine: &mut dyn Engine) {
        let s = 16.0;
        for y in 0..self.grid.size() as i32 {
            for x in 0..self.grid.size() as i32 {
                let cell = self.grid.get((x, y)).unwrap();
                let p = vec2(x as f32, y as f32);
                if cell.wall.is_some() {
                    engine.draw_rect(DrawRectParams {
                        pos: p * s,
                        size: vec2(s, s),
                        color: Color::BLACK,
                        texture: None,
                        ..Default::default()
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
            ..Default::default()
        });

        let p2 = p + self.camera.forward_body().truncate() * s * 2.0;
        engine.draw_line(DrawLineParams {
            begin: p,
            end: p2,
            line_width: 1.0,
            color: Color::RED,
            ..Default::default()
        });

        // draw scene
        engine.draw_scene(
            &self.camera,
            &Scene {
                sprites: &mut self.sprites,
                ceiling_texture: textures::CEILING_GREY,
                floor_texture: textures::FLOOR_GREY,
                grid: &mut self.grid,
            },
        );
    }
    pub fn update_ui(&mut self, engine: &mut dyn Engine) {
        // draw ui
        let _margin = vec2(16.0, 16.0);
        let center = engine.screen_size() / 2.0;

        let l = 8.0;
        let w = 1.0;
        engine.draw_line(DrawLineParams {
            begin: center + vec2(-l, 0.0),
            end: center + vec2(l, 0.0),
            line_width: w,
            color: Color::WHITE,
        });

        engine.draw_line(DrawLineParams {
            begin: center + vec2(0.0, -l),
            end: center + vec2(0.0, l),
            line_width: w,
            color: Color::WHITE,
        });
    }
}
