use std::f32::consts::PI;

use engine_sdk::{Game, Scene, Sprite, glam::Vec2, Engine, Camera, DrawLineParams, Color};

const TEST_SPRITE:u32 = 0;

#[derive(Default)]
pub struct MyGame {
    pub scene:Scene,
    pub iterations:u64
}

impl MyGame {
    pub fn start(&mut self, _engine:&mut dyn Engine) {
        let size = 16;
            for y in 0..size {
                for x in 0..size {
                    let s = 1.0 / size as f32;
                    let sprite = Sprite {
                        pos:Vec2::new(x as f32 * s,  y as f32 * s).extend(0.0),
                        size:s * 0.8,
                        tex:TEST_SPRITE
                    };
                    self.scene.sprites.push(sprite);
                }
            }

            dbg!(self.scene.sprites.len());
    }
}

impl Game for MyGame {
    fn update(&mut self, engine:&mut dyn Engine) {
        if self.scene.sprites.len() == 0 {
            self.start(engine);
        }

        let _camera = Camera::default();
        //engine.draw_scene(&camera, &self.scene);
        //engine.draw_rect(0.0, 0.0, 0.5, 0.12, engine_sdk::Color::WHITE);
        //engine.draw_rect(-0.3, -0.3, 0.2, 0.2, [1.0, 0.0, 0.0, 1.0].into());

        //engine.draw_rect(25.0, 25.0, 100.0, 100.0, engine_sdk::Color::WHITE);
        //engine.draw_rect(120.0, 120.0, 100.0, 100.0, engine_sdk::Color::RED);

        //engine.draw_rect(0.0, 0.0, 10.0, 10.0, Color::WHITE);

        let s = 8;
        let mut center = engine.screen_size() / 2.0;
        center.y = -center.y;
        for i in 0..s {
            let a = i as f32;
            let a = a / s as f32;
            let a = PI * 2.0 * a;
            let d = 100.0 + a * 50.0;
            engine.draw_line(DrawLineParams {
                begin:center,
                end:center + Vec2::new(a.cos() * d, a.sin() * d),
                line_width:2.0
            });
        }
       

        self.iterations += 1;

        if self.iterations % 60 == 0 {
            dbg!(engine.frame_time());
        }
    }

    fn init(&mut self, engine:&mut dyn engine_sdk::Engine) {
        engine.define_texture(TEST_SPRITE, "some_sprite.png".into());
    }
} 