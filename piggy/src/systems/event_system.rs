use engine_sdk::{Engine, Grid, Sprite, SpriteType, glam::{Vec3}, registry::{Registry, Commands}, Tile, Tilemap};
use crate::{textures::{self, FLOOR_GREY, CEILING_GREY}, components::{Item, Door, Effector, Player, Activator, Mob, Health, Event, RespawnEvent}, singletons::GameState};

pub fn spawn_thing(registry:&mut Registry, thing:u32, index:(i32, i32), facing:f32) {
    let mut sprite = Sprite {
        pos: Vec3::new(index.0 as f32 + 0.5, index.1 as f32 + 0.5, 0.5),
        texture: thing,
        opacity: None,
        facing,
        radius:0.3,
        clips:true,
        ..Default::default()
    };
    
    let mut e = registry.spawn();
    match thing {
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
                pos:sprite.pos,
                ..Default::default()
            });
            e.attach(Activator::Door {
                key: if sprite.texture == textures::THING_DOOR_BLUE { Some(textures::THING_ITEM_KEY_BLUE) } else if sprite.texture == textures::THING_DOOR_GOLD { Some(textures::THING_ITEM_KEY_GOLD) } else { None }
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
            e.attach(Item::new(1.0));
        }
        textures::THING_ITEM_KEY_GOLD => {
            sprite.clips = false;
            e.attach(Item::new(1.0));
        }
        textures::THING_MONSTER_PIGGY => {
            e.attach(Mob {
                is_killer:true,
                ..Default::default()
            });
        }
        _=>{}
    }

    e.attach(sprite);

}

pub fn on_respawn_event(registry:&mut Registry, _engine:&mut dyn Engine, _e:RespawnEvent) {
    let current_map = registry.singleton::<GameState>().unwrap().current_map.clone();

    registry.clear();
    let mut grid:Grid<Tile> = Grid::new(current_map.grid.size());
    current_map.grid.for_each(|cell, index| {
        if let Some(wall) = cell.wall {
            let tile = grid.get_mut(index).unwrap();
            tile.wall = Some(wall);
            tile.clips = true;
        }
        if let Some(thing) = cell.thing {
            spawn_thing(registry, thing, index, cell.thing_facing);
        }
    });

    let mut tilemap = registry.singleton_mut::<Tilemap>().unwrap();
    tilemap.grid = grid;
    tilemap.floor_texture = FLOOR_GREY;
    tilemap.ceiling_texture = CEILING_GREY;
    registry.singleton_mut::<GameState>().unwrap().current_map = current_map;
}

pub fn event_system(registry:&mut Registry, engine:&mut dyn Engine) {
    let mut events = Vec::with_capacity(32);
    for (id, _) in registry.components::<Event>().iter() {
        events.push(id);
    }   

    for id in events {
        if let Some(event) = registry.component_detach::<Event>(id) {
            match event {
                Event::Respawn(e) => {
                    on_respawn_event(registry, engine, e);
                },
                Event::PlayerWon { player_id } => {
                    //registry.despawn(player_id);
                },
            }
        }
        registry.despawn(id);
    }
    
}