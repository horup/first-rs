
use engine_sdk::{
    image,
    glam::{vec2, Vec3, vec3}, EditorProps, Camera, Color, DrawLineParams, LoadAtlasParams, Atlas, DrawRectParams, Engine, Grid, Map, Sprite, Event, SpriteType, Game,
};
use serde::{Deserialize, Serialize};
use crate::{textures, State, systems, components::{Item, Door}};


#[derive(Default, Serialize, Deserialize)]
pub struct Piggy {
    pub current_map: Map,
    pub state:State
}

impl Piggy {
    
    pub fn update_scene(&mut self, engine: &mut dyn Engine) {
        // self.draw_map(engine);

        let cam = self.state.camera;
        // draw scene
        engine.draw_scene(
            &cam,
            &self.state.as_world(),
        );
    }

    fn _draw_map(&mut self, engine: &mut dyn Engine) {
        let s = 16.0;
        for y in 0..self.state.grid.size() as i32 {
            for x in 0..self.state.grid.size() as i32 {
                let cell = self.state.grid.get((x, y)).unwrap();
                let p = vec2(x as f32, y as f32);
                if cell.wall.is_some() {
                    engine.draw_rect(DrawRectParams {
                        pos: p * s,
                        size: vec2(s, s),
                        color: Color::BLACK,
                        texture: None,
                        ..Default::default()
                    })
                }
            }
        }

        let p = vec2(self.state.camera.pos.x, self.state.camera.pos.y) * s;
        let s = vec2(s, s) / 2.0;
        engine.draw_rect(DrawRectParams {
            pos: p - s / 2.0,
            size: s,
            color: Color::WHITE,
            texture: None,
            ..Default::default()
        });

        let p2 = p + self.state.camera.forward_body().truncate() * s * 2.0;
        engine.draw_line(DrawLineParams {
            begin: p,
            end: p2,
            line_width: 1.0,
            color: Color::RED,
            ..Default::default()
        });
    }

    pub fn update_ui(&mut self, engine: &mut dyn Engine) {
        // draw ui
        let _margin = vec2(16.0, 16.0);
        let center = engine.screen_size() / 2.0;

        let l = 8.0;
        let w = 1.0;
        engine.draw_line(DrawLineParams {
            begin: center + vec2(-l, 0.0),
            end: center + vec2(l, 0.0),
            line_width: w,
            color: Color::WHITE,
        });

        engine.draw_line(DrawLineParams {
            begin: center + vec2(0.0, -l),
            end: center + vec2(0.0, l),
            line_width: w,
            color: Color::WHITE,
        });
    }
}

