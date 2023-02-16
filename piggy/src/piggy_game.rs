use engine_sdk::glam::{vec3, Vec3};
use engine_sdk::{Atlas, Game, Map, Event, Scene, Camera, Grid, Sprite, SpriteType};
use engine_sdk::image;
use crate::Piggy;

impl Game for Piggy {
    fn init(&mut self, engine:&mut dyn engine_sdk::Engine) {
        macro_rules! load_texture {
            ($id:expr, $path:expr) => {
                engine.load_atlas($id, &image::load_from_memory(include_bytes!($path)).unwrap(), Atlas::default());
            };
        }

        macro_rules! load_texture2 {
            ($id:expr, $path:expr, $atlas:expr) => {
                engine.load_atlas
                ($id, &image::load_from_memory(include_bytes!($path)).unwrap(), $atlas);
            };
        }
        
        load_texture!(1, "../assets/textures/brick_wall.png");
        load_texture!(2, "../assets/textures/bush_wall.png");
        load_texture!(3, "../assets/textures/white_wall.png");
        load_texture!(4, "../assets/textures/player.png");
        load_texture!(5, "../assets/textures/viktor.png");
        load_texture2!(6, "../assets/textures/william.png", Atlas::new(2, 1));
        load_texture!(7, "../assets/textures/floor.png");
        load_texture!(8, "../assets/textures/ceiling.png");
        load_texture!(9, "../assets/textures/blue_door.png");

        let map:Map = serde_json::from_str(include_str!("../assets/maps/test.map")).unwrap();
        engine.push_event(Event::Map { map });
    }

    fn update(&mut self, engine:&mut dyn engine_sdk::Engine) {
        engine.set_cursor_visible(false);
        self.update_controls(engine);
        self.update_scene(engine);
        self.update_ui(engine);

        for (_, sprite) in self.scene.sprites.iter_mut() {
            if sprite.texture == 6 {
                sprite.atlas_index += engine.dt() * 2.0;
            }
        }
    }

    fn on_event(&mut self, _engine:&mut dyn engine_sdk::Engine, event:&Event) {
        match event {
            Event::Map { map } => {
                self.current_map = map.clone();
                dbg!("new map loaded");

                let mut scene = Scene::default();
                scene.floor_texture = 7;
                scene.ceiling_texture = 8;

                scene.grid = Grid::new(self.current_map.grid.size());
                self.current_map.grid.for_each(|cell, index| {
                    scene.grid.get_mut(index).unwrap().wall = cell.wall;
                    if let Some(thing) = cell.thing {
                        let mut sprite = Sprite {
                            pos: Vec3::new(index.0 as f32 + 0.5, index.1 as f32 + 0.5, 0.5),
                            texture: thing,
                            opacity: None,
                            facing:cell.thing_facing,
                            ..Default::default()
                        };
                        if thing == 9 {
                            sprite.sprite_type = SpriteType::Wall;
                        } else if thing == 4 {
                            sprite.sprite_type = SpriteType::Floor;
                            sprite.pos.z = 0.1;
                        }
                        scene.sprites.spawn(sprite);
                    }
                });

                let camera = Camera {
                    pos: vec3(3.5, 3.5, 0.5),
                    yaw: 0.0//PI + PI / 4.0
                };

                self.scene = scene;
                self.camera = camera;
            },
            _=>{}
        }
    }

    
}
