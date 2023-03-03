use std::f32::consts::PI;

use engine_sdk::{Engine, glam::{vec3}};
use crate::State;

pub fn door_system(state:&mut State, engine:&mut dyn Engine) {
    if let Some(player_id) = state.player_id {
        if let Some(player) = state.sprites.get(player_id) {
            let player_pos = player.pos;
            let _world = state.as_world();
            let mut near = Vec::new();
            let world = state.as_world();
            let radius = 1.0;
            world.query_around(player_pos.truncate(), radius, &mut near);
            for id in near.drain(..) {
                if let Some(door) = state.doors.get_mut(id) {
                    door.open();
                }
            }
        }
    }

    let dt = engine.dt();
    for (id, sprite) in state.sprites.iter_mut() {
        if let Some(door) = state.doors.get_mut(id) {
            if door.direction != 0.0 {
                door.openess += door.direction * dt;
            }

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
                sprite.no_clip = true;
                if door.close_timer > door.time_to_start_closing() {
                    door.close();
                }
            } 

            let dir = sprite.facing - PI / 2.0;
            let v = vec3(dir.cos(), dir.sin(), 0.0);
            sprite.pos = door.pos + v * door.openess;            
        }
    }
}