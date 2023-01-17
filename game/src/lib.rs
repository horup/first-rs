

use engine_sdk::{Game, Scene, Sprite, glam::{Vec2, vec2}, Engine, Camera, Color, DrawRectParams, image, DrawTextParams};

const BRICK_WALL:u32 = 1;
const PLANT:u32 = 2;
const VIKTOR:u32 = 3;
const WILLIAM:u32 = 4;

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
                        tex:VIKTOR
                    };
                    self.scene.sprites.push(sprite);
                }
            }

            dbg!(self.scene.sprites.len());
    }
}

impl Game for MyGame {
    fn init(&mut self, engine:&mut dyn engine_sdk::Engine) {
        macro_rules! load_texture {
            ($id:expr, $path:expr) => {
                engine.load_texture($id, &image::load_from_memory(include_bytes!($path)).unwrap());
            };
        }
        
        load_texture!(BRICK_WALL, "../assets/textures/brick_wall_red.png");
        load_texture!(PLANT, "../assets/textures/plant.png");
        load_texture!(VIKTOR, "../assets/textures/viktor.png");
        load_texture!(WILLIAM, "../assets/textures/william.png");
    }

    fn update(&mut self, engine:&mut dyn Engine) {
        if self.scene.sprites.is_empty() {
            self.start(engine);
        }

        let _camera = Camera::default();
        let _cell_size = 128;
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
            pos: vec2(0.0, 0.0),
            size: vec2(screen_size.x, screen_size.y),
            color: Color::RED,
            texture: None,
        });

        let textures = [WILLIAM, VIKTOR, PLANT, BRICK_WALL];
        let mut x = 16.0;
        let y = 16.0;
        let scale = 3.0;
        for id in textures {
            let tex = engine.texture_info(&id).unwrap();
            let size = vec2(tex.width, tex.height) * scale;
            engine.draw_rect(DrawRectParams {
                pos: vec2(x, y),
                size,
                color: Color::WHITE,
                texture: Some(id),
            });

            x += size.x;
        }

        let s = 1;
        for y in 0..s {
            for x in 0..s {
                let spacing = 24.0;
                let x = x as f32;
                let y = y as f32;
                engine.draw_rect(DrawRectParams {
                    pos: vec2(x * spacing, y * spacing),
                    size:vec2(spacing,spacing),
                    color: Color::WHITE,
                    texture: Some(WILLIAM),
                });
            }
        }

        let mouse_pos = engine.mouse_pos();
        engine.draw_rect(DrawRectParams {
            pos: mouse_pos,
            size:vec2(24.0,48.0),
            color: Color::WHITE,
            texture: if engine.mouse_down(0) { Some(WILLIAM) } else { Some(VIKTOR)},
        });

        engine.draw_text(DrawTextParams {
            screen_pos: mouse_pos,
            text: mouse_pos.to_string(),
            scale:16.0,
            color: Color::WHITE,
        });


        if engine.key_just_pressed(32) {
            dbg!("pressed");
        }

        self.iterations += 1;

        if self.iterations % 60 == 0 {
            dbg!(engine.frame_time());
        }

        engine_sdk::egui::Window::new("Hello world").show(engine.egui(), |ui|{

        });
        
    }
} 


#[no_mangle]
pub fn create() -> Box<dyn Game> {
    Box::new(MyGame::default())
}