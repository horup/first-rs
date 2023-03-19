use std::f32::consts::PI;

use engine_sdk::{Engine, glam::{vec3}};
use crate::State;

pub fn door_system(state:&mut State, engine:&mut dyn Engine) {
    // update doors 
    let dt = engine.dt();
    for id in state.entities.iter() {
        if let (Some(door), Some(sprite)) = (state.doors.get_mut2(id), state.sprites.get_mut2(id)) {
            let speed = 2.0;
            door.openess += speed * door.direction * dt;
            if door.openess < 0.0 {
                door.openess = 0.0;
                door.direction = 0.0;
            } else if door.openess > 1.0 {
                door.openess = 1.0;
                door.direction = 0.0;
                door.close_timer = 0.0;
            }

            if door.is_open() {
                door.close_timer += dt;
                sprite.clips = true;
                if door.close_timer > door.time_to_start_closing() {
                    door.close();
                }
            } 

            let dir = sprite.facing - PI / 2.0;
            let v = vec3(dir.cos(), dir.sin(), 0.0);
            sprite.pos = door.pos + v * door.openess;     

            if let Some(tile) = state.grid.get_mut(door.pos.as_ivec3().truncate().into()) {
                if door.openess > 0.5 {
                    tile.clips = false;
                } else {
                    tile.clips = true;
                }   
            }
        }
    }
}