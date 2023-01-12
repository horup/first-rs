

use engine_sdk::{Game, Scene, Sprite, glam::{Vec2, vec2}, Engine, Camera, DrawLineParams, Color, DrawTextParams, DrawRectParams, image};

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
        let img = image::load_from_memory(include_bytes!("../assets/textures/test.png")).unwrap();
        engine.load_texture(TEST_SPRITE, &img);
    }

    fn update(&mut self, engine:&mut dyn Engine) {
        if self.scene.sprites.is_empty() {
            self.start(engine);
        }

        let _camera = Camera::default();
        let cell_size = 128;
        let screen_size = engine.screen_size();

     /*   fn draw_pos(engine:&mut dyn Engine, begin:Vec2, cell_size:f32) {
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
            let end = vec2(screen_size.x, i as f32);
            engine.draw_line(DrawLineParams {
                begin,
                end,
                line_width: 1.0,
                color: Color::WHITE,
            });

        }

        for i in (0..screen_size.x as i32).step_by(cell_size) {
            let begin = vec2(i as f32, 0.0);
            let end = vec2(i as f32, screen_size.y);
            engine.draw_line(DrawLineParams {
                begin,
                end,
                line_width: 1.0,
                color: Color::WHITE,
            });
        }

        for i in (0..screen_size.y as i32).step_by(cell_size) {
            let begin = vec2(0.0, i as f32);
            let _end = vec2(screen_size.x, i as f32);
            draw_pos(engine, begin, cell_size as f32);
        }

        for i in (0..screen_size.x as i32).step_by(cell_size) {
            let begin = vec2(i as f32, 0.0);
            let _end = vec2(i as f32, screen_size.y);
            draw_pos(engine, begin, cell_size as f32);
        }
*/
        engine.draw_rect(DrawRectParams {
            pos: vec2(25.0, 25.2),
            size: vec2(160.1, 60.1),
            color: Color::WHITE,
        });


        self.iterations += 1;

        if self.iterations % 60 == 0 {
            dbg!(engine.frame_time());
        }
    }
} 