use std::f32::consts::PI;

use engine_sdk::{Game, Scene, Sprite, glam::{Vec2, vec2}, Engine, Camera, DrawLineParams, Color, DrawTextParams};

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
    fn init(&mut self, engine:&mut dyn engine_sdk::Engine) {
        engine.define_texture(TEST_SPRITE, "some_sprite.png".into());
    }

    fn update(&mut self, engine:&mut dyn Engine) {
        if self.scene.sprites.is_empty() {
            self.start(engine);
        }

        let _camera = Camera::default();
        //engine.draw_scene(&camera, &self.scene);
        //engine.draw_rect(0.0, 0.0, 0.5, 0.12, engine_sdk::Color::WHITE);
        //engine.draw_rect(-0.3, -0.3, 0.2, 0.2, [1.0, 0.0, 0.0, 1.0].into());

        //engine.draw_rect(25.0, 25.0, 100.0, 100.0, engine_sdk::Color::WHITE);
        //engine.draw_rect(120.0, 120.0, 100.0, 100.0, engine_sdk::Color::RED);

        //engine.draw_rect(0.0, 0.0, 10.0, 10.0, Color::WHITE);

        
     /*    let s = 1024;
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

        engine.draw_text(DrawTextParams {
            screen_pos: vec2(30.0, 30.0),
            text: "Hello world!".into(),
            scale: 40.0,
            color: Color::BLUE,
        });

        engine.draw_text(DrawTextParams {
            screen_pos: vec2(30.0, 64.0),
            text: "Søren rules alot!".into(),
            scale: 16.0,
            color: Color::BLUE,
        });
*/



        let cell_size = 128;
        let screen_size = engine.screen_size();

        fn draw_pos(engine:&mut dyn Engine, begin:Vec2, cell_size:f32) {
            let mid = begin + vec2(cell_size / 2.0, cell_size / 2.0);
            engine.draw_text(DrawTextParams {
                screen_pos: mid,
                text: begin.to_string(),
                scale: 16.0,
                color: Color::RED,
            })
        }
        
        for i in (0..screen_size.y as i32).step_by(cell_size) {
            let begin = vec2(0.0, i as f32);
            let end = vec2(screen_size.x as f32, i as f32);
            engine.draw_line(DrawLineParams {
                begin,
                end,
                line_width: 1.0,
                color: Color::WHITE,
            });

        }

        for i in (0..screen_size.x as i32).step_by(cell_size) {
            let begin = vec2(i as f32, 0.0);
            let end = vec2(i as f32, screen_size.y as f32);
            engine.draw_line(DrawLineParams {
                begin,
                end,
                line_width: 1.0,
                color: Color::WHITE,
            });
        }

        for i in (0..screen_size.y as i32).step_by(cell_size) {
            let begin = vec2(0.0, i as f32);
            let end = vec2(screen_size.x as f32, i as f32);
            draw_pos(engine, begin, cell_size as f32);
        }

        for i in (0..screen_size.x as i32).step_by(cell_size) {
            let begin = vec2(i as f32, 0.0);
            let end = vec2(i as f32, screen_size.y as f32);
            draw_pos(engine, begin, cell_size as f32);
        }

        self.iterations += 1;

        if self.iterations % 60 == 0 {
            dbg!(engine.frame_time());
        }
    }
} 