use std::f32::consts::PI;
use engine_sdk::{
    image,
    glam::{vec2, Vec3, vec3}, EditorProps, Camera, Cell, Color, DrawLineParams, LoadAtlasParams, Atlas, DrawRectParams, Engine, Entities, Grid, Map,
    Scene, Sprite, SpriteId, VirtualKeyCode, Event, SpriteType, Game, egui::Vec2,
};
use serde::{Deserialize, Serialize};
use crate::textures;

#[derive(Default, Serialize, Deserialize)]
pub struct Piggy {
    pub current_map: Map,
    pub camera: Camera,
    pub sprites: Entities<SpriteId, Sprite>,
    pub grid: Grid<Cell>,
}

impl Piggy {
    pub fn update_controls(&mut self, engine: &mut dyn Engine) {
        let dt = engine.dt();
        let speed = 3.0;
        let left = self.camera.left();
        let forward = self.camera.forward_body();
        if engine.key_down(VirtualKeyCode::A) {
            self.camera.pos += speed * dt * left;
        }
        if engine.key_down(VirtualKeyCode::D) {
            self.camera.pos -= speed * dt * left;
        }
        if engine.key_down(VirtualKeyCode::W) {
            self.camera.pos += speed * dt * forward;
        }
        if engine.key_down(VirtualKeyCode::S) {
            self.camera.pos -= speed * dt * forward;
        }

        let turn_speed = PI / 4.0;
        self.camera.facing += turn_speed * dt * engine.mouse_motion().x;
    }

    pub fn update_scene(&mut self, engine: &mut dyn Engine) {
        // self.draw_map(engine);

        // draw scene
        engine.draw_scene(
            &self.camera,
            &Scene {
                sprites: &self.sprites,
                ceiling_texture: textures::CEILING_GREY,
                floor_texture: textures::FLOOR_GREY,
                grid: &self.grid,
            },
        );
    }

    fn _draw_map(&mut self, engine: &mut dyn Engine) {
        let s = 16.0;
        for y in 0..self.grid.size() as i32 {
            for x in 0..self.grid.size() as i32 {
                let cell = self.grid.get((x, y)).unwrap();
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

        let p = vec2(self.camera.pos.x, self.camera.pos.y) * s;
        let s = vec2(s, s) / 2.0;
        engine.draw_rect(DrawRectParams {
            pos: p - s / 2.0,
            size: s,
            color: Color::WHITE,
            texture: None,
            ..Default::default()
        });

        let p2 = p + self.camera.forward_body().truncate() * s * 2.0;
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
                let mut camera = Camera {
                    pos: Vec3::default(),
                    facing: 0.0
                };
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

                        match thing {
                            textures::THING_MARKER_EXIT => {
                                sprite.sprite_type = SpriteType::Floor;
                            }
                            textures::THING_DOOR_BLUE | textures::THING_DOOR_GOLD | textures::THING_DOOR_WHITE => {
                                sprite.sprite_type = SpriteType::Wall;
                            }
                            textures::THING_MARKER_SPAWN_PLAYER => {
                                sprite.sprite_type = SpriteType::Floor;
                                camera.pos = sprite.pos;
                                camera.facing = sprite.facing;
                            }
                            _=>{}
                        }
                        
                        self.sprites.spawn(sprite);
                    }
                });

                self.camera = camera;
            }
        }
    }
}