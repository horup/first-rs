use std::f32::consts::PI;

use engine_sdk::{Engine, glam::{vec3}, world::World, Grid, Tile, Tilemap};

use crate::DoorEntity;

pub fn door_system(world:&mut World, engine:&mut dyn Engine) {
    let mut tilemap = &mut world.singleton_mut::<Tilemap>().unwrap().grid;
    // update doors 
    let dt = engine.dt();
    for mut e in world.query::<DoorEntity>() {
        let speed = 2.0;
        e.door.openess += speed * e.door.direction * dt;
        if e.door.openess < 0.0 {
            e.door.openess = 0.0;
            e.door.direction = 0.0;
        } else if e.door.openess > 1.0 {
            e.door.openess = 1.0;
            e.door.direction = 0.0;
            e.door.close_timer = 0.0;
        }

        if e.door.is_open() {
            e.door.close_timer += dt;
            e.sprite.clips = true;
            if e.door.close_timer > e.door.time_to_start_closing() {
                e.door.close();
            }
        } 

        let dir = e.sprite.facing - PI / 2.0;
        let v = vec3(dir.cos(), dir.sin(), 0.0);
        e.sprite.pos = e.door.pos + v * e.door.openess;     

        if let Some(tile) = tilemap.get_mut(e.door.pos.as_ivec3().truncate().into()) {
            if e.door.openess > 0.5 {
                tile.clips = false;
            } else {
                tile.clips = true;
            }   
        }
    }
}