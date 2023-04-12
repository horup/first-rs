use engine_sdk::{Grid, Sprite, SpriteType, glam::{Vec3}, registry::{Registry}, Tile, Tilemap, Engine};
use crate::{textures::{self, FLOOR_GREY, CEILING_GREY}, components::{Item, Door, Effector, Player, Activator, Mob, Health, StartEvent}, singletons::Global, Campaign, sounds};

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

pub fn on_start(registry:&mut Registry, campaign:&Campaign, start:&StartEvent, engine:&mut dyn Engine) {
    let current_level = start.level;
    let mut current_map = campaign.get(current_level).unwrap().map.clone();
    if let Some(map) = &start.override_map {
        current_map = map.clone();
    }
    
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
    registry.singleton_mut::<Global>().unwrap().current_level = current_level;
    engine.play_music(sounds::MUSIC01);
}