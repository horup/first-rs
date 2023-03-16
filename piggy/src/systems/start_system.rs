use engine_sdk::{Engine, Map, Grid, Sprite, SpriteType, glam::{Vec3}};
use crate::{State, textures, components::{Item, Door, Effector, Player, Activator, Mob, Health}};

pub fn spawn_thing(state:&mut State, thing:u32, index:(i32, i32), facing:f32) {
    let sprite = Sprite {
        pos: Vec3::new(index.0 as f32 + 0.5, index.1 as f32 + 0.5, 0.5),
        texture: thing,
        opacity: None,
        facing,
        radius:0.3,
        clips:true,
        ..Default::default()
    };
    
    let id = state.entities.spawn();
    state.sprites.attach(id, sprite);
    let sprite = state.sprites.get_mut(id).unwrap();
    match thing {
        textures::THING_MARKER_EXIT => {
            sprite.hidden = true;
            sprite.clips = false;
            sprite.radius = 0.0;
            state.effectors.attach(id, Effector::ExitMarker);
        }
        textures::THING_DOOR_BLUE | textures::THING_DOOR_GOLD | textures::THING_DOOR_WHITE => {
            sprite.sprite_type = SpriteType::Wall;
            sprite.clips = false;
            state.doors.attach(id, Door {
                pos:sprite.pos,
                ..Default::default()
            });
            state.activators.attach(id, Activator::Door {
                key: if sprite.texture == textures::THING_DOOR_BLUE { Some(textures::THING_ITEM_KEY_BLUE) } else if sprite.texture == textures::THING_DOOR_GOLD { Some(textures::THING_ITEM_KEY_GOLD) } else { None }
            });
        }
        textures::THING_MARKER_SPAWN_PLAYER => {
            state.player_id = Some(id);
            sprite.texture = textures::THING_WILLIAM;
            sprite.hidden = true;
            state.players.attach(id, Player::default());
            state.healths.attach(id, Health::default());
        }
        textures::THING_ITEM_POKEMONCARD => {
            sprite.clips = false;
            state.items.attach(id, Item::new(1.0));
        }
        textures::THING_ITEM_KEY_BLUE => {
            sprite.clips = false;
            state.items.attach(id, Item::new(1.0));
        }
        textures::THING_ITEM_KEY_GOLD => {
            sprite.clips = false;
            state.items.attach(id, Item::new(1.0));
        }
        textures::THING_MONSTER_PIGGY => {
            state.mobs.attach(id, Mob {
                is_killer:true,
                ..Default::default()
            });
        }
        _=>{}
    }
}

pub fn start_system(state:&mut State, _engine:&mut dyn Engine, map:&Map) {
    *state = State::default();
    state.grid = Grid::new(state.grid.size());
    map.grid.for_each(|cell, index| {
        if let Some(wall) = cell.wall {
            let tile = state.grid.get_mut(index).unwrap();
            tile.wall = Some(wall);
            tile.clips = true;
        }
        if let Some(thing) = cell.thing {
            spawn_thing(state, thing, index, cell.thing_facing);
        }
    });
}