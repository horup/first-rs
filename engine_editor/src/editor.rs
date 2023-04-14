use std::f32::consts::PI;

use engine_sdk::{Game, glam::{vec2}, Engine, Color, DrawRectParams, egui::{self, Rect}, Map, DrawLineParams, DrawTextParams, VirtualKeyCode, Rect2, AtlasDef};
use serde::{Serialize, Deserialize};

use crate::{EditorCamera, Tool};

#[derive(Default, Serialize, Deserialize)]
pub struct Editor {
    pub camera:EditorCamera,
    pub map:Map,
    pub tool:Tool,
    pub walls:Vec<AtlasDef>,
    pub entities:Vec<AtlasDef>,
    pub selected_wall:Option<AtlasDef>,
    pub selected_entity:Option<AtlasDef>,
    pub show_atlas_def_selector:bool
}

impl Editor {
    pub fn selected_atlas_def(&self) -> Option<AtlasDef> {
        match self.tool {
            Tool::PlaceWall => self.selected_wall.clone(),
            Tool::PlaceThing => self.selected_entity.clone(),
        }
    }

    pub fn update(&mut self, engine:&mut dyn Engine) {
        engine.set_cursor_grabbed(true);
        if engine.key_just_pressed(VirtualKeyCode::T) {
            self.show_atlas_def_selector = !self.show_atlas_def_selector;
        }

        if self.show_atlas_def_selector {
            let atlas_defs = match self.tool {
                Tool::PlaceWall => &self.walls,
                Tool::PlaceThing => &self.entities,
            };
            let selected = self.atlas_def_selector(engine, &atlas_defs);
            if let Some(selected) = selected {
                match self.tool {
                    Tool::PlaceWall => self.selected_wall = Some(selected),
                    Tool::PlaceThing => self.selected_entity = Some(selected),
                }
            }
        } else {
            self.camera.update(engine);
            self.edit_map(engine);
            self.draw_map(engine);
            self.draw_grid(engine);
            self.draw_grid_cursor(engine);
            self.draw_cursor(engine);
            self.update_ui(engine);
        }
    }

    fn atlas_def_selector(&self, engine:&mut dyn Engine, atlas_defs:&[AtlasDef]) -> Option<AtlasDef> {
        let mut selected:Option<AtlasDef> = None;
        let screen_size = engine.screen_size();
        let mut pos = vec2(0.0, 0.0);
        let scale = 2.0;
        let mouse_pos = engine.mouse_pos();
        for a in atlas_defs.iter() {
            if let Some(atlas) = engine.atlas(&a.atlas) {
                let s = vec2(atlas.width(a.atlas_index) as f32, atlas.height(a.atlas_index) as f32) * scale;
                if pos.x + s.x > screen_size.x {
                    pos.x = 0.0;
                    pos.y += s.y;
                }

                engine.draw_rect(DrawRectParams {
                    pos:pos,
                    size:s,
                    atlas_index:a.atlas_index as f32,
                    texture:Some(a.atlas),
                    ..Default::default()
                });

                let r = Rect2 {
                    x: pos.x,
                    y: pos.y,
                    w: s.x,
                    h: s.y,
                };

                if r.contains(&mouse_pos) {
                    engine.draw_rect(DrawRectParams {
                        pos:pos,
                        size:s,
                        color:Color { r: 1.0, g: 1.0, b: 1.0, a: 0.2 },
                        ..Default::default()
                    });

                    if engine.mouse_down(0) {
                        selected = Some(a.clone());
                    }
                }

                pos.x += s.x;
                if pos.x + s.x > screen_size.x {
                    pos.x = 0.0;
                    pos.y += s.y;
                }
            }
        }

        selected
    }

    fn draw_map(&mut self, engine:&mut dyn Engine) {
        // draw walls
        self.map.grid.for_each(|cell, (x,y)| {
            let p = self.camera.to_screen(&vec2(x as f32, y as f32));
            if cell.wall.is_some() {
                engine.draw_rect(DrawRectParams {
                    pos: p,
                    size: (self.camera.zoom, self.camera.zoom).into(),
                    color: Color::WHITE,
                    texture: cell.wall,
                    atlas_index: cell.wall_index as f32,
                    ..Default::default()
                });
            }
        });

        // draw things
        self.map.grid.for_each(|cell, (x,y)| {
            let center = self.camera.to_screen(&vec2(x as f32 + 0.5, y as f32 + 0.5));
            let size = vec2(self.camera.zoom, self.camera.zoom);
            let p = center - size/2.0;
            if cell.thing.is_some() {
                let _ps = [vec2(p.x, p.y), vec2(p.x + size.x, p.y), vec2(p.x + size.x, p.y + size.y), vec2(p.x, p.y + size.y)];

                /*for i in 0..ps.len() {
                    let p1 = ps[i];
                    let p2 = ps[(i+1)% ps.len()];
                    engine.draw_line(DrawLineParams {
                        begin: p1,
                        end: p2,
                        line_width: 1.0,
                        color: Color::RED,
                    });
                }*/

                
                engine.draw_rect(DrawRectParams {
                    pos: p,
                    size,
                    color: Color::WHITE,
                    texture: cell.thing,
                    ..Default::default()
                });

                let v = vec2(cell.thing_facing.cos(), cell.thing_facing.sin()) * size.x / 2.0;
                engine.draw_line(DrawLineParams {
                    begin: center,
                    end: center + v,
                    line_width: 1.0,
                    color: Color::RED,
                });
            }
        });
       
    }

