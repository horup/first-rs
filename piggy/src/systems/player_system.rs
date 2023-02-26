use std::f32::consts::PI;

use engine_sdk::{Engine, VirtualKeyCode};
use crate::State;

pub fn player_system(state:&mut State, engine:&mut dyn Engine) {
    let dt = engine.dt();
    let speed = 3.0;
    let left = state.camera.left();
    let forward = state.camera.forward_body();
    let mut new_pos = state.camera.pos;
    let mut new_facing = state.camera.facing;
    if let Some(player_id) = state.player_id {
        if let Some(player_sprite) = state.sprites.get_mut(player_id) {
            new_pos = player_sprite.pos;
            new_facing = player_sprite.facing;
        }
    }

    if engine.key_down(VirtualKeyCode::A) {
        new_pos += speed * dt * left;
    }
    if engine.key_down(VirtualKeyCode::D) {
        new_pos -= speed * dt * left;
    }
    if engine.key_down(VirtualKeyCode::W) {
        new_pos += speed * dt * forward;
    }
    if engine.key_down(VirtualKeyCode::S) {
        new_pos -= speed * dt * forward;
    }

    let turn_speed = PI / 4.0;
    new_facing += turn_speed * dt * engine.mouse_motion().x;

    if let Some(player_id) = state.player_id {
        let mut world = state.as_world();
        world.clip_move(player_id, new_pos);
        match state.sprites.get_mut(player_id) {
            Some(player_sprite) => {
                player_sprite.facing = new_facing;
                state.camera.pos = player_sprite.pos;
                state.camera.facing = player_sprite.facing;
            },
            None => {
                state.camera.pos = new_pos;
                state.camera.facing = new_facing;
            },
        }
    }
}