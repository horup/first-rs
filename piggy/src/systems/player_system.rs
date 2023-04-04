use std::f32::consts::PI;

use engine_sdk::{Engine, VirtualKeyCode, registry::{Registry, Facade}, Sprite};
use crate::{components::{PlayerState}, singletons::GameState, PlayerEntity, PiggyFacade, Signal, Start};

pub fn player_system(registry:&mut Registry, engine:&mut dyn Engine, start_signals:&mut Signal<Start>) {
    {
        let facade = registry.facade::<PiggyFacade>();
        let mut game_state = registry.singleton_mut::<GameState>().unwrap();
        let dt = engine.dt();
        let speed = 3.0;
        let left = game_state.camera.left();
        let forward = game_state.camera.forward_body();

        for mut e in facade.query::<PlayerEntity>() {
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
            }

            match e.player.state {
                PlayerState::BeingCought { .. } | PlayerState::Cought { .. }  => {
                    if let Some(killer) = e.health.killer {
                        if let Some(killer) = registry.component::<Sprite>(killer) {
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
                                //e.player.state.set_cought();
                            }
                        }
                    }
                },
                _=>{}
            }
        
            match &mut e.player.state {
                PlayerState::BeingCought { turn_around_timer } => {
                    turn_around_timer.tick(dt);
                    

                    if turn_around_timer.is_done() {
                        e.player.state.set_cought();
                    }
                }
                PlayerState::Cought { fade_out_timer } => {
                    fade_out_timer.tick(dt);
                    if fade_out_timer.alpha() > 2.0 {
                        e.player.state.set_can_respawn();
                    }
                },
                PlayerState::CanRespawn => {
                    if engine.key_just_pressed(VirtualKeyCode::Space) {
                        start_signals.push(Start {
                            override_map: None,
                            level: registry.singleton::<GameState>().unwrap().current_level,
                        });
                    }
                },
                PlayerState::Won { fade_out_timer } =>{
                    fade_out_timer.tick(dt);
                    if fade_out_timer.alpha() > 1.2 {
                        e.player.state.set_can_continue();
                    }
                }
                PlayerState::Ready { fade_in_timer } => {
                    fade_in_timer.tick(dt);
                }
                PlayerState::CanContinue {  } => {
                    if engine.key_just_pressed(VirtualKeyCode::Space) {
                        start_signals.push(Start {
                            override_map:None,
                            level: game_state.current_level + 1
                        });
                    }
                },
                
            }
        
            e.sprite.vel = new_pos - old_pos;
            e.sprite.facing = new_facing;
            let pos = e.sprite.pos;
            let facing = e.sprite.facing;
            game_state.camera.pos = pos;
            game_state.camera.facing = facing;

        }
    }

    registry.execute();
}