    fn edit_map(&mut self, engine:&mut dyn Engine) {
        if let Some(def) = self.selected_atlas_def() {
            match self.tool {
                Tool::PlaceWall => {
                    if let Some(cell) = self.map.grid.get_mut(self.camera.grid_cursor.into()) {
                        if engine.mouse_down(0) {
                            cell.wall = Some(def.atlas);
                            cell.wall_index = def.atlas_index;
                        } else if engine.mouse_down(1) {
                            cell.wall = None;
                        }
                    }
                },
                Tool::PlaceThing => {
                    if let Some(cell) = self.map.grid.get_mut(self.camera.grid_cursor.into()) {
                        if engine.mouse_down(0) {
                            cell.thing = Some(def.atlas);
                        } else if engine.mouse_down(1) {
                            cell.thing = None;
                        }
                    }
                },
            }
            
            if let Some(cell) = self.map.grid.get_mut(self.camera.grid_cursor.into()) {
                if engine.key_down(VirtualKeyCode::Up) {
                    cell.thing_facing = PI / 2.0 * 3.0;
                } else if  engine.key_down(VirtualKeyCode::Down) {
                    cell.thing_facing = PI / 2.0;
                } else if engine.key_down(VirtualKeyCode::Left) {
                    cell.thing_facing = PI;
                } else if  engine.key_down(VirtualKeyCode::Right) {
                    cell.thing_facing = 0.0;
                }
            }
        }
    }

    fn draw_cursor(&mut self, engine:&mut dyn Engine) {
        let cursor_pos = engine.mouse_pos() + vec2(16.0, 16.0);
        if let Some(def) = self.selected_atlas_def() {
            if let Some(atlas) = engine.atlas(&def.atlas) {
                let s = 32.0;
                engine.draw_text(DrawTextParams {
                    screen_pos: cursor_pos - vec2(0.0, 12.0),
                    text: match self.tool {
                        Tool::PlaceWall => "Wall",
                        Tool::PlaceThing => "Thing",
                    }.to_string(),
                    scale: 12.0,
                    color: Color::WHITE,
                    ..Default::default()
                });
                engine.draw_rect(DrawRectParams {
                    pos: cursor_pos,
                    size: (s, s * atlas.aspect(def.atlas_index)).into(),
                    color: Color::WHITE,
                    texture: Some(atlas.id()),
                    atlas_index: def.atlas_index as f32,
                    ..Default::default()
                });
            }
        }
    }

    fn update_ui(&mut self, engine:&mut dyn Engine) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.native_update_ui(engine);
        }

        let size = 64.0;
        let ctx = engine.egui().clone();

        egui::Window::new("Toolbox").show(&ctx, |ui|{
            ui.radio_value(&mut self.tool, Tool::PlaceWall, Tool::PlaceWall.to_string());
            ui.radio_value(&mut self.tool, Tool::PlaceThing, Tool::PlaceThing.to_string()); 
        });

        /*egui::Window::new("Textures").show(&ctx, |ui|{
            egui::containers::ScrollArea::vertical().show(ui, |ui|{
                let line_length = 3;
                let mut count= 0;
                let mut texture_line = Vec::new();

                for texture in engine.atlases().iter() { 
                    if count % line_length == 0 {
                        texture_line.push(Vec::new());
                    }

                   /* match self.tool {
                        Tool::PlaceWall => if !texture.editor_props().is_wall { continue; },
                        Tool::PlaceThing => if !texture.editor_props().is_thing { continue; },
                    }*/

                    texture_line.last_mut().unwrap().push(texture.clone());
                    count+=1;
                }

                for line in texture_line {
                    ui.horizontal(|ui|{
                        for texture in line.iter() {
                            let aspect = texture.aspect(0);
                            if let Some(handle) = engine.egui_texture(&texture.id()) {
                                if ui.add(egui::ImageButton::new(handle.id(), [size, size * aspect])).clicked() {
                                    self.selected_texture = texture.id();
                                }
                            }
                        }
                    });
                } 
            });
        });*/
    }

    fn draw_grid(&mut self, engine:&mut dyn Engine) {
        let size = self.map.grid.size();
        let color:Color = [1.0, 1.0, 1.0, 0.1].into();
        for x in 0..(size+1) {
            let x = x as f32;
            let start = vec2(x, 0.0);
            let end = vec2(x, size as f32);
            engine.draw_line(DrawLineParams {
                begin: self.camera.to_screen(&start),
                end: self.camera.to_screen(&end),
                line_width: 1.0,
                color,
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
                color,
            });
        }
    }

    fn draw_grid_cursor(&mut self, engine:&mut dyn Engine) {
        let grid_cursor = self.camera.grid_cursor.as_vec2();
        engine.draw_rect(DrawRectParams {
            pos: self.camera.to_screen(&grid_cursor),
            size: (self.camera.zoom, self.camera.zoom).into(),
            color: Color { r: 1.0, g: 1.0, b: 1.0, a: 0.25 },
            texture: None,
            ..Default::default()
        });
    }
}

impl Game for Editor {
    fn deserialize(&mut self, bytes:&Vec<u8>) {
        *self = bincode::deserialize(bytes).unwrap()
    }

    fn serialize(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }
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
    }

    fn on_event(&mut self, _engine:&mut dyn Engine, event:&engine_sdk::Event) {
        match event {
            engine_sdk::Event::Map { map } => {
                self.map = map.clone();
            },
            _ => {},
        }
    }
} 