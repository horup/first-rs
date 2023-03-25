use engine_sdk::{Engine, Map, Grid, Sprite, SpriteType, glam::{Vec3}, world::World, Tile};
use crate::{textures, components::{Item, Door, Effector, Player, Activator, Mob, Health}};

pub fn spawn_thing(world:&mut World, thing:u32, index:(i32, i32), facing:f32) {
    let mut sprite = Sprite {
        pos: Vec3::new(index.0 as f32 + 0.5, index.1 as f32 + 0.5, 0.5),
        texture: thing,
        opacity: None,
        facing,
        radius:0.3,
        clips:true,
        ..Default::default()
    };
    
    let mut e = world.spawn();
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
            //world.player_id = Some(id);
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

pub fn start_system(world:&mut World, _engine:&mut dyn Engine, map:&Map) {
    world.clear();
    let mut grid:Grid<Tile> = Grid::new(map.grid.size());
    map.grid.for_each(|cell, index| {
        if let Some(wall) = cell.wall {
            let tile = grid.get_mut(index).unwrap();
            tile.wall = Some(wall);
            tile.clips = true;
        }
        if let Some(thing) = cell.thing {
            spawn_thing(world, thing, index, cell.thing_facing);
        }
    });

    *world.singleton_mut::<Grid<Tile>>().unwrap() = grid;
}