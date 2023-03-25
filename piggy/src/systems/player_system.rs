use std::f32::consts::PI;

use engine_sdk::{Engine, VirtualKeyCode, world::World, Sprite};
use crate::{components::PlayerState, Global, PlayerEntity};

pub fn player_system(world:&mut World, engine:&mut dyn Engine) {
    let mut global = world.singleton_mut::<Global>().unwrap();
    let dt = engine.dt();
    let speed = 3.0;
    let left = global.camera.left();
    let forward = global.camera.forward_body();

    for mut e in world.query::<PlayerEntity>() {
        let old_pos = e.sprite.pos;
        let mut new_pos = e.sprite.pos;
        let mut new_facing = e.sprite.facing;
    
        if e.health.is_alive() {
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
            if let Some(killer) = e.health.killer {
                if let Some(killer) = world.component::<Sprite>(killer) {
                    let facing_towards_killer = killer.pos - e.sprite.pos;
                    let facing_towards_killer = facing_towards_killer.normalize_or_zero().truncate();
                    let facing = e.sprite.facing_as_vec2(); 

                    let angle = facing_towards_killer.angle_between(facing);
                    // turn player towards killer
                    let alpha = 10.0;
                    let facing = facing + facing_towards_killer * alpha * dt;
                    let facing = facing.normalize_or_zero();
                    new_facing = facing.y.atan2(facing.x);

                    if angle < 0.1 {
                        // is looking straight towards killer
                        // transition to cought if possible
                        e.player.state.cought();
                    }
                }
            }
        }
    
        match &mut e.player.state {
            PlayerState::Cought { timer_sec } => {
                *timer_sec -= dt;
                if *timer_sec <= 0.0 {
                    e.player.state.can_respawn();
                }
            },
            PlayerState::CanRespawn => {
                dbg!("can respawn");
            }
            _ => {}
        }
    
        e.sprite.vel = new_pos - old_pos;
        e.sprite.facing = new_facing;
        let pos = e.sprite.pos;
        let facing = e.sprite.facing;
        global.camera.pos = pos;
        global.camera.facing = facing;
    }
}