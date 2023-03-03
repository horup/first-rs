use engine_sdk::{Event, Engine, Map, Camera, Grid, Sprite, SpriteType, glam::{Vec3, vec3, IVec2}, egui::Vec2};
use crate::{State, textures, components::{Item, Door, Effector, Player, KeyType}};

pub fn spawn_thing(state:&mut State, thing:u32, index:(i32, i32), facing:f32) {
    let sprite = Sprite {
        pos: Vec3::new(index.0 as f32 + 0.5, index.1 as f32 + 0.5, 0.5),
        texture: thing,
        opacity: None,
        facing:facing,
        radius:0.5,
        ..Default::default()
    };
    
    let id = state.sprites.spawn(sprite);
    let sprite = state.sprites.get_mut(id).unwrap();
    match thing {
        textures::THING_MARKER_EXIT => {
            sprite.hidden = true;
            sprite.no_clip = true;
            sprite.radius = 0.0;
            state.effectors.attach(id, Effector::ExitMarker);
        }
        textures::THING_DOOR_BLUE | textures::THING_DOOR_GOLD | textures::THING_DOOR_WHITE => {
            sprite.sprite_type = SpriteType::Wall;
            state.doors.attach(id, Door {
                pos:sprite.pos,
                ..Default::default()
            });
        }
        textures::THING_MARKER_SPAWN_PLAYER => {
            state.player_id = Some(id);
            sprite.texture = textures::THING_WILLIAM;
            sprite.radius = 0.25;
            state.players.attach(id, Player::default());
        }
        textures::THING_ITEM_POKEMONCARD => {
            sprite.no_clip = true;
            state.items.attach(id, Item::PokemonCard);
        }
        textures::THING_ITEM_KEY_BLUE => {
            sprite.no_clip = true;
            state.items.attach(id, Item::Key {
                key_type:KeyType::Blue
            });
        }
        textures::THING_ITEM_KEY_GOLD => {
            sprite.no_clip = true;
            state.items.attach(id, Item::Key {
                key_type:KeyType::Gold
            });
        }
        _=>{}
    }
}

pub fn start_system(state:&mut State, engine:&mut dyn Engine, map:&Map) {
    *state = State::default();
    state.grid = Grid::new(state.grid.size());
    map.grid.for_each(|cell, index| {
        state.grid.get_mut(index).unwrap().wall = cell.wall;
        if let Some(thing) = cell.thing {
            spawn_thing(state, thing, index, cell.thing_facing);
        }
    });
}