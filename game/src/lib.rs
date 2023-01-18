

use engine_sdk::{Game, Scene, Sprite, glam::{Vec2, vec2}, Engine, Color, DrawRectParams, image, DrawTextParams, egui, Map, DrawLineParams};

const BRICK_WALL:u32 = 1;
const PLANT:u32 = 2;
const VIKTOR:u32 = 3;
const WILLIAM:u32 = 4;

pub struct EditorCamera {
    pub pos:Vec2,
    pub zoom:f32,
    pub screen_size:Vec2
}

impl EditorCamera {
    pub fn to_screen(&self, p:Vec2) -> Vec2 {
        p * self.zoom + self.pos * self.zoom + self.screen_size / 2.0
    }
    pub fn update(&mut self, screen_size:Vec2) {
        self.screen_size = screen_size;
    }

}

impl Default for EditorCamera {
    fn default() -> Self {
        Self { pos: Default::default(), zoom: 64.0, screen_size:vec2(0.0, 0.0) }
    }
}

#[derive(Default)]
pub struct Editor {
    pub scene:Scene,
    pub iterations:u64,
    pub camera:EditorCamera,
    pub map:Map
}

impl Editor {
    pub fn update(&mut self, engine:&mut dyn Engine) {
        self.camera.update(engine.screen_size());
        self.draw_grid(engine);
       // self.ui(engine);
    }

    fn ui(&mut self, engine:&mut dyn Engine) {
        let ctx = engine.egui().clone();
        egui::TopBottomPanel::top("top_pane").show(&ctx, |ui|{
            ui.menu_button("File", |ui|{
                if ui.button("New").clicked() {
                }
                if ui.button("Save").clicked() {

                }
                if ui.button("Load").clicked() {
                    
                } 
            });
        });
    }

    fn draw_grid(&mut self, engine:&mut dyn Engine) {
        let size = self.map.grid.size();
        for x in 0..(size+1) {
            let x = x as f32;
            let start = vec2(x, 0.0);
            let end = vec2(x, size as f32);
            engine.draw_line(DrawLineParams {
                begin: self.camera.to_screen(start),
                end: self.camera.to_screen(end),
                line_width: 1.0,
                color: Color::WHITE,
            });
        }
        for y in 0..(size+1) {
            let y = y as f32;
            let start = vec2(0.0, y);
            let end = vec2(size as f32, y);
            engine.draw_line(DrawLineParams {
                begin: self.camera.to_screen(start),
                end: self.camera.to_screen(end),
                line_width: 1.0,
                color: Color::WHITE,
            });
        }
    }
}

impl Game for Editor {
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
        self.update(engine);  
      
/*
        engine.draw_rect(DrawRectParams {
            pos: vec2(5.0, 5.0),
            size: vec2(screen_size.x, screen_size.y),
            color: Color::RED,
            texture: None,
        }); 

        let mouse_pos = engine.mouse_pos();
        engine.draw_rect(DrawRectParams {
            pos: mouse_pos,
            size:vec2(24.0,48.0),
            color: Color::WHITE,
            texture: if engine.mouse_down(0) { Some(WILLIAM) } else { Some(VIKTOR)},
        });*/

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
        /*engine.draw_rect(DrawRectParams {
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

        let mouse_pos = engine.mouse_pos();
        engine.draw_rect(DrawRectParams {
            pos: mouse_pos,
            size:vec2(24.0,48.0),
            color: Color::WHITE,
            texture: if engine.mouse_down(0) { Some(WILLIAM) } else { Some(VIKTOR)},
        });


        if engine.key_just_pressed(32) {
            dbg!("pressed");
        }

        self.iterations += 1;

        if self.iterations % 60 == 0 {
            dbg!(engine.frame_time());
        }

        engine_sdk::egui::Window::new("Hello world").show(engine.egui(), |ui|{

        });*/
        
    }
} 


#[no_mangle]
pub fn create() -> Box<dyn Game> {
    Box::new(Editor {
        map:Map::new(16),
        ..Default::default()
    })
}