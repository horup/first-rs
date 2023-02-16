use engine_sdk::glam::{vec3, Vec3};
use engine_sdk::{EditorProps, Atlas, Game, Map, Event, Scene, Camera, Grid, Sprite, SpriteType, LoadAtlasParams};
use engine_sdk::image;
use crate::{Piggy, textures};

impl Game for Piggy {
    fn init(&mut self, engine:&mut dyn engine_sdk::Engine) {
        macro_rules! load_atlas {
            ($id:expr, $path:expr) => {
                engine.load_atlas($id, &image::load_from_memory(include_bytes!($path)).unwrap(), LoadAtlasParams {
                    editor_props:EditorProps::wall(),
                    ..Default::default()
                });
            };
        }

        macro_rules! load_atlas2 {
            ($id:expr, $path:expr, $atlas:expr) => {
                engine.load_atlas
                ($id, &image::load_from_memory(include_bytes!($path)).unwrap(), LoadAtlasParams {
                    atlas:$atlas,
                    editor_props:EditorProps::thing(),
                    ..Default::default()
                });
            };
        }
        
        load_atlas!(textures::WALL_BRICK, "../assets/textures/brick_wall.png");
        load_atlas!(textures::WALL_BUSH, "../assets/textures/bush_wall.png");
        load_atlas!(textures::WALL_WHITE, "../assets/textures/white_wall.png");
        load_atlas2!(textures::THING_PLAYER, "../assets/textures/player.png", Atlas::new(1, 1));
        load_atlas2!(textures::THING_VIKTOR, "../assets/textures/viktor.png", Atlas::new(1, 1));
        load_atlas2!(textures::THING_WILLIAM, "../assets/textures/william.png", Atlas::new(2, 1));
        load_atlas!(textures::FLOOR_GREY, "../assets/textures/floor.png");
        load_atlas!(textures::CEILING_GREY, "../assets/textures/ceiling.png");
        load_atlas2!(textures::DOOR_BLUE, "../assets/textures/blue_door.png", Atlas::new(1, 1));

        let map:Map = serde_json::from_str(include_str!("../assets/maps/test.map")).unwrap();
        engine.push_event(Event::Map { map });
    }

    fn update(&mut self, engine:&mut dyn engine_sdk::Engine) {
        engine.set_cursor_visible(false);
        self.update_controls(engine);
        self.update_scene(engine);
        self.update_ui(engine);

        for (_, sprite) in self.sprites.iter_mut() {
            if sprite.texture == 6 {
                sprite.atlas_index += engine.dt() * 2.0;
            }
        }
    }

    fn on_event(&mut self, _engine:&mut dyn engine_sdk::Engine, event:&Event) {
        match event {
            Event::Map { map } => {
                self.current_map = map.clone();
                self.grid = Grid::new(self.current_map.grid.size());
                self.current_map.grid.for_each(|cell, index| {
                    self.grid.get_mut(index).unwrap().wall = cell.wall;
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
                        self.sprites.spawn(sprite);
                    }
                });

                let camera = Camera {
                    pos: vec3(3.5, 3.5, 0.5),
                    yaw: 0.0//PI + PI / 4.0
                };

                self.camera = camera;
            },
            _=>{}
        }
    }

    
}
