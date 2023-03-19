use std::f32::consts::PI;

use engine_sdk::{Engine, VirtualKeyCode, glam::Vec2};
use crate::State;

pub fn player_system(state:&mut State, engine:&mut dyn Engine) {
    let dt = engine.dt();
    let speed = 3.0;
    let left = state.camera.left();
    let forward = state.camera.forward_body();
    let mut old_pos = state.camera.pos;
    let mut new_pos = state.camera.pos;
    let mut new_facing = state.camera.facing;
    if let Some(player) = state.player_entity() {
        old_pos = player.sprite.pos;
        new_pos = player.sprite.pos;
        new_facing = player.sprite.facing;
    }

    if let Some(player) = state.player_entity() {
        if player.health.is_alive() {
            if !engine.cursor_grabbed() {
                if engine.key_down(VirtualKeyCode::A) {
                    new_pos += speed * left;
                }
                if engine.key_down(VirtualKeyCode::D) {
                    new_pos -= speed * left;
                }
                if engine.key_down(VirtualKeyCode::W) {
                    new_pos += speed * forward;
                }
                if engine.key_down(VirtualKeyCode::S) {
                    new_pos -= speed * forward;
                }
                let turn_speed = PI / 4.0;
                new_facing += turn_speed * dt * engine.mouse_motion().x;

                let turn_speed = turn_speed * 5.0;
                if engine.key_down(VirtualKeyCode::Left) {
                    new_facing -= turn_speed * dt;
                } else if engine.key_down(VirtualKeyCode::Right) {
                    new_facing += turn_speed * dt;
                }
            }
        } else {
            // player is not alive, ensure player is facing the killar
            if let Some(killer) = player.health.killer {
                if let Some(killer) = state.sprites.get(killer) {
                    let facing_towards_killer = killer.pos - player.sprite.pos;
                    let facing_towards_killer = facing_towards_killer.normalize_or_zero().truncate();
                    let facing = player.sprite.facing_as_vec2(); 

                    let angle = facing_towards_killer.angle_between(facing);
                    // turn player towards killer
                    let alpha = 10.0;
                    let facing = facing + facing_towards_killer * alpha * dt;
                    let facing = facing.normalize_or_zero();
                    new_facing = facing.y.atan2(facing.x);

                    if angle < 0.1 {
                        // is looking straight towards killer
                        // start blackout countdown
                    }
                }
            }
        }
    }

    if let Some(player) = state.player_entity() {
        player.sprite.vel = new_pos - old_pos;
    }

    if let Some(player) = state.player_entity() {
        player.sprite.facing = new_facing;
        let pos = player.sprite.pos;
        let facing = player.sprite.facing;
        state.camera.pos = pos;
        state.camera.facing = facing;
    } else {
        state.camera.pos = new_pos;
        state.camera.facing = new_facing;
    }
}