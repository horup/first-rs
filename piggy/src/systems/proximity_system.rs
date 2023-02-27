use engine_sdk::Engine;
use crate::{State};

pub fn proximity_system(state:&mut State, engine:&mut dyn Engine) {
    if let Some(player_id) = state.player_id {
        if let Some(player) = state.sprites.get(player_id) {
            let player_pos = player.pos;
            let pickup_radius = 0.5;
            let world = state.as_world();

            let mut near = Vec::new();
            world.query_around(player_pos.truncate(), pickup_radius, &mut near);
            for id in near.drain(..) {
                if let Some(_) = state.items.get(id) {
                    state.sprites.despawn(id);
                    state.flash.flash(0.2, 0.5);
                }
            }
        }
    }
    
}