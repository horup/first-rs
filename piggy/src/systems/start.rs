
use crate::{
    components::{
        Activator, Door, Effector, Event, Health, Item, Mob, Player,
        PlayerCompletedFinalLevelEvent, StartEvent,
    },
    singletons::{Campaign, Global},
    sounds,
    textures::{self, CEILING_GREY, FLOOR_GREY}, atlases,
};
use engine_sdk::{
    glam::Vec3, registry::Registry, Engine, Grid, SoundEmitter, Sprite, SpriteType, Tile, Tilemap, MapEntity, egui::epaint::ahash::HashMap, Pic,
};

fn spawn_player(r: &mut Registry, map_entity:&MapEntity, index: (i32, i32)) {
    let mut e = r.spawn();
    let mut sprite = Sprite {
        pos: Vec3::new(index.0 as f32 + 0.5, index.1 as f32 + 0.5, 0.5),
        pic:Pic::new(0, 0),
        opacity: None,
        facing:map_entity.facing,
        radius: 0.3,
        clips: true,
        ..Default::default()
    };
    sprite.hidden = true;
    e.attach(sprite);
    e.attach(Player::default());
    e.attach(Health::default());
}

fn spawn_exit(r: &mut Registry, _map_entity:&MapEntity, index: (i32, i32)) {
    let mut e = r.spawn();
    e.attach(Sprite {
        pos: Vec3::new(index.0 as f32 + 0.5, index.1 as f32 + 0.5, 0.5),
        clips: false,
        hidden: true,
        ..Default::default()
    });
    e.attach(Effector::ExitMarker);
}

fn spawn_item(r: &mut Registry, map_entity:&MapEntity, index: (i32, i32)) {
    let mut e = r.spawn();
    e.attach(Sprite {
        pos: Vec3::new(index.0 as f32 + 0.5, index.1 as f32 + 0.5, 0.5),
        clips: false,
        pic:map_entity.pic,
        ..Default::default()
    });
    e.attach(Item::new(1.0));
}

fn spawn_door(r: &mut Registry, map_entity:&MapEntity, index: (i32, i32)) {
    let mut e = r.spawn();
    let pos = Vec3::new(index.0 as f32 + 0.5, index.1 as f32 + 0.5, 0.5);
    e.attach(Sprite {
        pos: Vec3::new(index.0 as f32 + 0.5, index.1 as f32 + 0.5, 0.5),
        clips: false,
        facing:map_entity.facing,
        pic:map_entity.pic,
        sprite_type:SpriteType::Wall,
        ..Default::default()
    });
    e.attach(Door {
        pos: pos,
        ..Default::default()
    });

    e.attach(Activator::Door {
        key: match map_entity.pic.index {
            1 => Some(Pic::new(atlases::ITEMS, 1)),
            2 => Some(Pic::new(atlases::ITEMS, 2)),
            _=> None
        },
    });
}

fn spawn_mob(r: &mut Registry, map_entity:&MapEntity, index: (i32, i32)) {
    //todo!();
}

pub fn spawn_entity(r: &mut Registry, map_entity:&MapEntity, index: (i32, i32)) {
    let class = map_entity.class.as_str();
    match class {
        "spawn_player" => spawn_player(r, map_entity, index),
        "exit" => spawn_exit(r, map_entity, index),
        "item" => spawn_item(r, map_entity, index),
        "door" => spawn_door(r, map_entity, index),
        "mob"  => spawn_mob(r, map_entity, index),
        _ => {}
    }


   /* let mut sprite = Sprite {
        pos: Vec3::new(index.0 as f32 + 0.5, index.1 as f32 + 0.5, 0.5),
        texture: map_entity.pic.atlas,
        opacity: None,
        facing:map_entity.facing,
        radius: 0.3,
        clips: true,
        ..Default::default()
    };*/

   /* match thing {
        textures::THING_MARKER_EXIT => {
            sprite.hidden = true;
            sprite.clips = false;
            sprite.radius = 0.0;
            e.attach(Effector::ExitMarker);
        }
        textures::THING_DOOR_BLUE | textures::THING_DOOR_GOLD | textures::THING_DOOR_WHITE => {
            sprite.sprite_type = SpriteType::Wall;
            sprite.clips = false;
            e.attach(Door {
                pos: sprite.pos,
                ..Default::default()
            });
            e.attach(Activator::Door {
                key: if sprite.texture == textures::THING_DOOR_BLUE {
                    Some(textures::THING_ITEM_KEY_BLUE)
                } else if sprite.texture == textures::THING_DOOR_GOLD {
                    Some(textures::THING_ITEM_KEY_GOLD)
                } else {
                    None
                },
            });
        }
        textures::THING_MARKER_SPAWN_PLAYER => {
            //registry.player_id = Some(id);
            sprite.texture = textures::THING_WILLIAM;
            sprite.hidden = true;
            e.attach(Player::default());
            e.attach(Health::default());
        }
        textures::THING_ITEM_POKEMONCARD => {
            sprite.clips = false;
            e.attach(Item::new(1.0));
        }
        textures::THING_ITEM_KEY_BLUE => {
            sprite.clips = false;
            e.attach(Item::new(1.0).with_pickup_sound(sounds::PICKUP_KEY));
        }
        textures::THING_ITEM_KEY_GOLD => {
            sprite.clips = false;
            e.attach(Item::new(1.0).with_pickup_sound(sounds::PICKUP_KEY));
        }
        textures::THING_MONSTER_PIGGY => {
            e.attach(Mob {
                is_killer: true,
                ..Default::default()
            });
        }
        _ => {}
    }*/
}

pub fn start(r: &mut Registry, start: &StartEvent, engine: &mut dyn Engine) {
    let level = start.level;
    let map_to_load = match &start.override_map {
        Some(map) => Option::Some(map.clone()),
        None => match r.singleton::<Campaign>().unwrap().get(level) {
            Some(level) => Some(level.map.clone()),
            None => Option::default(),
        },
    };

    if let Some(map_to_load) = map_to_load {
        r.clear();
        let mut grid: Grid<Tile> = Grid::new(map_to_load.grid.size());
        map_to_load.grid.for_each(|cell, index| {
            if let Some(wall) = cell.wall {
                let tile = grid.get_mut(index).unwrap();
                tile.wall = Some(wall.pic);
                tile.clips = true;
            }
            if let Some(e) = &cell.entity {
                spawn_entity(r, e, index);
                //spawn_thing(r, thing, index, cell.thing_facing);
            }
        });
        {
            let mut tilemap = r.singleton_mut::<Tilemap>().unwrap();
            tilemap.grid = grid;
            tilemap.floor_texture = FLOOR_GREY;
            tilemap.ceiling_texture = CEILING_GREY;
            r.singleton_mut::<Global>().unwrap().current_level = level;
        }
        r.spawn().attach(SoundEmitter {
            sound: sounds::MUSIC01,
            position_secs: 0.0,
            loops: true,
        });
    } else {
        // show finish game screen
        r.spawn().attach(Event::PlayerCompletedFinalLevel(
            PlayerCompletedFinalLevelEvent {},
        ));
    }
}
