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
        } else {
            // player is not alive, ensure player is facing the killar
            if let Some(killer) = player.health.killer {
                if let Some(killer) = state.sprites.get(killer) {
                    let v = killer.pos - player.sprite.pos;
                    let v = v.normalize_or_zero().truncate();
                    let facing = v.y.atan2(v.x);
                    new_facing = facing;
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