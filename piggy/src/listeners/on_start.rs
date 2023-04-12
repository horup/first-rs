use engine_sdk::{Grid, Sprite, SpriteType, glam::{Vec3}, registry::{Registry}, Tile, Tilemap, Engine};
use crate::{textures::{self, FLOOR_GREY, CEILING_GREY}, components::{Item, Door, Effector, Player, Activator, Mob, Health, StartEvent, Event, PlayerCompletedFinalLevelEvent}, singletons::{Global, Campaign}, sounds};

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
            e.attach(Item::new(1.0).with_pickup_sound(sounds::PICKUP_KEY));
        }
        textures::THING_ITEM_KEY_GOLD => {
            sprite.clips = false;
            e.attach(Item::new(1.0).with_pickup_sound(sounds::PICKUP_KEY));
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

pub fn on_start(r:&mut Registry,start:&StartEvent, engine:&mut dyn Engine) {
    let level = start.level;
    let map_to_load = match &start.override_map {
        Some(map) => Option::Some(map.clone()),
        None => {
            match r.singleton::<Campaign>().unwrap().get(level) {
                Some(level) => Some(level.map.clone()),
                None => Option::default(),
            }
        },
    };

    if let Some(map_to_load) = map_to_load {
        r.clear();
        // start new map
        let mut grid:Grid<Tile> = Grid::new(map_to_load.grid.size());
        map_to_load.grid.for_each(|cell, index| {
            if let Some(wall) = cell.wall {
                let tile = grid.get_mut(index).unwrap();
                tile.wall = Some(wall);
                tile.clips = true;
            }
            if let Some(thing) = cell.thing {
                spawn_thing(r, thing, index, cell.thing_facing);
            }
        });

        let mut tilemap = r.singleton_mut::<Tilemap>().unwrap();
        tilemap.grid = grid;
        tilemap.floor_texture = FLOOR_GREY;
        tilemap.ceiling_texture = CEILING_GREY;
        r.singleton_mut::<Global>().unwrap().current_level = level;
        engine.play_music(sounds::MUSIC01);
    } else {
        // show finish game screen
        r.spawn().attach(Event::PlayerCompletedFinalLevel(PlayerCompletedFinalLevelEvent {}));

    }
}