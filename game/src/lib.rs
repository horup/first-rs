use engine_sdk::{Game, Scene, Sprite, glam::Vec2, Engine, Camera};

const TEST_SPRITE:u32 = 0;

#[derive(Default)]
pub struct MyGame {
    pub scene:Scene,
    pub iterations:u64
}

impl MyGame {
    pub fn start(&mut self, engine:&mut dyn Engine) {
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

        let camera = Camera::default();
        //engine.draw_scene(&camera, &self.scene);
        engine.draw_rect(0.0, 0.0, 1.0, 1.0);

        self.iterations += 1;

        if self.iterations % 60 == 0 {
            dbg!(engine.frame_time());
        }
    }

    fn init(&mut self, engine:&mut dyn engine_sdk::Engine) {
        engine.define_texture(TEST_SPRITE, "some_sprite.png".into());
    }
} 