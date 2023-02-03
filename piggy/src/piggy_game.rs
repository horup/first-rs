use std::f32::consts::PI;
use engine_sdk::glam::vec3;
use engine_sdk::{Game, Map, Event, Scene, Camera, CursorGrabMode, Grid};
use engine_sdk::image;
use crate::Piggy;


impl Game for Piggy {
    fn init(&mut self, engine:&mut dyn engine_sdk::Engine) {
        macro_rules! load_texture {
            ($id:expr, $path:expr) => {
                engine.load_texture($id, &image::load_from_memory(include_bytes!($path)).unwrap());
            };
        }
        
        load_texture!(1, "../assets/textures/brick_wall.png");
        load_texture!(2, "../assets/textures/bush_wall.png");
        load_texture!(3, "../assets/textures/white_wall.png");
        load_texture!(4, "../assets/textures/player.png");
        load_texture!(5, "../assets/textures/viktor.png");
        load_texture!(6, "../assets/textures/william.png");

        let map:Map = serde_json::from_str(include_str!("../assets/maps/test.map")).unwrap();
        engine.push_event(Event::Map { map });
    }

    fn update(&mut self, engine:&mut dyn engine_sdk::Engine) {
        engine.set_cursor_visible(false);
        self.update_controls(engine);
        self.update_scene(engine);
        self.update_ui(engine);
    }

    fn on_event(&mut self, _engine:&mut dyn engine_sdk::Engine, event:&Event) {
        match event {
            Event::Map { map } => {
                self.current_map = map.clone();
                dbg!("new map loaded");

                let mut scene = Scene::default();
                /*for i in 0..scene.grid.size() {
                    scene.grid.get_mut((i as i32, 0)).unwrap().wall = Some(1);
                    scene.grid.get_mut((i as i32, scene.grid.size() as i32 - 1)).unwrap().wall = Some(1);
                    scene.grid.get_mut((0, i as i32)).unwrap().wall = Some(1);
                    scene.grid.get_mut((scene.grid.size() as i32 - 1, i as i32)).unwrap().wall = Some(1);
                }

                scene.grid.get_mut((2, 2)).unwrap().wall = Some(1);*/

                scene.grid = Grid::new(self.current_map.grid.size());
                self.current_map.grid.for_each(|cell, index| {
                    scene.grid.get_mut(index).unwrap().wall = cell.wall;
                });

                let camera = Camera {
                    pos: vec3(8.0, 8.0, 0.5),
                    yaw: PI + PI / 4.0
                };

                self.scene = scene;
                self.camera = camera;
            },
            _=>{}
        }
    }

    
}