impl Game for Piggy {
    fn init(&mut self, engine:&mut dyn engine_sdk::Engine) {
        macro_rules! wall {
            ($id:expr, $path:expr) => {
                engine.load_atlas($id, &image::load_from_memory(include_bytes!($path)).unwrap(), LoadAtlasParams {
                    editor_props:EditorProps::wall(),
                    ..Default::default()
                });
            };
        }

        macro_rules! thing {
            ($id:expr, $path:expr, $atlas:expr) => {
                engine.load_atlas
                ($id, &image::load_from_memory(include_bytes!($path)).unwrap(), LoadAtlasParams {
                    atlas:$atlas,
                    editor_props:EditorProps::thing(),
                    ..Default::default()
                });
            };
        }
        
        wall!(textures::WALL_BRICK, "../assets/textures/wall_brick.png");
        wall!(textures::WALL_BUSH, "../assets/textures/wall_bush.png");
        wall!(textures::WALL_WHITE, "../assets/textures/wall_white.png");

        thing!(textures::THING_MARKER_SPAWN_PLAYER, "../assets/textures/thing_player.png", Atlas::new(1, 1));
        thing!(textures::THING_VIKTOR, "../assets/textures/thing_player_viktor.png", Atlas::new(1, 1));
        thing!(textures::THING_WILLIAM, "../assets/textures/thing_player_william.png", Atlas::new(2, 1));
        thing!(textures::THING_DOOR_BLUE, "../assets/textures/thing_door_blue.png", Atlas::new(1, 1));
        thing!(textures::THING_DOOR_WHITE, "../assets/textures/thing_door_white.png", Atlas::new(1, 1));
        thing!(textures::THING_DOOR_GOLD, "../assets/textures/thing_door_gold.png", Atlas::new(1, 1));
        thing!(textures::THING_ITEM_POKEMONCARD, "../assets/textures/thing_item_pokemoncard.png", Atlas::new(1, 1));
        thing!(textures::THING_ITEM_KEY_GOLD, "../assets/textures/thing_item_key_gold.png", Atlas::new(1, 1));
        thing!(textures::THING_ITEM_KEY_BLUE, "../assets/textures/thing_item_key_blue.png", Atlas::new(1, 1));
        thing!(textures::THING_MONSTER_PIGGY, "../assets/textures/thing_monster_piggy.png", Atlas::new(1, 1));
        thing!(textures::THING_PLANT, "../assets/textures/thing_plant.png", Atlas::new(1, 1));
        thing!(textures::THING_MARKER_EXIT, "../assets/textures/thing_marker_exit.png", Atlas::new(1, 1));
        wall!(textures::FLOOR_GREY, "../assets/textures/floor_grey.png");
        wall!(textures::CEILING_GREY, "../assets/textures/ceiling_grey.png");

        let map:Map = serde_json::from_str(include_str!("../assets/maps/test.map")).unwrap();
        engine.push_event(Event::Map { map });
    }

    fn update(&mut self, engine:&mut dyn engine_sdk::Engine) {
        engine.set_cursor_visible(false);
        systems::player_system(&mut self.state, engine);
        systems::proximity_system(&mut self.state, engine);
        systems::door_system(&mut self.state, engine);
        self.update_scene(engine);
        systems::render_system(&mut self.state, engine);
        self.update_ui(engine);

        for (_, sprite) in self.state.sprites.iter_mut() {
            if sprite.texture == 6 {
                sprite.atlas_index += engine.dt() * 2.0;
            }
        }
    }

    fn on_event(&mut self, _engine:&mut dyn engine_sdk::Engine, event:&Event) {
        match event {
            Event::Map { map } => {
                let mut camera = Camera {
                    pos: vec3(0.0, 0.0, 0.5),
                    facing: 0.0
                };
                self.state.sprites.clear();
                self.current_map = map.clone();
                self.state.grid = Grid::new(self.current_map.grid.size());
                self.current_map.grid.for_each(|cell, index| {
                    self.state.grid.get_mut(index).unwrap().wall = cell.wall;
                    if let Some(thing) = cell.thing {
                        let sprite = Sprite {
                            pos: Vec3::new(index.0 as f32 + 0.5, index.1 as f32 + 0.5, 0.5),
                            texture: thing,
                            opacity: None,
                            facing:cell.thing_facing,
                            radius:0.5,
                            ..Default::default()
                        };

                        
                        
                        let id = self.state.sprites.spawn(sprite);
                        let sprite = self.state.sprites.get_mut(id).unwrap();
                        match thing {
                            textures::THING_MARKER_EXIT => {
                                sprite.sprite_type = SpriteType::Floor;
                                sprite.pos.z = 0.01;
                            }
                            textures::THING_DOOR_BLUE | textures::THING_DOOR_GOLD | textures::THING_DOOR_WHITE => {
                                sprite.sprite_type = SpriteType::Wall;
                                self.state.doors.attach(id, Door::default());
                            }
                            textures::THING_MARKER_SPAWN_PLAYER => {
                                self.state.player_id = Some(id);
                                sprite.texture = textures::THING_WILLIAM;
                                camera.pos = sprite.pos;
                                camera.facing = sprite.facing;
                                sprite.radius = 0.25;
                            }
                            textures::THING_ITEM_POKEMONCARD => {
                                sprite.no_clip = true;
                                self.state.items.attach(id, Item::PokemonCard);
                            }
                            _=>{}
                        }
                    }
                });

                self.state.camera = camera;
            }
        }
    }
}