use engine_sdk::{Game, Scene, glam::{vec2}, Engine, Color, DrawRectParams, egui, Map, DrawLineParams, VirtualKeyCode};

use crate::{EditorCamera, Tool};

#[derive(Default)]
pub struct Editor {
    pub camera:EditorCamera,
    pub map:Map,
    pub selected_texture:u32,
    pub tool:Tool
}

impl Editor {
    pub fn update(&mut self, engine:&mut dyn Engine) {
        self.camera.update(engine);
        self.edit_map(engine);
        self.draw_map(engine);
        self.draw_grid(engine);
        self.draw_grid_cursor(engine);
        self.draw_cursor(engine);
        self.update_ui(engine);
    }

    fn draw_map(&mut self, engine:&mut dyn Engine) {
        self.map.grid.for_each(|cell, (x,y)| {
            let p = self.camera.to_screen(&vec2(x as f32, y as f32));
            if cell.wall.is_some() {
                engine.draw_rect(DrawRectParams {
                    pos: p,
                    size: (self.camera.zoom, self.camera.zoom).into(),
                    color: Color::WHITE,
                    texture: cell.wall,
                });
            }
        });
       
    }

    fn edit_map(&mut self, engine:&mut dyn Engine) {
        let _keys = engine.keys_just_pressed();
        if engine.key_down(VirtualKeyCode::Key1) {
            self.selected_texture = 1;
        } else if engine.key_down(VirtualKeyCode::Key2) {
            self.selected_texture = 2;
        }
        if let Some(cell) = self.map.grid.get_mut(self.camera.grid_cursor.into()) {
            if engine.mouse_down(0) {
                cell.wall = Some(self.selected_texture);
            } else if engine.mouse_down(1) {
                cell.wall = None;
            }
        }
    }

    fn draw_cursor(&mut self, engine:&mut dyn Engine) {
        let cursor_pos = engine.mouse_pos() + vec2(16.0, 16.0);

        if let Some(tex) = engine.texture(&self.selected_texture) {
            let s = 32.0;
            engine.draw_rect(DrawRectParams {
                pos: cursor_pos,
                size: (s, s * tex.aspect()).into(),
                color: Color::WHITE,
                texture: Some(self.selected_texture),
            });
        }
       
    }

    fn update_ui(&mut self, engine:&mut dyn Engine) {
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

        let size = 64.0;

        egui::Window::new("Toolbox").show(&ctx, |ui|{
            ui.radio_value(&mut self.tool, Tool::PlaceWall, "Wall");
            ui.radio_value(&mut self.tool, Tool::PlaceThing, "Thing");
          /*  match engine.egui_texture(&self.selected_texture) {
                Some(handle) => {
                    ui.image(handle.id(), [size, size]);
                }
                None => {
                    ui.add_space(size + 3.0);
                }
            };*/
        });

        egui::Window::new("Textures").show(&ctx, |ui|{
            egui::containers::ScrollArea::vertical().show(ui, |ui|{

                let line_length = 3;
                let mut count= 0;
                let mut texture_line = Vec::new();

                for texture in engine.textures().iter() { 
                    if count % line_length == 0 {
                        texture_line.push(Vec::new());
                    }
                    texture_line.last_mut().unwrap().push(texture.clone());
                    count+=1;
                }

                for line in texture_line {
                    ui.horizontal(|ui|{
                        for texture in line.iter() {
                            let aspect = texture.height / texture.width;
                            if let Some(handle) = engine.egui_texture(&texture.id) {
                                if ui.add(egui::ImageButton::new(handle.id(), [size, size * aspect])).clicked() {
                                    self.selected_texture = texture.id;
                                }
                            }
                        }
                    });
                } 
            });
        });

        /*egui::SidePanel::left("left_panel").show(&ctx, |ui| {
            let handle = engine.egui_texture(&1).unwrap();
            ui.image(handle.id(), [16.0,16.0]);
        });*/
    }

    fn draw_grid(&mut self, engine:&mut dyn Engine) {
        let size = self.map.grid.size();
        for x in 0..(size+1) {
            let x = x as f32;
            let start = vec2(x, 0.0);
            let end = vec2(x, size as f32);
            engine.draw_line(DrawLineParams {
                begin: self.camera.to_screen(&start),
                end: self.camera.to_screen(&end),
                line_width: 1.0,
                color: Color::WHITE,
            });
        }
        for y in 0..(size+1) {
            let y = y as f32;
            let start = vec2(0.0, y);
            let end = vec2(size as f32, y);
            engine.draw_line(DrawLineParams {
                begin: self.camera.to_screen(&start),
                end: self.camera.to_screen(&end),
                line_width: 1.0,
                color: Color::WHITE,
            });
        }
    }

    fn draw_grid_cursor(&mut self, engine:&mut dyn Engine) {
        let grid_cursor = self.camera.grid_cursor.as_vec2();
        engine.draw_rect(DrawRectParams {
            pos: self.camera.to_screen(&grid_cursor),
            size: (self.camera.zoom, self.camera.zoom).into(),
            color: Color { r: 1.0, g: 1.0, b: 1.0, a: 0.25 },
            texture: None
        });
    }
}

impl Game for Editor {
    fn init(&mut self, _engine:&mut dyn engine_sdk::Engine) {
     /*   macro_rules! load_texture {
            ($id:expr, $path:expr) => {
                engine.load_texture($id, &image::load_from_memory(include_bytes!($path)).unwrap());
            };
        }
        
        load_texture!(1, "../assets/textures/brick_wall_red.png");
        load_texture!(2, "../assets/textures/plant.png");
        load_texture!(3, "../assets/textures/viktor.png");
        load_texture!(4, "../assets/textures/william.png");*/
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